use std::{collections::HashSet, path::PathBuf};

use clap::Parser;

/// Generate all-file name similarity
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start point for file-name locating
    root: Vec<String>,

    /// Minimum similarity to consider a match
    #[arg(short, long, default_value_t = 0.6)]
    threshold: f32,

    /// Reverse display direction of results
    #[arg(short, long, default_value_t = false)]
    reverse: bool,

    /// File-names must match this pattern
    #[arg(short, long, default_value_t = String::from(".*"))]
    filename_pattern: String,
}

fn scan_dir(root: &str, pattern: &str) -> Vec<(PathBuf, u64, HashSet<String>)> {
    let filename_regex = regex::Regex::new(pattern).unwrap();
    log::info!("Getting file listing from: {root}");
    walkdir::WalkDir::new(root)
        .into_iter()
        .flatten()
        .fold(Vec::new(), |mut acc, entry| {
            if let Ok(meta) = entry.metadata() {
                if meta.is_file() && filename_regex.is_match(&entry.file_name().to_string_lossy()) {
                    let parts: HashSet<String> = entry
                        .file_name()
                        .to_string_lossy()
                        .split(|c: char| !c.is_alphanumeric())
                        .map(|v| v.to_lowercase())
                        .collect();
                    acc.push((entry.path().to_owned(), entry.metadata().map(|m| m.len()).unwrap_or_default(), parts));
                }
            }
            acc
        })
}

fn calculate_duplicates(
    entries: &[(PathBuf, u64, HashSet<String>)],
    threshold: f32,
) -> Vec<(f32, &PathBuf, &PathBuf, u64)> {
    log::info!("Generating similarity between {} entries", entries.len());

    let mut duplicates = Vec::new();

    for (i, (path_a, size_a, words_a)) in entries.iter().enumerate() {
        let l1_sum = words_a.len();
        for j in 0..i {
            let (path_b, size_b, words_b) = entries.get(j).unwrap();

            let ab_words: HashSet<_> = words_a.intersection(words_b).collect();

            let l2_sum = words_b.len();
            let c = ab_words.len();

            let cosine = (c as f32) / ((l1_sum * l2_sum) as f32).sqrt();

            if cosine > threshold {
                log::debug!("{path_a:?} {path_b:?} {cosine}");
                duplicates.push((cosine, path_a, path_b, size_a + size_b));
            }
        }
    }

    duplicates
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let args = Args::parse();

    let entries = args
        .root
        .iter()
        .flat_map(|r| scan_dir(r, &args.filename_pattern).into_iter())
        .collect::<Vec<_>>();

    let mut duplicates = calculate_duplicates(&entries, args.threshold);

    duplicates.sort_by_key(|v| v.3);
    duplicates.sort_by_key(|v| (v.0 * 10000.0) as u64);

    if args.reverse {
        duplicates.reverse();
    }

    log::debug!("{duplicates:#?}");
    for (score, path_a, path_b, total) in &duplicates {
        log::info!("{score} {total} {path_a:?} {path_b:?}");
    }
    log::info!("total duplicates: {}", duplicates.len());
}
