//! calculating source logic
use std::convert::TryFrom;
// check file extension
use super::errors::StatsError;
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct SrcStats {
    // number of files
    pub files: u32,
    // number of lines of code
    pub code: u32,
    // number of comments
    pub comments: u32,
    // number of blanks
    pub blanks: u32,
}

pub fn get_stats_for_file(file_name: &Path) -> Result<SrcStats, StatsError> {
    let file_contents = fs::read_to_string(file_name)?;
    let mut total_codes = 0;
    let mut total_comments = 0;
    let mut total_blanks = 0;

    for line in file_contents.lines() {
        if line.len() == 0 {
            total_blanks += 1;
        } else if line.trim_start().starts_with("//")
            || line.trim_start().starts_with("///")
            || line.trim_start().starts_with("//!")
        {
            total_comments += 1;
        } else {
            total_codes += 1;
        }
    }

    Ok(SrcStats {
        files: u32::try_from(file_contents.lines().count())?,
        code: total_codes,
        comments: total_comments,
        blanks: total_blanks,
    })
}

pub fn get_stats_for_dir(in_dir: &Path) -> Result<SrcStats, StatsError> {
    let mut total_codes = 0;
    let mut total_comments = 0;
    let mut total_blanks = 0;
    let mut dir_entries: Vec<PathBuf> = vec![in_dir.to_path_buf()];
    let mut file_entries: Vec<DirEntry> = vec![];

    while let Some(entry) = dir_entries.pop() {
        for inner_entry in fs::read_dir(&entry)? {
            if let Ok(entry) = inner_entry {
                if entry.path().is_dir() {
                    dir_entries.push(entry.path());
                } else {
                    file_entries.push(entry);
                }
            }
        }
    }

    let file_count = file_entries.len();

    for entry in file_entries {
        let stat = get_stats_for_file(&entry.path()).unwrap();
        total_codes += stat.code;
        total_comments = stat.comments;
        total_blanks = stat.blanks;
    }

    Ok(SrcStats {
        files: u32::try_from(file_count)?,
        code: total_codes,
        comments: total_comments,
        blanks: total_blanks,
    })
}
