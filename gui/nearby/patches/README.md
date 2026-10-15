# SDL2 GLES configuration

`mirui-0.49.0-gles.patch` adds fallible SDL GPU initialization, GLES 2.0 context selection, fullscreen and multisample settings, and an opt-in capture before presentation. Presentation runs in `end_flush`, once per frame. The SDL surface retains its transient backbuffer declaration.

`scripts/build_nearby_gpu.py` applies the patch to a checksum-verified registry archive under `/tmp/arkos-rust-build/public/gpu/`. The application lockfile retains its registry identity; the GPU staging lockfile identifies the patched source. The build report records both the original crate checksum and patch checksum.

The default framebuffer build uses the registry package directly. GPU initialization requires SDL2 with RenderGeometry support, KMSDRM, GLES2, and the handheld's EGL/GBM driver libraries.
