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

Volume Locker 是一款托盘图标应用，可以锁定音频设备的音量，并根据设备顺序管理音频与通信设备。换了 USB 口、蓝牙耳机重新配对、或者更新了声卡驱动之后，它也能把音量锁定、保持不静音和设备顺序原样迁移过去。它是一个基于 Rust 编写的 Windows 便携程序，体积约 1MB，绿色免安装。

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

1.  **输出设备（Output devices）**：所有已激活的输出设备列表。
2.  **输入设备（Input devices）**：所有已激活的输入设备列表。
3.  **输出设备的默认顺序（Output device priority）**：管理输出与通信音频的切换顺序。
4.  **输入设备的默认顺序（Input device priority）**：管理输入与通信麦克风的切换顺序。
5.  **临时设备顺序（Temporary priority）**：为任意音频角色提供便捷的临时覆盖。

### 锁定音量与静音状态

要锁定某个设备的音量或静音状态：

1.  导航至 **输出设备** 或 **输入设备**。
2.  选择目标设备。
3.  勾选 **锁定音量（Keep volume locked）**，将音量锁定在当前级别。
4.  勾选 **保持不静音（Keep unmuted）**，防止设备被静音。
5.  你还可以为这些操作启用通知。
6.  设备列表末尾的 **播放设备...** 或 **录制设备...** 会打开 Windows 声音控制面板的对应选项卡。

### 默认设备顺序

Volume Locker 会按你排好的顺序自动切换音频设备。

1.  在托盘菜单中导航至 **输出设备的默认顺序** 或 **输入设备的默认顺序**。当前生效的设备显示在 **当前：<设备名>** 下。
2.  打开 **设备顺序设置** 管理这台设备的切换顺序：
    - 勾选 **恢复时通知**，在工具按顺序切换设备时收到通知。
    - 使用 **上移** 和 **下移** 调整顺序。排在最上面的设备最先被选中。
    - 在 **添加设备** 下选择设备，然后选 **添加到顶部** 或 **添加到底部**。
3.  打开 **通信设备顺序设置** 单独配置通话时用的设备。

### 通信设备顺序设置

输出和输入各自有一份独立的通信设备顺序，用于语音通话和聊天软件。

1.  在输出或输入下打开 **通信设备顺序设置**。
2.  默认情况下，通信设备跟随输出（或输入）的默认顺序，由 **跟随系统默认设备** 这一条目表示。
3.  你可以调整顺序、把某个设备排到它前面，或移除 **跟随系统默认设备** 条目（移除后通信设备就不再跟随主顺序）。
4.  在这里勾选 **恢复时通知**，在通信设备切换时收到通知。

### 临时设备顺序

在不改动已保存的设备顺序的前提下，临时把音频切到某台设备：

1.  在托盘菜单中打开 **临时设备顺序**。
2.  选择目标角色：**输出**、**通信**（输出通信）、**输入** 或 **通信**（输入通信）。子菜单标签会显示当前选中的设备。
3.  选择要临时使用的设备。在你取消勾选或应用重启之前，该设置保持有效。临时的通信选择优先于已保存的通信设备顺序。

### 实验性：无线耳机的可用性检测

无线接收器可能一直连着，而耳机本身是关着的。这里提供一个需要手动开启的被动式 HID 检测：顺序切换时会跳过处于关机状态的 A50 X。在设备菜单的 **可用性（Availability）** 子菜单里开启，并可指定一个替补设备。刚启动时耳机状态未知，要等到第一次检测到电源事件才会确定。固件方面的限制、只读诊断脚本，以及用于支持其他设备的协议适配接口见 [实验性耳机可用性](docs/experimental-headset-presence.md)。目前只支持 A50 X 的电源事件。

### 设备 ID 变化时无缝迁移

把设备插到另一个 USB 口、蓝牙耳机取消配对后重新配对、或者更新音频驱动之后，Windows 经常会重新生成音频设备的端点 ID。

Volume Locker 会跨过 ID 变化识别出同一台硬件，并把音量锁定、保持不静音设置、设备顺序迁移到新的连接上。完整的设计说明见 [docs/device-id-migration.md](docs/device-id-migration.md)。

### 首选项

在 **首选项（Preferences）** 中配置全局行为：

- **登录时自动启动**：登录 Windows 时自动启动 Volume Locker。
- **启动时检查更新**：启动时自动检查新版本。
- **设备迁移时通知**：设置被迁移到重新连接的设备时弹出桌面通知。

## 致谢

> 以下为原作者 Felipe Santos 的自述。

Volume Locker 是我第一个 Rust 项目。它源于对现有解决方案的不满——那些工具要么依赖闭源软件，要么缺乏特定的设备锁定能力。如今它已发展出包括默认设备顺序管理等高级功能。

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
