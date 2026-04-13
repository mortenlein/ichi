<p align="center">
  <img src="icon.png" width="160" alt="Ichi Logo">
</p>

<h1 align="center">ICHI ・ イチ</h1>

<p align="center">
  <strong>The definitive zero-flicker window snapping utility for Windows.</strong><br>
  <em>Built in Rust for power, precision, and performance.</em>
</p>

<p align="center">
  <a href="https://github.com/mortenlein/ichi/releases/latest">
    <img src="https://img.shields.io/github/v/release/mortenlein/ichi?style=for-the-badge&color=D0021B" alt="Latest Release">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/github/license/mortenlein/ichi?style=for-the-badge&color=333" alt="License">
  </a>
  <img src="https://img.shields.io/badge/Language-Rust-black?style=for-the-badge&logo=rust" alt="Rust">
</p>

---


**Ichi** is a high-performance, minimalist window-snapping utility for Windows, rebuilt from the ground up in Rust. It provides a lightweight, flicker-free alternative to legacy window managers, inspired by the classic WinSplit Revolution.

## 🚀 Key Features

- **Blazing Fast**: Rewritten in Rust with direct Win32 API calls for near-zero latency.
- **Zero-Flicker Snapping**: Uses optimized window positioning logic (`WM_SETREDRAW`) to eliminate the visual "jitters" common in other managers.
- **Stealth Mode**: Operation is entirely windowless and OSD-free, ensuring it never steals focus or interferes with your workflow.
- **Multi-Tap Cycling**: Pressing the same hotkey multiple times cycles through different layout ratios (1/2, 1/3, 2/3).

## ⌨️ Global Hotkeys

All actions are triggered via **Ctrl + Alt + Numpad**:

| Hotkey | Action | Cycle Ratios |
| :--- | :--- | :--- |
| **Numpad 5** | Center / Maximize | 100% → 80% → 60% Center |
| **Numpad 4 / 6** | Left / Right Half | 1/2 → 1/3 → 2/3 Width |
| **Numpad 8 / 2** | Top / Bottom Half | 1/2 → 1/3 → 2/3 Height |
| **Numpad 7 / 9** | Top Left / Right | 1/2 → 1/3 → 2/3 Corner |
| **Numpad 1 / 3** | Bottom Left / Right | 1/2 → 1/3 → 2/3 Corner |

## 🏗️ Architecture

Ichi has transitioned from a legacy C++ codebase to a modern, safe, and performant Rust architecture:

1.  **Direct Win32 Hooks**: Uses low-level keyboard hooks (`WH_KEYBOARD_LL`) to capture hotkeys globally without a persistent window.
2.  **Pure Functional Engine**: Calculation logic is isolated from OS side effects, allowing for predictable and rock-solid window placement.
3.  **Minimalist Dependency Tree**: Built using the official Microsoft `windows-rs` crate, ensuring maximum compatibility and minimum binary size.

## 🛠️ Building & Installation

### Prerequisites
- [Rust Toolchain](https://rustup.rs/) (Stable)
- Windows 10 or 11

### Build from source
```bash
cargo build --release
```
The resulting binary will be located at `target/release/ichi.exe`. Simply run it to start the background service.

## 📜 License
MIT License - See [LICENSE](LICENSE) for details.
