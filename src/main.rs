use anyhow::Result as HowResult;
use camino::Utf8PathBuf as PathBuf;
use clap::Parser;
use std::io::{ Write, self };
use tokio::sync::mpsc;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[arg(short = 'p', long = "pattern")]
    pattern: String,

    /// The path to the file to read
    #[arg(short = 'f', long = "file")]
    path: Vec<PathBuf>,
}

#[tokio::main]
async fn main() -> HowResult<()> {
    tracing_subscriber::fmt::init();

    let args = Cli::parse();
    let mut stdout = io::BufWriter::new(io::stdout());
    let paths = args.path;
    let is_mutltiple_paths = paths.len() > 1;
    let (tx, mut rx) = mpsc::channel(64);

    tokio::spawn(async move {
        for path in &paths {
            let matches = clapbasics::find_matches(path, &args.pattern).await;

            tx.send(matches).await.unwrap();
        }
    });

    while let Some(res) = rx.recv().await {
        let matches = res?;

        if is_mutltiple_paths {
            writeln!(stdout, "{}:", matches.path)?;
        }

        for line in matches.lines {
            writeln!(stdout, "{}", line)?;
        }

        if is_mutltiple_paths {
            writeln!(stdout, "")?;
        }
    }

    Ok(())
}
