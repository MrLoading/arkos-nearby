# ArkOS Nearby 0.7.0

- 打开工具即可发现附近的游戏房间和组网房间；上下切换时，列表与操作区域会展开或缩成微缩预览。
- 加入房间时匹配本机游戏，提供对应版本供选择；从游戏启动入口进入时直接使用当前游戏。
- 游戏库显示平台图标、游戏数量、名称和截图。
- 主页按 **FN + X** 打开联机工具，单独按 **X** 保留随机选择。
- 修复发现大厅启动黑屏和主页快捷键偶发无法打开的问题。

![双机联机演示](https://raw.githubusercontent.com/W-Mai/arkos-nearby/v0.7.0/docs/assets/nearby-dual-demo.svg)

## 安装与更新

下载 **arkos-nearby-0.7.0-arm64.tar.gz**，解压后将 `arkos-nearby` 文件夹放入游戏卡的 `ports` 目录，在掌机 **Ports → Install Nearby Multiplayer** 中安装。已有版本使用相同方式更新，两台掌机安装同一版本，安装完成后重启掌机。

适用配置：**ArkOS4Clone 08262026 · Linux ARM64 4.4.189 · RTL8188EU · 640×480 屏幕**。

从 **Options → Nearby Multiplayer** 或主页 **FN + X** 打开附近房间列表，向下移动可展开「仅组网／选择游戏」。游戏加载前按住 **X** 可直接为当前游戏创建或加入房间。一台创建房间，另一台选中房间加入，双方准备完成后由房主开始游戏。Options 中的 **仅组网** 可用于 Ports 局域网游戏。

[安装与更新](https://github.com/W-Mai/arkos-nearby/blob/v0.7.0/docs/installation.md) · [游戏兼容性](https://github.com/W-Mai/arkos-nearby/blob/v0.7.0/docs/compatibility.md)
