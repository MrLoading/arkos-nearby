use mirui::prelude::*;

#[component]
pub(super) struct GpuLayerCache;

#[event]
fn attach(node: entity!(), commands: commands!()) {
    if cfg!(feature = "gpu") {
        commands.insert(node, mirui::ui::OffscreenRender::default());
    }
}

#[view(component = GpuLayerCache, attach = attach::callback())]
pub(super) fn gpu_layer() {}

#[component]
pub(super) struct SettledLayerCache {
    pub state: model!(super::state::DisplayState),
    pub backdrop: bool,
}

#[system(order = Animation)]
pub(super) fn settled_layer(
    runtime: res!(super::state::DisplayRuntime),
    outputs: res!(mirui::ui::Outputs<mirui::ui::WidgetTexture>),
    nodes: query!(Entity, SettledLayerCache, optional mirui::ui::OffscreenRender),
    commands: commands!(),
) {
    let gpu = cfg!(any(feature = "gpu", test));
    let viewport_settled = runtime.deck.viewport_settled();
    for (entity, layer, output) in nodes {
        // Content and focus animate independently. A completed content output
        // can survive its parent's transform; new captures wait for it to settle.
        let reusable = output.is_some() && outputs.get(entity).is_some();
        // The retained shelf contains card motion; loading and actor timelines
        // belong to separate layers and cannot keep its pixels uncached.
        let content_settled = if layer.backdrop {
            layer.state.card_motion().cards_settled()
        } else {
            layer.state.motion().settled()
        };
        let cached = gpu && (!layer.backdrop || viewport_settled || reusable) && content_settled;
        if cached && output.is_none() {
            commands.insert(entity, mirui::ui::OffscreenRender::default());
        } else if !cached && output.is_some() {
            commands.remove::<mirui::ui::OffscreenRender>(entity);
        }
    }
}
