use anyhow::{Context, Result as HowResult};
use camino::Utf8PathBuf as PathBuf;
use std::fmt;

use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

pub struct Data {
    pub line_number: usize,
    pub content: String,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.line_number, self.content)
    }
}

pub struct Matches {
    pub path: String,
    pub lines: Vec<Data>,
}

impl fmt::Debug for Matches {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} has {} match(es)", self.path, self.lines.len())
    }
}

#[tracing::instrument]
pub async fn find_matches(path: &PathBuf, pattern: &str) -> HowResult<Matches> {
    let path_txt = path.to_string();
    let file = File::open(path).await
        .with_context(|| format!("Error opening {}", &path_txt))?;

    let reader = BufReader::new(file);
    let mut matches = Matches {
        path: path.to_string(),
        lines: Vec::new()
    };

    let mut num = 0;
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await
         .with_context(|| format!("Error reading line {} from {}", num, &path_txt))?
    {
        num += 1;

        if line.contains(pattern) {
            matches.lines.push(Data {
                line_number: num,
                content: line,
            });
        }
    }

    Ok(matches)
}

