use super::state::{DisplayState, PageKey};
use mirui::prelude::*;
use mirui::ui::widgets::FollowBounds;
use mirui::ui::{Hidden, Parent};

#[component]
pub(super) enum FocusTarget {
    Fixed(usize),
    Row(Signal<usize>),
}

impl FocusTarget {
    pub fn index(&self) -> usize {
        match self {
            Self::Fixed(index) => *index,
            Self::Row(index) => index.get_untracked(),
        }
    }
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

fn resolve<'a>(
    frame: &SelectionFrame,
    targets: impl IntoIterator<Item = (Entity, &'a FocusTarget)>,
    parent: impl Fn(Entity) -> Option<Entity>,
) -> Option<Entity> {
    let selected = frame.state.selected();
    targets.into_iter().find_map(|(node, target)| {
        (selected == Some(target.index()) && contains(frame.body, node, &parent)).then_some(node)
    })
}

enum FocusChange {
    Bounds(FollowBounds),
    Show,
    Hide,
}

fn follow(
    node: Option<Entity>,
    current: Option<&FollowBounds>,
    hidden: bool,
    mut apply: impl FnMut(FocusChange),
) {
    if let Some(target) = node {
        if current.is_none_or(|bounds| bounds.target != target) {
            let mut next = current
                .copied()
                .unwrap_or_else(|| FollowBounds::new(target).with_transition(mirui::anim::SNAPPY));
            next.target = target;
            apply(FocusChange::Bounds(next));
        }
        if hidden {
            apply(FocusChange::Show);
        }
    } else if !hidden {
        apply(FocusChange::Hide);
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
    let target = resolve(frame, targets, |e| parents.get(e).map(|p| p.0));
    follow(target, None, false, |change| match change {
        FocusChange::Bounds(bounds) => commands.insert(node, bounds),
        FocusChange::Show => commands.remove::<Hidden>(node),
        FocusChange::Hide => commands.insert(node, Hidden),
    });
}

#[view(component = SelectionFrame, attach = initialize::callback())]
pub(super) fn frame() {}

#[event]
fn initialize_target(
    node: entity!(),
    targets: query!(FocusTarget),
    frames: query!(Entity, SelectionFrame, optional Hidden),
    followers: query!(FollowBounds),
    parents: query!(Parent),
    commands: commands!(),
) {
    let Some(target) = targets.get(node) else {
        return;
    };
    for (entity, frame, hidden) in frames {
        if frame.state.selected() == Some(target.index())
            && contains(frame.body, node, |e| parents.get(e).map(|p| p.0))
        {
            follow(
                Some(node),
                followers.get(entity),
                hidden.is_some(),
                |change| match change {
                    FocusChange::Bounds(bounds) => commands.insert(entity, bounds),
                    FocusChange::Show => commands.remove::<Hidden>(entity),
                    FocusChange::Hide => commands.insert(entity, Hidden),
                },
            );
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
    let Some((entity, frame, hidden)) = frames.into_iter().find(|(_, frame, _)| frame.page == page)
    else {
        return;
    };
    let target = resolve(frame, targets, |e| parents.get(e).map(|p| p.0));
    follow(
        target,
        followers.get(entity),
        hidden.is_some(),
        |change| match change {
            FocusChange::Bounds(bounds) => commands.insert(entity, bounds),
            FocusChange::Show => commands.remove::<Hidden>(entity),
            FocusChange::Hide => commands.insert(entity, Hidden),
        },
    );
}
