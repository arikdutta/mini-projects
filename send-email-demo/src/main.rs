//! resend-demo: a tiny CLI that sends email through Resend.
//!
//! Usage:
//!   export RESEND_API_KEY=re_xxx        (or put it in a .env file)
//!   cargo run -- send --to you@example.com
//!   cargo run -- send --to you@example.com --attach ./invoice.pdf

use clap::{Parser, Subcommand};
use resend_rs::types::{CreateAttachment, CreateEmailBaseOptions};
use resend_rs::Resend;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "send-email-demo", about = "Send emails with Resend from Rust")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Send a demo email (optionally with a file attached)
    Send {
        /// Recipient address
        #[arg(long)]
        to: String,

        /// Sender address (use onboarding@resend.dev for testing)
        #[arg(long, default_value = "Test App <test@mail.rustfinance.com>")]
        from: String,

        /// Subject line
        #[arg(long, default_value = "Hello from Rust + Resend")]
        subject: String,

        /// Optional path to a file to attach
        #[arg(long)]
        attach: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Load RESEND_API_KEY from a .env file if present (real env vars win).
    let _ = dotenvy::dotenv();

    if std::env::var("RESEND_API_KEY").is_err() {
        eprintln!("error: RESEND_API_KEY is not set.");
        eprintln!("       Copy .env.example to .env and paste your key, or run:");
        eprintln!("       export RESEND_API_KEY=re_your_key_here");
        std::process::exit(1);
    }

    let resend = Resend::default(); // picks up RESEND_API_KEY

    match cli.command {
        Command::Send {
            to,
            from,
            subject,
            attach,
        } => {
            let html = format!(
                "<h1>It works! 🎉</h1>\
                 <p>This email was sent from a Rust program using the \
                 <a href=\"https://resend.com\">Resend</a> API.</p>\
                 <p><em>Subject:</em> {subject}</p>"
            );
            let text = format!(
                "It works!\n\nThis email was sent from a Rust program \
                 using the Resend API.\n\nSubject: {subject}"
            );

            let mut email = CreateEmailBaseOptions::new(&from, vec![to.clone()], &subject)
                .with_html(&html)
                .with_text(&text); // plain-text fallback

            if let Some(path) = attach {
                let bytes = std::fs::read(&path)?;
                let filename = path
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
                    .unwrap_or_else(|| "attachment".to_string());
                println!("Attaching {filename} ({} bytes)...", bytes.len());

                let attachment = CreateAttachment::from_content(bytes).with_filename(&filename);
                email = email.with_attachment(attachment);
            }

            println!("Sending to {to}...");
            match resend.emails.send(email).await {
                Ok(sent) => println!("✅ Sent! Email ID: {}", sent.id),
                Err(e) => {
                    eprintln!("❌ Failed to send: {e}");
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
