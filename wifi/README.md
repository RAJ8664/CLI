<div align="center">

# 🔐 WiFi Password Checker

### Instantly Retrieve Your Saved WiFi Passwords

[![Crates.io](https://img.shields.io/crates/v/wifi-checker.svg)](https://crates.io/crates/wifi-checker)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/wifi-checker.svg)](https://crates.io/crates/wifi-checker)

<img src="https://res.cloudinary.com/dzgoq3ikq/image/upload/v1763719328/Screenshot_21-Nov_15-31-05_25689_ctjkns.png" alt="WiFi Checker Screenshot" width="700"/>

_Never forget a WiFi password again_ 📡

</div>

---

## ✨ Features

📋 **Quick Access** - View all your saved WiFi passwords in seconds

🎨 **Clean Display** - Beautiful table format for easy reading

🔍 **Complete History** - See every network you've ever connected to

⚡ **Lightning Fast** - Built with Rust for instant results

🔒 **Secure** - Reads directly from NetworkManager without compromising security

💻 **Simple CLI** - One command to see all your passwords

## 🚀 Installation

### Prerequisites

This tool requires:

- **Linux Operating System** - Designed specifically for Linux distributions
- **NetworkManager** - Must be installed and running on your system

Most modern Linux distributions come with NetworkManager pre-installed. You can verify by running:

```bash
nmcli --version
```

### Install WiFi Checker

Install directly from crates.io using cargo:

```bash
cargo install wifi-checker
```

## 📖 Usage

Run the tool with a single command:

```bash
wifi-checker
```

The tool will display a formatted table showing:

- **SSID** - Network names you've connected to
- **Password** - The saved password for each network

### Example Output

```
┌──────────────────────┬──────────────────────┐
│ SSID                 │ Password             │
├──────────────────────┼──────────────────────┤
│ Home_Network         │ MySecurePass123      │
│ Office_WiFi          │ OfficePass456        │
│ CoffeeShop_Guest     │ freewifi2024         │
└──────────────────────┴──────────────────────┘
```

## 🛡️ Security Note

This tool accesses system-level network configuration files. Depending on your system's security settings, you may need to run it with elevated privileges:

```bash
sudo wifi-checker
```

Your WiFi passwords are read directly from NetworkManager's secure storage and are only displayed in your terminal.
