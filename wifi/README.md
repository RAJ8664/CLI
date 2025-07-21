# WiFi Password Checker

[Click here to view the crate on crates.io](https://crates.io/crates/wifi-checker)

A simple command-line tool for Linux to quickly view the passwords of your saved WiFi networks. It reads data from NetworkManager via the `nmcli` command.

## Description

This tool lists all the WiFi network SSIDs you have connected to in the past and displays their corresponding passwords in a clean, readable table. This is useful when you need to recall a password for a network you've previously used.

## Requirements

- **Linux Distribution**: This tool is designed for Linux.
- **NetworkManager**: You must have NetworkManager installed and enabled, as this tool relies on `nmcli`.

## Installation

You can install `wifi-checker` directly from crates.io using cargo:

```bash
cargo install wifi-checker
```

## Usage

Simply run the command in your terminal:

```bash
wifi-checker
```

The tool will display a table with all your saved WiFi networks and their passwords.
