<div align="center">

# 🎬 RNYTD

### A Fast & Beautiful YouTube Downloader CLI

[![Crates.io](https://img.shields.io/crates/v/rnytd.svg)](https://crates.io/crates/rnytd)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/rnytd.svg)](https://crates.io/crates/rnytd)

<img src="https://res.cloudinary.com/dzgoq3ikq/image/upload/v1763718484/Screenshot_21-Nov_15-17-15_7379_kjvnwi.png" alt="RNYTD Screenshot" width="700"/>

_Download YouTube videos and extract audio with style_ 🎵

</div>

---

## ✨ Features

🎯 **Interactive Experience** - Beautiful command-line interface with intuitive prompts and menus

📹 **Video Downloads** - Download videos in multiple quality options from 144p to 4K

🎵 **Audio Extraction** - Extract high-quality MP3 audio from any YouTube video

⚡ **Lightning Fast** - Built with Rust for optimal performance and speed

🎨 **Visual Feedback** - Colorful output with progress bars and spinners for real-time status

🔍 **Smart Validation** - Automatic checks for dependencies and URL validation

💾 **Quality Control** - Choose your preferred video and audio quality before downloading

## 🚀 Installation

### Prerequisites

RNYTD requires `yt-dlp` to function. Install it using pip:

```bash
pip install yt-dlp
```

### Install RNYTD

Once `yt-dlp` is ready, install RNYTD from crates.io:

```bash
cargo install rnytd
```

That's it! You're ready to start downloading.

## 📖 Usage

Launch the downloader by running:

```bash
rnytd
```

Follow the interactive prompts to:

1. **Enter YouTube URL** - Paste any valid YouTube video link
2. **Choose Download Type** - Select between video or audio-only download
3. **Select Quality** - Pick your preferred quality from available options
4. **Sit Back** - Watch the progress bar as your content downloads

### Example Session

```
🎬 RNYTD - YouTube Downloader
━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📺 Enter YouTube URL: https://youtube.com/watch?v=...
⚙️  Choose download type: Video with Audio
🎯 Select quality: 1080p
⬇️  Downloading... ████████████████████ 100%
✅ Download complete!
```

## 🛠️ Built With

This project is powered by excellent Rust crates:

- **[dialoguer](https://crates.io/crates/dialoguer)** - Interactive CLI prompts and menus
- **[indicatif](https://crates.io/crates/indicatif)** - Progress bars and spinners
- **[console](https://crates.io/crates/console)** - Terminal styling and colors
