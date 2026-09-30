# crystalify

A modern, high-performance headless Spotify CLI and TUI client written in Rust. It features real-time 3D audio visualizers, synchronized lyrics, and inline album artwork rendering directly in your terminal.

---

## Features

- Headless Spotify Playback: Built on Spotify Connect protocols, allowing crystalify to act as an independent audio sink and remote-controllable playback device.
- Real-Time 3D Audio Visualizer: Audio stream analysis via Fast Fourier Transform (FFT) projected into dynamic 3D meshes and point clouds rendered in terminal space.
- Terminal Album Artwork: High-resolution cover art rendering supporting Kitty graphics protocol, Sixel, and Unicode halfblock fallbacks.
- Synchronized Lyrics: Real-time scrolling lyrics synchronized with track playback timestamps.
- Low Resource Footprint: Native async Rust implementation designed for speed, minimal CPU overhead, and responsive terminal interaction.

---

## Architecture

The project is structured into distinct, modular subsystems designed for concurrency and separation of concerns:

```
src/
├── main.rs                   # Entry point and runtime initialization
├── app.rs                    # Central application state machine and event dispatcher
├── event.rs                  # Input handling, timer ticks, and cross-thread channels
├── config/
│   └── mod.rs                # Configuration management, credentials, and settings
├── spotify/                  # Spotify streaming and metadata integration
│   ├── mod.rs                # Module declarations and interfaces
│   ├── auth.rs               # Authentication, token exchange, and credential caching
│   ├── client.rs             # Spotify Web API client (search, playlists, lyrics, metadata)
│   ├── models.rs             # Strongly typed data models for API responses
│   └── player.rs             # Librespot audio backend, session lifecycle, and Spirc events
├── audio/                    # Audio signal processing and mathematical models
│   ├── mod.rs                # Audio sink intercepts and stream pipelines
│   ├── fft.rs                # Spectrum extraction via RustFFT
│   └── visualizer.rs         # 3D coordinate transformations, projections, and matrix math (glam)
└── ui/                       # Ratatui TUI rendering
    ├── mod.rs                # Root UI layout, draw passes, and view router
    ├── theme.rs              # Color palettes, borders, and typography
    └── components/           # Isolated UI widgets
        ├── mod.rs
        ├── player.rs         # Playback controls, progress bars, track info
        ├── playlist.rs       # Library browser, queue, and playlist manager
        ├── statusbar.rs      # Device status, network state, and volume levels
        └── visualizer.rs     # Canvas rendering for 3D visualizers and album art
```

---

## Dependencies and Crates

| Crate | Purpose |
|---|---|
| `librespot` | Headless Spotify Connect player implementation, handling authentication, audio stream decryption, and PCM decoding. |
| `tokio` | Asynchronous multi-threaded runtime managing background tasks, audio pipelines, API requests, and event loops. |
| `ratatui` | Terminal UI framework providing windowing, layout trees, widgets, and canvas primitives. |
| `crossterm` | Cross-platform terminal control enabling raw mode, alternate screens, and keyboard/mouse event capture. |
| `ratatui-image` | Inline terminal image rendering for album covers using Kitty, Sixel, or Unicode halfblocks. |
| `rustfft` | High-performance Fast Fourier Transform library computing frequency domain spectrums from audio streams. |
| `glam` | SIMD-accelerated 3D linear algebra library used for perspective projection, rotations, and 3D visualizer geometry. |
| `reqwest` | Async HTTP client for interacting with the Spotify Web API and fetching lyrics and cover art. |
| `serde` & `serde_json` | Serialization framework for parsing API payloads, config files, and cached session tokens. |

---

## Requirements

### Operating System & System Libraries
- Linux (ALSA or PulseAudio / PipeWire) or macOS.
- Development headers for ALSA (Linux):
  - Debian / Ubuntu: `sudo apt install libasound2-dev pkg-config`
  - Fedora / RHEL: `sudo dnf install alsa-lib-devel pkg-config`
  - Arch Linux: `sudo pacman -S alsa-lib pkg-config`

### Spotify Account
- Spotify Premium account (required by the Spotify Connect / librespot protocol).
- Spotify Developer Application credentials (Client ID and Client Secret) for Web API access.

### Terminal
- A terminal emulator supporting TrueColor (24-bit color).
- Optional (for high-resolution album artwork): terminals supporting the Kitty graphics protocol (e.g., Kitty, Ghostty, WezTerm) or Sixel (e.g., Foot, Alacritty with sixel patch). Standard unicode halfblock fallback is used on other terminals.

### Rust Toolchain
- Rust 1.85+ with Cargo (2024 edition support).

---

## Installation

### 1. Install System Dependencies

On Debian/Ubuntu:
```bash
sudo apt update
sudo apt install build-essential pkg-config libasound2-dev
```

On Fedora:
```bash
sudo dnf install gcc pkg-config alsa-lib-devel
```

On Arch Linux:
```bash
sudo pacman -S base-devel pkg-config alsa-lib
```

### 2. Clone the Repository

```bash
git clone https://github.com/3entropyy/crystalify.git
cd crystalify
```

### 3. Build the Project

Build an optimized release binary:
```bash
cargo build --release
```

The compiled binary will be located at `target/release/crystalify`.

### 4. Running the Client

Run crystalify directly via Cargo:
```bash
cargo run --release
```

Or execute the binary directly:
```bash
./target/release/crystalify
```

---

## License

This project is licensed under the Apache License, Version 2.0. See the [LICENSE](LICENSE) file for details.
