# ArkOS Nearby 0.6.0

- 增加 8-bit 菜单音效，移动选中项、确认和返回时提供声音反馈。
- 发现房间、收到加入请求、准备完成、开始游戏及连接结果都有对应提示音。
- 支持设置菜单音量与静音，音量配置见[音效设置](https://github.com/W-Mai/arkos-nearby/blob/v0.6.0/docs/troubleshooting.md)。
- 更新双机联机演示，优化 SVG 动画播放并提供 MP4 导出。

![双机联机演示](https://raw.githubusercontent.com/W-Mai/arkos-nearby/v0.6.0/docs/assets/nearby-dual-demo.svg)

## 安装与更新

下载 **arkos-nearby-0.6.0-arm64.tar.gz**，解压后将 `arkos-nearby` 文件夹放入游戏卡的 `ports` 目录，在掌机 **Ports → Install Nearby Multiplayer** 中安装。已有版本使用相同方式更新，两台掌机安装同一版本。

适用配置：**ArkOS4Clone 08262026 · Linux ARM64 4.4.189 · RTL8188EU · 640×480 屏幕**。

游戏加载前按住 **X**，或从 **Options → Nearby Multiplayer** 选择游戏。一台创建房间，另一台选中房间加入，双方准备完成后由房主开始游戏。Options 中的 **仅组网** 可用于 Ports 局域网游戏。

[安装与更新](https://github.com/W-Mai/arkos-nearby/blob/v0.6.0/docs/installation.md) · [游戏兼容性](https://github.com/W-Mai/arkos-nearby/blob/v0.6.0/docs/compatibility.md)
