<img align="right" width="200" alt="Volume Locker" src="https://github.com/user-attachments/assets/80473f4a-f901-440b-9d7b-5003ef1c96c7" />

# Volume Locker（中文分支 · 已停止维护）

简体中文翻译已经合并进上游，**从 v0.12.0 起，上游的官方版本自带简体中文界面**。

这个分支不再更新，请前往上游：

## 👉 [felipecrs/volume-locker](https://github.com/felipecrs/volume-locker)

安装或更新，把下面这段粘进 _Windows PowerShell_：

```powershell
New-Item -ItemType Directory -Path 'C:\Apps\Volume Locker' -Force >$null; `
  Get-Process | Where-Object { $_.Path -eq 'C:\Apps\Volume Locker\VolumeLocker.exe' } | Stop-Process; `
  curl.exe --progress-bar --location --output 'C:\Apps\Volume Locker\VolumeLocker.exe' `
  'https://github.com/felipecrs/volume-locker/releases/latest/download/VolumeLocker.exe'; `
  Start-Process 'C:\Apps\Volume Locker\VolumeLocker.exe'
```

---

## 关于这个仓库

这里曾经是一个手工汉化的分支，因为在很长一段时间里，上游只有英文界面。每次上游发版，都要重新对照源码把中文补一遍。

2026 年 11 月，作者 [Felipe Santos](https://github.com/felipecrs) 为项目引入了 [Fluent](https://projectfluent.org/) 本地化框架（[#195](https://github.com/felipecrs/volume-locker/pull/195)），并邀请本仓库把中文翻译贡献回去。从这里开始，中文不再需要单独维护。

随后发生的事：

- 简体中文翻译以 [PR #205](https://github.com/felipecrs/volume-locker/pull/205) 合并进上游，`i18n/zh-Hans/volume_locker.ftl` 现在是上游源码的一部分。
- 那套用词被作者采纳，反过来改写了英文原文（[#209](https://github.com/felipecrs/volume-locker/pull/209)、[#210](https://github.com/felipecrs/volume-locker/pull/210)）：`Device Priority` 在英文里也改名成了 `Default device order`。

也就是说，这个 fork 存在的理由已经消失。后续的 bug 修复、新功能、以及中文译文的更新，全部在上游进行。

## 旧的发布包

本仓库 releases 里 `vX.Y.Z-zh` 命名的构建是历史遗留，版本停留在同步上游的某个时点，**不会再有更新**，也不建议使用。它们的功能与对应版本的上游一致，只是界面为中文。

## 致谢

Volume Locker 的作者是 [Felipe Santos](https://github.com/felipecrs)。这个项目用 Rust 写成，是一个约 2MB 的 Windows 便携程序：锁定音频设备的音量，按默认设备顺序切换音频与通信设备，并在 Windows 把设备当成新设备时保留它的设置、名称和图标。

作者在 [#205](https://github.com/felipecrs/volume-locker/pull/205) 的合并留言里写道：

> I know the pain of maintaining a fork just because of some little missing feature, so I can relate to you.

原仓库的致谢名单（[Kingloo/volume](https://github.com/Kingloo/volume)、[tauri-apps/tray-icon](https://github.com/tauri-apps/tray-icon)、[@jmb](https://github.com/jmb) 的图标设计等）请见[上游 README](https://github.com/felipecrs/volume-locker#credits)。
