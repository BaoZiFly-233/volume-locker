<img align="right" width="200" alt="Volume Locker" src="https://github.com/user-attachments/assets/80473f4a-f901-440b-9d7b-5003ef1c96c7" />

<p align="center">
⭐<b>如果这个项目帮到了你，请在 GitHub 上给它点个 Star！</b>⭐
</p>

# Volume Locker

受够了应用未经允许就改你的麦克风音量？受够了 Windows 总是切到错误的音频设备？受够了换个 USB 口就丢掉设备名称和图标？

Volume Locker 是一个托盘应用：它锁定音频设备的音量，并按优先级列表切换你的音频与通信设备。换 USB 口、蓝牙设备重新配对或者更新驱动时，Windows 会把设备当成新设备，它会把你的设置连同设备名称、图标一起迁移过去。它是一个用 Rust 写的 Windows 便携程序，体积约 2MB。

## 演示

https://github.com/user-attachments/assets/b7e47898-ee9f-42b4-a804-f107beac4e98

## 快速开始

把下面这段粘进 _Windows PowerShell_ 即可安装或更新：

```powershell
New-Item -ItemType Directory -Path 'C:\Apps\Volume Locker' -Force >$null; `
  Get-Process | Where-Object { $_.Path -eq 'C:\Apps\Volume Locker\VolumeLocker.exe' } | Stop-Process; `
  curl.exe --progress-bar --location --output 'C:\Apps\Volume Locker\VolumeLocker.exe' `
  'https://github.com/BaoZiFly-233/volume-locker/releases/latest/download/VolumeLocker.exe'; `
  Start-Process 'C:\Apps\Volume Locker\VolumeLocker.exe'
```

也可以从[发布页](https://github.com/BaoZiFly-233/volume-locker/releases)下载可执行文件，放到 `C:\Apps\Volume Locker\VolumeLocker.exe` 之类的位置再运行。

## 使用方法

点击托盘里的 Volume Locker 图标即可打开菜单。

### 锁定音量与取消静音

1.  打开 **输出设备** 或 **输入设备**，选中一个设备。
2.  勾选 **锁定音量**，把音量固定在当前水平。
3.  勾选 **保持不静音**，防止设备被静音。

**音量恢复时通知** 和 **取消静音时通知** 会在 Volume Locker 动手时弹通知。

### 设置默认设备顺序

Volume Locker 会把默认设备切换成顺序里第一个可用的设备。

1.  打开 **输出设备的默认顺序** 或 **输入设备的默认顺序**。
2.  打开 **设备顺序设置**。
3.  在 **添加设备** 下选中一个设备，然后点 **添加到顶部** 或 **添加到底部**。
4.  在 **当前顺序** 里选中某个设备，即可上移、下移或移除它。

**通信设备顺序设置** 对语音通话和聊天软件是同样的道理，默认跟随设备顺序列表。

在任意一个列表里勾选 **恢复时通知**，Volume Locker 切换设备时就会给你提示。

### 临时使用某个设备

如果想临时覆盖顺序列表、又不想改动它：

1.  打开 **临时设备顺序**。
2.  选择 **输出**、**输入**，或者它们下面的 **通信** 角色。
3.  选中设备。

该设置会一直生效，直到你取消勾选或重启应用。

### 跳过已断开的无线耳机

有些无线接收器在耳机断电或超出范围时仍然保持连接。Volume Locker 会跳过处于这种状态的受支持耳机，所以请在顺序列表里把备用设备排在耳机下面。详见[耳机可用性](docs/headset-availability.md)。

### 保留你的设置、名称和图标

换 USB 口、重新配对或更新驱动时，Windows 有时会把你的设备当成一个全新的设备。按常理，你会丢掉自己起的名称、图标，以及一切与该设备绑定的东西。

Volume Locker 能认出这是同一个设备，并把它的音量锁定、保持不静音、顺序列表条目、名称和图标一并迁移过去。

详见[设置迁移](docs/settings-transfer.md)和[迁移名称与图标](docs/device-names-and-icons.md)。

### 使用你的语言

Volume Locker 跟随 Windows 显示语言。目前内置英文和简体中文，要添加其它语言见[本地化](docs/localization.md)。

### 首选项

| 首选项                                     | 默认 | 作用                                     |
| ------------------------------------------ | ---- | ---------------------------------------- |
| 设备设置迁移 > 自动迁移设备设置            | 开   | Windows 把设备当成新设备时迁移它的设置   |
| 设备设置迁移 > 同时迁移设备名称和图标      | 开   | 连同设置一起迁移设备的 Windows 名称和图标 |
| 设备设置迁移 > 设备迁移时通知              | 关   | 发生设置迁移时弹通知                     |
| 跳过已断开的无线耳机                       | 开   | 跳过已断电或超出范围的受支持耳机         |
| 登录时自动启动                             | 关   | 登录 Windows 时启动 Volume Locker        |
| 启动时检查更新                             | 开   | 应用启动时检查有没有新版本               |

## 致谢

Volume Locker 是作者的第一个 Rust 项目，起因是对现有方案不满意——它们要么依赖闭源工具，要么缺少针对具体设备的锁定能力。后来它逐渐长出了默认设备顺序管理。

在决定自己动手之前，作者用了 [wolfinabox/Windows-Mic-Volume-Locker](https://github.com/wolfinabox/Windows-Mic-Volume-Locker) 好几年。

[AntoineGS/teams-status-rs](https://github.com/AntoineGS/teams-status-rs) 让作者意识到这么轻量的包里也能做出很棒的东西，于是想照着做一个，只不过方向是音量锁定。

特别感谢：

- [Kingloo/volume](https://github.com/Kingloo/volume)：Windows 音量控制代码的基础。
- [tauri-apps/tray-icon](https://github.com/tauri-apps/tray-icon)：托盘图标的实现。
- [GitHub Copilot](https://github.com/copilot/)：在代码结构、语法和功能实现上提供了协助。
- [@jmb](https://github.com/jmb)：[出色的图标设计](https://github.com/Templarian/MaterialDesign/issues/7714)。

本仓库是 [felipecrs/volume-locker](https://github.com/felipecrs/volume-locker) 的中文分支，跟随上游更新，并维护简体中文翻译与对应的发布包。
