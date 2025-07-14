use clap::Parser;
use comfy_table::{presets::UTF8_FULL, Cell, Color, ContentArrangement, Table};
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Cli {}

fn main() {
    let _cli = Cli::parse();

    let output = Command::new("nmcli")
        .args(&["-g", "NAME", "connection", "show"])
        .output()
        .expect("Failed to execute nmcli. Is NetworkManager installed?");
    let ssids = String::from_utf8_lossy(&output.stdout);

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("WiFi Network")
                .fg(Color::Blue)
                .add_attribute(comfy_table::Attribute::Bold),
            Cell::new("Password")
                .fg(Color::Blue)
                .add_attribute(comfy_table::Attribute::Bold),
        ]);

    for ssid in ssids.lines() {
        let ssid = ssid.trim();
        if ssid.is_empty() {
            continue;
        }
        let pass_output = Command::new("nmcli")
            .args(&[
                "-s",
                "-g",
                "802-11-wireless-security.psk",
                "connection",
                "show",
                ssid,
            ])
            .output()
            .expect("Failed to get password");
        let password = String::from_utf8_lossy(&pass_output.stdout)
            .trim()
            .to_string();
        let password_cell = if password.is_empty() {
            Cell::new("Not found or no password required").fg(Color::Red)
        } else {
            Cell::new(password).fg(Color::Green)
        };
        table.add_row(vec![Cell::new(ssid).fg(Color::Yellow), password_cell]);
    }
    println!("{table}");
}
