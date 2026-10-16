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
