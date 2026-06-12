# resend-demo

A mini Rust CLI that sends email through the [Resend](https://resend.com) API.
Demonstrates HTML + plain-text emails and file attachments using the official
`resend-rs` SDK.

## Prerequisites

- Rust 1.85+ (edition 2024 support; install via [rustup](https://rustup.rs))
- A free Resend account and API key from <https://resend.com/api-keys>

## Setup

```bash
cp .env.example .env
# edit .env and paste your key:  RESEND_API_KEY=re_...
```

(Or just `export RESEND_API_KEY=re_...` in your shell.)

## Usage

Send a demo email (on the free tier without a verified domain, the recipient
must be the email address you signed up with):

```bash
cargo run -- send --to you@example.com
```

With a custom subject and an attachment:

```bash
cargo run -- send --to you@example.com \
    --subject "Quarterly report" \
    --attach ./report.pdf
```

With your own verified domain as the sender:

```bash
cargo run -- send --to anyone@example.com --from "Me <hello@yourdomain.com>"
```

On success it prints the Resend email ID, which you can look up in the
dashboard under **Emails** to see delivery status.

## Project structure

```
resend-demo/
├── Cargo.toml        # resend-rs, tokio, clap, dotenvy
├── .env.example      # template for your API key (never commit .env)
└── src/main.rs       # the CLI
```

## Notes

- Every email includes both HTML and a plain-text fallback (good for
  deliverability and accessibility).
- Attachments are read from disk and passed as raw bytes; the SDK handles
  encoding. Total email size limit is 40 MB.
- To email arbitrary recipients, verify a domain in the Resend dashboard
  (Domains → Add Domain → set the DNS records).
