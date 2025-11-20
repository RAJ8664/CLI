
# RNYTD - YouTube Downloader CLI

RNYTD is a simple and interactive command-line tool for downloading YouTube videos and extracting audio. It uses the powerful `yt-dlp` library to handle the downloading process.

![RNYTD Demo](./rnytd-demo.gif) 

## ✨ Features

-   **Interactive Interface**: User-friendly prompts to guide you through the downloading process.
-   **Download Video & Audio**: Choose to download the full video or extract the audio in MP3 format.
-   **Multiple Quality Options**: Select from a range of video and audio quality options to suit your needs.
-   **Pre-requisite Check**: Verifies if `yt-dlp` is installed and provides installation instructions if it's missing.
-   **Visually Appealing**: Uses colors, spinners, and progress bars to create a pleasant user experience.

## 🚀 Installation

First, ensure you have `yt-dlp` installed. If not, you can install it using pip:

```bash
pip install yt-dlp
```

Once `yt-dlp` is installed, you can install RNYTD from crates.io:

```bash
cargo install rnytd
```

## Usage

To start the YouTube downloader, run the following command in your terminal:

```bash
rnytd
```

The tool will then prompt you to enter a YouTube URL and guide you through the rest of the process.

## 📦 Dependencies

This project relies on the following Rust crates:

-   [dialoguer](https://crates.io/crates/dialoguer): For creating interactive prompts and menus.
-   [indicatif](https://crates.io/crates/indicatif): For displaying progress bars and spinners.
-   [console](https://crates.io/crates/console): For styling terminal output with colors and effects.

## 📜 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! If you have any ideas, suggestions, or bug reports, please open an issue or submit a pull request.

---

_This README was generated with the help of the Gemini CLI._
