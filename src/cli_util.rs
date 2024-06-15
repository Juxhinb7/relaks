use clap::Command;
use colored::Colorize;

pub fn cli(logo: &str) -> Command {
    let description: Vec<String> = vec![
    format!("{}\n", "Note! This project is still work in progress."),
    format!("\n{}\n", "Introduction".underline().bold()),
    format!("{}\n", "Introducing relaks, a simple and elegant command-line application designed for playing your favorite LoFi music."),
    format!("{}\n\n", "Whether you're studying, working, or just relaxing, this music player provides a seamless audio experience directly from your terminal."),
    format!("{}\n", "Technical details".underline().bold()),
    format!("{} {}\n", colored::Colorize::bold("Built with Rust:"), "Ensuring performance, safety and reliability."),
    format!("{} {}\n", colored::Colorize::bold("Modern UI:"), "Uses the 'ratatui' and 'inquire' crates to ensure a modern ui feel for the terminal application."), 
    format!("{} {}\n", colored::Colorize::bold("Audio Backend:"), "Uses the 'rodio' crate for high-quality audio playback."),
    format!("{} {}", colored::Colorize::bold("Lightweight:"), "Minimal dependencies for a fast and responsive experience."),
];

    Command::new("relaks")
        .about(colored::ColoredString::magenta(logo.into()).to_string() + &description[0..].join(""))
        .subcommand(
            Command::new("menu").about("Opens the menu")
        )
}