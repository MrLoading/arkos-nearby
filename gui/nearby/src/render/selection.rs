use super::state::{DisplayState, PageKey};
use mirui::prelude::*;
use mirui::ui::widgets::FollowBounds;
use mirui::ui::{Hidden, Parent};

#[component]
pub(super) struct FocusTarget {
    pub index: usize,
}

#[component]
pub(super) struct SelectionFrame {
    pub body: Entity,
    pub page: PageKey,
    pub state: model!(DisplayState),
}

pub(super) fn contains(
    root: Entity,
    mut node: Entity,
    parent: impl Fn(Entity) -> Option<Entity>,
) -> bool {
    loop {
        if node == root {
            return true;
        }
        let Some(next) = parent(node) else {
            return false;
        };
        node = next;
    }
}

#[event]
fn initialize(
    node: entity!(),
    frames: query!(SelectionFrame),
    targets: query!(Entity, FocusTarget),
    parents: query!(Parent),
    commands: commands!(),
) {
    let Some(frame) = frames.get(node) else {
        return;
    };
    let selected = frame.state.selected();
    for (entity, target) in targets {
        if selected == Some(target.index)
            && contains(frame.body, entity, |e| parents.get(e).map(|p| p.0))
        {
            commands.insert(
                node,
                FollowBounds::new(entity).with_transition(mirui::anim::SNAPPY),
            );
            return;
        }
    }
    commands.insert(node, Hidden);
}

#[view(component = SelectionFrame, attach = initialize::callback())]
pub(super) fn frame() {}

#[event]
fn initialize_target(
    node: entity!(),
    targets: query!(FocusTarget),
    frames: query!(Entity, SelectionFrame),
    followers: query!(FollowBounds),
    parents: query!(Parent),
    commands: commands!(),
) {
    let Some(target) = targets.get(node) else {
        return;
    };
    for (entity, frame) in frames {
        if frame.state.selected() == Some(target.index)
            && contains(frame.body, node, |e| parents.get(e).map(|p| p.0))
        {
            if let Some(bounds) = followers.get(entity) {
                if bounds.target != node {
                    let mut next = *bounds;
                    next.target = node;
                    commands.insert(entity, next);
                }
            } else {
                commands.insert(
                    entity,
                    FollowBounds::new(node).with_transition(mirui::anim::SNAPPY),
                );
            }
            commands.remove::<Hidden>(entity);
        }
    }
}

#[view(component = FocusTarget, attach = initialize_target::callback())]
pub(super) fn target() {}

#[system(order = Animation)]
pub(super) fn update(
    state: model!(DisplayState),
    frames: query!(Entity, SelectionFrame, optional Hidden),
    followers: query!(FollowBounds),
    targets: query!(Entity, FocusTarget),
    parents: query!(Parent),
    commands: commands!(),
) {
    let page = state.page_key();
    let selected = state.selected();
    let Some((entity, frame, hidden)) = frames.into_iter().find(|(_, frame, _)| frame.page == page)
    else {
        return;
    };
    let mut target = None;
    for (node, marker) in targets {
        if selected == Some(marker.index)
            && contains(frame.body, node, |e| parents.get(e).map(|p| p.0))
        {
            target = Some(node);
            break;
        }
    }
    if let Some(target) = target {
        if let Some(bounds) = followers.get(entity) {
            if bounds.target != target {
                let mut next = *bounds;
                next.target = target;
                commands.insert(entity, next);
            }
        } else {
            commands.insert(
                entity,
                FollowBounds::new(target).with_transition(mirui::anim::SNAPPY),
            );
        }
        if hidden.is_some() {
            commands.remove::<Hidden>(entity);
        }
    } else if hidden.is_none() {
        commands.insert(entity, Hidden);
    }
}
