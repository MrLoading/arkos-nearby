# ArkOS Nearby 0.8.0

- 新增收藏夹：游戏列表按 **X** 收藏，横向浏览卡带，按 **Y** 切换名称或最近使用排序。
- 平台列表按 **Y** 打开搜索，支持游戏名、文件名、中文名拼音及首字母；按 **START** 切换键盘和搜索结果。
- 为各游戏平台增加主机示意图，扩大游戏截图，调整标题排版。
- 加快游戏库读取，后台更新时保留游戏数量，改善主页上下展开的流畅度。
- 修复返回上一级时闪出空游戏列表的问题。

![双机联机演示](https://raw.githubusercontent.com/W-Mai/arkos-nearby/v0.8.0/docs/assets/nearby-dual-demo.svg)

## 安装与更新

下载 **arkos-nearby-0.8.0-arm64.tar.gz**，解压后将 `arkos-nearby` 文件夹放入游戏卡的 `ports` 目录，在掌机 **Ports → Install Nearby Multiplayer** 中安装。已有版本使用相同方式更新，两台掌机安装同一版本，安装完成后重启掌机。

适用配置：**ArkOS4Clone 08262026 · Linux ARM64 4.4.189 · RTL8188EU · 640×480 屏幕**。

从 **Options → Nearby Multiplayer** 或主页 **FN + X** 打开附近房间列表，向下移动可展开「仅组网／选择游戏」。游戏加载前按住 **X** 可直接为当前游戏创建或加入房间。一台创建房间，另一台选中房间加入，双方准备完成后由房主开始游戏。Options 中的 **仅组网** 可用于 Ports 局域网游戏。

[安装与更新](https://github.com/W-Mai/arkos-nearby/blob/v0.8.0/docs/installation.md) · [游戏兼容性](https://github.com/W-Mai/arkos-nearby/blob/v0.8.0/docs/compatibility.md)
