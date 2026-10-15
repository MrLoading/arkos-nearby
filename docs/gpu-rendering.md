# GPU 界面构建

GPU 候选版通过 SDL2 的 GLES2 渲染器显示菜单，使用现有的显示协议、页面和手柄输入。软件渲染和 GPU 渲染共享场景代码，通过 `Surface` 与 `RendererFactory` 选择输出。默认安装包使用 framebuffer；GPU 产物由独立命令生成。

## 构建环境

需要 Rust 1.92、`aarch64-unknown-linux-gnu` 标准库、ARM64 GNU C 链接器及 sysroot，以及启用 KMSDRM、GLES2、RenderGeometry 的 ARM64 SDL2 动态库。SDL2 最低版本为 2.0.18；本地候选使用 2.32.10。sysroot 包含 glibc 启动对象和链接库，版本应与目标设备兼容。EGL、GBM 和 DRM 驱动在设备上加载。

```sh
python3 scripts/build_nearby_gpu.py check --sysroot /path/to/sysroot --sdl-library /path/to/libSDL2-2.0.so.0 --linker /path/to/aarch64-linux-gnu-clang
python3 scripts/build_nearby_gpu.py build --sysroot /path/to/sysroot --sdl-library /path/to/libSDL2-2.0.so.0 --linker /path/to/aarch64-linux-gnu-clang
```

SDL2 库所在目录需提供 `libSDL2.so` 链接名。构建使用锁文件固定的 mirui 0.50.0 正式包和已缓存依赖，编译缓存位于 `/tmp/arkos-rust-build/`。输出位于 `dist/gpu/`，包含界面程序、私有 SDL2 库、许可证和构建记录。记录包含源码、锁文件、mirui 包、程序和 SDL2 库的 SHA-256。

## 实机检查

在掌机图形前端释放显示设备后，执行以下命令。`LD_LIBRARY_PATH` 指向候选目录自己的 `lib/`，`SDL_VIDEODRIVER=kmsdrm` 选择直接显示后端。

```sh
LD_LIBRARY_PATH=/path/to/gpu/lib SDL_VIDEODRIVER=kmsdrm SDL_INPUT_LINUX_KEEP_KBD=1 /path/to/gpu/arkos-nearby-gui --gpu-probe /tmp/nearby-gpu.png
```

检查程序要求 SDL 渲染器为 `opengles2`，绘制模式选择页，并在交换缓冲区前保存 PNG。标准输出包含渲染器、首帧耗时和截图字节数。检查结束后恢复前端；正式菜单运行日志包含渲染后端与帧耗时统计。

菜单启动器设置 `SDL_INPUT_LINUX_KEEP_KBD=1`，让手柄映射生成的按键继续进入 TTY 输入通道，并设置 `SDL_RENDER_BATCHING=1`，为显式选择的 GLES2 渲染器启用批处理。SDL 保留事件轮询，页面操作仍通过既有显示协议发送给房间管理器。

GPU 页面通过 `GpuTextCache` 缓存步骤、标题、状态和底部提示的文字区域，文本或布局变化时由场景失效机制更新。活动指示和焦点动画独立绘制；原有静态面板缓存继续复用。软件渲染保持原有缓存策略。主循环每轮最多提交一帧，收到新状态后先处理输入，再进入下一轮动画绘制。

部署前需完成设备上的显示、按键和退出检查，并通过安装记录管理界面程序及私有库。候选构建成功表示编译与链接完成，实机表现以设备记录为准。
