> 🀄 汉化分支：[BaoZiFly-233/volume-locker](https://github.com/BaoZiFly-233/volume-locker)
> 
> **原作者**：[Felipe Santos](https://github.com/felipecrs) · [上游仓库](https://github.com/felipecrs/volume-locker)
> **汉化**：[BaoZiFly-233](https://github.com/BaoZiFly-233) — 仅中文化界面文字与文档，功能未做任何改动

> ⚠️ **注意**：以下为原项目 README 的翻译。文中「我」指原作者 [Felipe Santos](https://github.com/felipecrs)，而非汉化者。汉化分支仅翻译界面，功能完全跟随上游。

<img align="right" width="200" alt="Volume Locker" src="https://github.com/user-attachments/assets/80473f4a-f901-440b-9d7b-5003ef1c96c7" />

<p align="center">
⭐<b>如果本工具对你有帮助，欢迎在 GitHub 上 star 本项目！</b>⭐
</p>

# Volume Locker

你是否受够了应用程序未经同意就擅自更改麦克风音量？或者 Windows 总是自动切换到错误的音频设备？

Volume Locker 是一款托盘图标应用，可以锁定音频设备的音量，并根据优先级列表管理默认音频设备。它是一个基于 Rust 编写的 Windows 便携程序，体积约 1MB，绿色免安装。

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
3.  **Default output device priority（默认输出设备优先级）**：管理默认输出设备的优先级列表。
4.  **Default input device priority（默认输入设备优先级）**：管理默认输入设备的优先级列表。
5.  **Temporary default device priority（临时默认设备优先级）**：临时覆盖默认设备优先级。

### 锁定音量与静音状态

要锁定某个设备的音量或静音状态：

1.  导航至 **Output devices（输出设备）** 或 **Input devices（输入设备）**。
2.  选择目标设备。
3.  勾选 **Keep volume locked（保持音量锁定）**，将音量锁定在当前级别。
4.  勾选 **Keep unmuted（保持非静音）**，防止设备被静音。
5.  你还可以为这些操作启用通知。

### 默认设备优先级

Volume Locker 可以根据优先级列表自动切换默认音频设备。如果你有多个设备（例如扬声器和耳机），并希望确保在可用时始终使用某个特定设备，此功能非常实用。

1.  导航至 **Default output device priority（默认输出设备优先级）** 或 **Default input device priority（默认输入设备优先级）**。
2.  选择 **Add device（添加设备）** 将设备加入优先级列表。
3.  使用 **Move Up（上移）** 和 **Move Down（下移）** 调整优先级顺序。列表顶部的设备优先级最高。
4.  Volume Locker 会持续监控你的设备，并自动将默认设备切换为优先级最高的可用设备。
5.  勾选 **Notify on restore（恢复时通知）**，在默认设备被切换时收到通知。
6.  勾选 **Also switch default communication device（同时切换默认通信设备）**，一并切换默认通信设备。

### 临时默认设备优先级

如果你希望临时使用其他设备而又不想更改优先级列表（例如在耳机已连接的情况下临时切换到扬声器进行通话），可以使用 **Temporary default device priority（临时默认设备优先级）** 功能。

1.  导航至 **Temporary default device priority（临时默认设备优先级）**。
2.  选择 **Output device（输出设备）** 或 **Input device（输入设备）**。
3.  选择你希望临时使用的设备。
4.  该设备将被视作最高优先级设备，直至你取消选中或重启应用。

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
