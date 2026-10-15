use mirui::prelude::*;

#[component]
pub(super) struct GpuTextCache;

#[event]
fn attach(node: entity!(), commands: commands!()) {
    if cfg!(feature = "gpu") {
        commands.insert(node, mirui::ui::OffscreenRender::default());
    }
}

#[view(component = GpuTextCache, attach = attach::callback())]
pub(super) fn text_cache() {}
