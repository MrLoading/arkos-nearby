# ArkOS Nearby 0.8.1

- 修复直接加入游戏房间时，本机匹配游戏列表无法显示的问题。
- 加快游戏库再次打开、目录浏览和游戏搜索，保留后台更新时的列表内容。
- 加快加入房间时查找本机游戏的过程，并根据目录变化更新游戏列表。
- 已查看的游戏封面立即显示，并提前加载相邻游戏的图片。

![双机联机演示](https://raw.githubusercontent.com/W-Mai/arkos-nearby/v0.8.1/docs/assets/nearby-dual-demo.svg)

## 安装与更新

下载 **arkos-nearby-0.8.1-arm64.tar.gz**，解压后将 `arkos-nearby` 文件夹放入游戏卡的 `ports` 目录，在掌机 **Ports → Install Nearby Multiplayer** 中安装。已有版本使用相同方式更新，两台掌机安装同一版本，安装完成后重启掌机。

适用配置：**ArkOS4Clone 08262026 · Linux ARM64 4.4.189 · RTL8188EU · 640×480 屏幕**。

从 **Options → Nearby Multiplayer** 或主页 **FN + X** 打开附近房间列表，向下移动可展开「仅组网／选择游戏」。游戏加载前按住 **X** 可直接为当前游戏创建或加入房间。一台创建房间，另一台选中房间加入，双方准备完成后由房主开始游戏。Options 中的 **仅组网** 可用于 Ports 局域网游戏。

[安装与更新](https://github.com/W-Mai/arkos-nearby/blob/v0.8.1/docs/installation.md) · [游戏兼容性](https://github.com/W-Mai/arkos-nearby/blob/v0.8.1/docs/compatibility.md)
