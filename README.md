> 🀄 汉化分支：[BaoZiFly-233/volume-locker](https://github.com/BaoZiFly-233/volume-locker)
> 
> **原作者**：[Felipe Santos](https://github.com/felipecrs) · [上游仓库](https://github.com/felipecrs/volume-locker)
> **汉化**：[BaoZiFly-233](https://github.com/BaoZiFly-233) — 仅中文化界面文字与文档，功能未做任何改动

> ⚠️ **注意**：以下为原项目 README 的翻译。文中「我」指原作者 [Felipe Santos](https://github.com/felipecrs)，而非汉化者。汉化分支仅翻译界面，功能完全跟随上游。

<img align="right" width="200" alt="Volume Locker" src="https://github.com/user-attachments/assets/80473f4a-f901-440b-9d7b-5003ef1c96c7" />

<p align="center">
⭐<b>如果本工具对你有帮助，欢迎在 GitHub 上 star 原项目！</b>⭐
</p>

# Volume Locker

你是否受够了应用程序未经同意就擅自更改麦克风音量？或者 Windows 总是自动切换到错误的音频设备？

Volume Locker 是一款托盘图标应用，可以锁定音频设备的音量，并根据优先级列表管理音频与通信设备。它是一个基于 Rust 编写的 Windows 便携程序，体积约 1MB，绿色免安装。

从此再也不用担心麦克风音量或默认设备被意外更改！

## 演示

https://github.com/user-attachments/assets/b7e47898-ee9f-42b4-a804-f107beac4e98

## 快速开始

只需从 [releases 页面](https://github.com/BaoZiFly-233/volume-locker/releases) 下载可执行文件，将其放在某个目录（例如 `C:\Apps\Volume Locker\VolumeLocker.exe`），然后运行即可。

或者，你也可以复制以下内容并在 _Windows PowerShell_ 中执行：

```powershell
New-Item -ItemType Directory -Path 'C:\Apps\Volume Locker' -Force >$null; `
  Get-Process | Where-Object { $_.Path -eq 'C:\Apps\Volume Locker\VolumeLocker.exe' } | Stop-Process; `
  curl.exe --progress-bar --location --output 'C:\Apps\Volume Locker\VolumeLocker.exe' `
  'https://github.com/BaoZiFly-233/volume-locker/releases/latest/download/VolumeLocker.exe'; `
  Start-Process 'C:\Apps\Volume Locker\VolumeLocker.exe'
```

如需更新应用，再次运行上述命令即可。

## 使用方法

点击 Volume Locker 托盘图标即可打开菜单。菜单分为以下几个部分：

1.  **Output devices（输出设备）**：所有已激活的输出设备列表。
2.  **Input devices（输入设备）**：所有已激活的输入设备列表。
3.  **Output device priority（输出设备优先级）**：管理输出与通信音频的优先级。
4.  **Input device priority（输入设备优先级）**：管理输入与通信麦克风的优先级。
5.  **Temporary priority（临时优先级）**：为任意音频角色提供便捷的临时覆盖。

### 锁定音量与静音状态

要锁定某个设备的音量或静音状态：

1.  导航至 **Output devices（输出设备）** 或 **Input devices（输入设备）**。
2.  选择目标设备。
3.  勾选 **Keep volume locked（保持音量锁定）**，将音量锁定在当前级别。
4.  勾选 **Keep unmuted（保持非静音）**，防止设备被静音。
5.  你还可以为这些操作启用通知。

### 设备优先级

Volume Locker 可以根据优先级列表自动切换音频设备。

1.  在托盘菜单中导航至 **Output device priority（输出设备优先级）** 或 **Input device priority（输入设备优先级）**。当前激活的设备显示在 **Current: <device>（当前）** 下。
2.  打开 **Device priority settings（设备优先级设置）** 管理优先级列表：
    - 勾选 **Notify on restore（恢复时通知）**，在根据优先级切换设备时收到通知。
    - 使用 **Move up（上移）** 和 **Move down（下移）** 调整设备顺序。最顶部的设备优先级最高。
    - 在 **Add device（添加设备）** 下选择设备，然后选 **Add to top（添加到顶部）** 或 **Add to bottom（添加到底部）**。
3.  打开 **Communication priority settings（通信优先级设置）** 配置通信设备的切换。

### 通信优先级设置

输出和输入各自支持独立的通信优先级列表，用于语音通话和聊天软件。

1.  在输出或输入下打开 **Communication priority settings（通信优先级设置）**。
2.  默认情况下，通信设备跟随主优先级列表（**Output device priority（输出设备优先级）** 或 **Input device priority（输入设备优先级）**）。
3.  你可以调整优先级顺序、添加优先于「跟随主列表」的特定设备，或移除「主优先级」条目。
4.  在通信优先级设置中勾选 **Notify on restore（恢复时通知）**，在通信设备切换时收到通知。

### 临时优先级

在不更改已保存的优先级列表的情况下，将音频临时路由到指定设备：

1.  在托盘菜单中打开 **Temporary priority（临时优先级）**。
2.  选择目标角色：**Output（输出）**、**Communication（通信）**、**Input（输入）** 或 **Communication（通信）**。子菜单标签会显示当前选中的设备。
3.  选择要临时使用的设备。在你取消勾选或应用重启之前，该设置保持有效。临时的通信覆盖优先于已配置的通信优先级。

## 致谢

> 以下为原作者 Felipe Santos 的自述。

Volume Locker 是我第一个 Rust 项目。它源于对现有解决方案的不满——那些工具要么依赖闭源软件，要么缺乏特定的设备锁定能力。如今它已发展出包括默认设备优先级管理等高级功能。

在此之前，我长期使用 [wolfinabox/Windows-Mic-Volume-Locker](https://github.com/wolfinabox/Windows-Mic-Volume-Locker)，直到决定亲手编写自己的解决方案。

[AntoineGS/teams-status-rs](https://github.com/AntoineGS/teams-status-rs) 以如此轻量级的体积实现如此出色的功能，给了我很大启发。我希望能做出类似但专注于音量锁定的工具。

特别感谢：

- [Kingloo/volume](https://github.com/Kingloo/volume) 提供了 Windows 音量控制的代码基础。
- [tauri-apps/tray-icon](https://github.com/tauri-apps/tray-icon) 实现了托盘图标功能。
- [GitHub Copilot](https://github.com/copilot/) 协助了代码结构、语法和功能实现。
- [@jmb](https://github.com/jmb) 设计了[出色的图标](https://github.com/Templarian/MaterialDesign/issues/7714)。

---

## 🀄 汉化分支说明

本仓库是 [felipecrs/volume-locker](https://github.com/felipecrs/volume-locker) 的汉化分支，由 [BaoZiFly-233](https://github.com/BaoZiFly-233) 维护。

- **仅汉化**：界面文字和文档翻译为简体中文，功能代码未做任何改动。
- **跟随上游**：汉化分支仅做翻译，功能和 bug 修复完全跟随 [上游仓库](https://github.com/felipecrs/volume-locker)，issue 和 feature request 请提交至上游。
- **捐助说明**：如需捐助，请捐助原作者 [Felipe Santos](https://www.paypal.com/donate/?business=XSNV4QP8JFY9Y&no_recurring=0&item_name=Built+and+maintained+out+of+passion.+Always+FOSS.+Donations+appreciated.+%28smtty%2C+MicLockTray%2C+awtarchy%29&currency_code=USD)，汉化分支不接受捐助。
