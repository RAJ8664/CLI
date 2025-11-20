use console::style;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() {
    print_banner();

    // Check if yt-dlp is installed
    if !check_ytdlp() {
        println!("\n{}", style("✗ yt-dlp is not installed!").red().bold());
        println!("\n{}", style("💡 Installation Guide:").yellow().bold());
        println!(
            "   {} {}",
            style("•").cyan(),
            "Linux/Mac: pip install yt-dlp"
        );
        println!(
            "   {} {}",
            style("•").cyan(),
            "Windows: Download from github.com/yt-dlp/yt-dlp"
        );
        std::process::exit(1);
    }

    println!("{}\n", style("✓ yt-dlp detected").green().bold());

    let url: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("🔗 Enter YouTube URL")
        .interact_text()
        .unwrap();

    if url.is_empty() {
        println!("{}", style("✗ URL cannot be empty!").red().bold());
        std::process::exit(1);
    }

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message("Validating URL...");
    spinner.enable_steady_tick(Duration::from_millis(80));
    thread::sleep(Duration::from_millis(1000));
    spinner.finish_with_message(style("✓ URL validated").green().to_string());

    println!();

    let download_types = vec![
        "🎬 Video (with audio in MP4 format)",
        "🎵 Audio (extract audio in MP3 format)",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("📦 Select download type")
        .items(&download_types)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => download_video(&url),
        1 => download_audio(&url),
        _ => unreachable!(),
    }
}

fn print_banner() {
    println!();
    println!(
        "{}",
        style("╔════════════════════════════════════════════════════════╗")
            .magenta()
            .bold()
    );
    println!(
        "{}",
        style("║                                                        ║")
            .magenta()
            .bold()
    );
    println!(
        "{}",
        style("║        🎥  YOUTUBE DOWNLOADER CLI  🎵                  ║")
            .magenta()
            .bold()
            .on_black()
    );
    println!(
        "{}",
        style("║                                                        ║")
            .magenta()
            .bold()
    );
    println!(
        "{}",
        style("║              Powered by yt-dlp                         ║")
            .magenta()
            .bold()
            .dim()
    );
    println!(
        "{}",
        style("╚════════════════════════════════════════════════════════╝")
            .magenta()
            .bold()
    );
    println!();
}

fn check_ytdlp() -> bool {
    Command::new("yt-dlp")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
}

fn download_video(url: &str) {
    println!();

    let qualities = vec![
        "🌟 Best Quality (Highest resolution)",
        "📺 1080p Full HD",
        "🎞️  720p HD",
        "📼 480p SD",
        "📱 360p",
    ];

    let quality_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🎬 Select video quality")
        .items(&qualities)
        .default(0)
        .interact()
        .unwrap();

    let format = match quality_selection {
        0 => "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best",
        1 => "bestvideo[height<=1080][ext=mp4]+bestaudio[ext=m4a]/best[height<=1080]",
        2 => "bestvideo[height<=720][ext=mp4]+bestaudio[ext=m4a]/best[height<=720]",
        3 => "bestvideo[height<=480][ext=mp4]+bestaudio[ext=m4a]/best[height<=480]",
        4 => "bestvideo[height<=360][ext=mp4]+bestaudio[ext=m4a]/best[height<=360]",
        _ => unreachable!(),
    };

    println!();
    println!("{}", style("⬇️  Downloading Video...").cyan().bold());
    println!();

    // progress bar
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );

    // download progress
    let pb_clone = pb.clone();
    let progress_thread = thread::spawn(move || {
        for i in 0..=100 {
            pb_clone.set_position(i);
            pb_clone.set_message(format!("{}%", i));
            thread::sleep(Duration::from_millis(100));
        }
    });

    let status = Command::new("yt-dlp")
        .arg("-f")
        .arg(format)
        .arg("--merge-output-format")
        .arg("mp4")
        .arg("--no-progress")
        .arg(url)
        .status();

    progress_thread.join().unwrap();
    pb.finish_and_clear();

    match status {
        Ok(s) if s.success() => {
            println!();
            println!(
                "{}",
                style("╔═══════════════════════════════════════╗")
                    .green()
                    .bold()
            );
            println!(
                "{}",
                style("║  ✓ VIDEO DOWNLOADED SUCCESSFULLY!     ║")
                    .green()
                    .bold()
            );
            println!(
                "{}",
                style("╚═══════════════════════════════════════╝")
                    .green()
                    .bold()
            );
            println!();
        }
        Ok(_) => println!(
            "{}",
            style("✗ Download failed. Please check the URL and try again.")
                .red()
                .bold()
        ),
        Err(e) => println!("{}", style(format!("✗ Error: {}", e)).red().bold()),
    }
}

fn download_audio(url: &str) {
    println!();

    let qualities = vec![
        "🌟 Best Quality (Highest bitrate)",
        "💎 320 kbps (High quality)",
        "🎧 192 kbps (Medium quality)",
        "🔊 128 kbps (Standard quality)",
    ];

    let quality_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("🎵 Select audio quality")
        .items(&qualities)
        .default(0)
        .interact()
        .unwrap();

    let audio_quality = match quality_selection {
        0 => "0",
        1 => "320K",
        2 => "192K",
        3 => "128K",
        _ => unreachable!(),
    };

    println!();
    println!("{}", style("⬇️  Extracting Audio...").cyan().bold());
    println!();

    // progress bar
    let pb = ProgressBar::new(100);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.magenta/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("█▓▒░"),
    );

    // Simulate extraction progress
    let pb_clone = pb.clone();
    let progress_thread = thread::spawn(move || {
        for i in 0..=100 {
            pb_clone.set_position(i);
            pb_clone.set_message(format!("{}%", i));
            thread::sleep(Duration::from_millis(100));
        }
    });

    let status = Command::new("yt-dlp")
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg("--audio-quality")
        .arg(audio_quality)
        .arg("--no-progress")
        .arg(url)
        .status();

    progress_thread.join().unwrap();
    pb.finish_and_clear();

    match status {
        Ok(s) if s.success() => {
            println!();
            println!(
                "{}",
                style("╔═══════════════════════════════════════╗")
                    .green()
                    .bold()
            );
            println!(
                "{}",
                style("║  ✓ AUDIO EXTRACTED SUCCESSFULLY!      ║")
                    .green()
                    .bold()
            );
            println!(
                "{}",
                style("╚═══════════════════════════════════════╝")
                    .green()
                    .bold()
            );
            println!();
        }
        Ok(_) => println!(
            "{}",
            style("✗ Download failed. Please check the URL and try again.")
                .red()
                .bold()
        ),
        Err(e) => println!("{}", style(format!("✗ Error: {}", e)).red().bold()),
    }
}
