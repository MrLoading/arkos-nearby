# ArkOS Nearby 0.9.0

- 新增 **Options → Nearby Launcher**，从个人收藏开始选游戏，短按 X 启动单机，长按 X 打开当前游戏的联机面板。
- 横向收藏卡带展示游戏实图，按可用空间排列成一排或两排，支持方向键翻页和名称／最近游玩排序。
- 选中末尾的游戏库图标可展开游戏库，上方保留收藏缩略图；返回时恢复原来的选中位置。
- 增加游戏操作与设置面板，统一按键提示、选中反馈、小人动效和音效。
- 优化游戏库浏览与联机面板响应，修复长按 X 时收藏布局变化和操作提示错误。
- 单机沿用系统的模拟器设置和存档，退出游戏后回到刚才的位置。

![双机联机演示](https://raw.githubusercontent.com/W-Mai/arkos-nearby/v0.9.0/docs/assets/nearby-dual-demo.svg)

## 安装与更新

下载 **arkos-nearby-0.9.0-arm64.tar.gz**，解压后将 `arkos-nearby` 文件夹放入游戏卡的 `ports` 目录，在掌机 **Ports → Install Nearby Multiplayer** 中安装。已有版本使用相同方式更新，两台掌机安装同一版本，安装完成后重启掌机。

适用配置：**ArkOS4Clone 08262026 · Linux ARM64 4.4.189 · RTL8188EU · 640×480 屏幕**。

从 **Options → Nearby Multiplayer** 或主页 **FN + X** 打开附近房间列表；从 **Options → Nearby Launcher** 打开个人收藏。游戏加载前按住 **X** 可直接为当前游戏创建或加入房间。一台创建房间，另一台选中房间加入，双方准备完成后由房主开始游戏。**仅组网** 可用于 Ports 局域网游戏。

[安装与更新](https://github.com/W-Mai/arkos-nearby/blob/v0.9.0/docs/installation.md) · [游戏兼容性](https://github.com/W-Mai/arkos-nearby/blob/v0.9.0/docs/compatibility.md)
