use std::time::Instant;
use std::{collections::BTreeMap, error::Error, fmt::Display, path::Path};

use anyhow::Context;
use count_digits::CountDigits;
use tokio::{
    fs::{File, read_dir},
    io::AsyncReadExt,
};

const DATADIR_NAME: &str = "matchdata";
use crate::constants::ERR_PFX;

type JsonDirectoryName = String;
type JsonFileName = String;
type JsonFileContents = String;

pub type JsonFileContentsRaw = BTreeMap<JsonFileName, JsonFileContents>;
pub type JsonFilesContentsAllRaw = BTreeMap<JsonDirectoryName, JsonFileContentsRaw>;

const MOD: &str = "JSON_FETCHER";

// Custom errors that correspond to edge cases of json fetcher
#[derive(Debug)]
pub enum JsonFetcherError {
    DatadirNotFound,
    NoFilesFound,
}
impl Display for JsonFetcherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use JsonFetcherError::*;
        match self {
            DatadirNotFound => write!(f, "Main data directory not found."),
            NoFilesFound => write!(
                f,
                "No files with usable data found in subdirectories of main data directory."
            ),
        }
    }
}
impl Error for JsonFetcherError {}

pub async fn fetch_json_raw_data() -> anyhow::Result<JsonFilesContentsAllRaw> {
    const ERR_FN: &str = "::fetch_json_raw_data";
    let mut directory_count = 0;
    let mut file_count = 0;

    println!("{MOD}: fetching json files.");
    let now = Instant::now();

    let datadir = Path::new(DATADIR_NAME);
    let mut raw_file_contents: JsonFilesContentsAllRaw = BTreeMap::new();

    if !datadir.is_dir() {
        return Err(JsonFetcherError::DatadirNotFound.into());
    } else {
        let mut dir_contents = read_dir(datadir).await.with_context(|| {
            format!(
                "{ERR_PFX} {MOD}{ERR_FN}: Failed to fetch main data directory '{DATADIR_NAME}' contents."
            )
        })?;

        while let Some(sub) = dir_contents.next_entry().await.with_context(|| {
            format!("{ERR_PFX} {MOD}{ERR_FN}: Could not read the name of subdirectory.")
        })? {
            // Ignore the error condition for file type reading
            // We'll use what json file we have available
            // Also ignore LICENSE.md file with `is_dir()`
            if let Ok(ftype) = sub.file_type().await
                && ftype.is_dir()
            {
                let dir_path = sub.path();
                let dir_name = sub.file_name().into_string().unwrap();

                // Subdir name must be in the format for example: 2017-18 or 2017
                // Because this will later be treated as categorization data
                // We print a soft error to the console and carry-on ignoring this
                // offending directory.
                if !validate_subdir_name(&dir_name) {
                    eprintln!(
                        "{ERR_PFX} {MOD}{ERR_FN}: Could not validate sub directory name '{}'. Continuing without it... The name should be in the format 2017-18 or 2017.",
                        dir_name
                    );
                    // Skip this directory
                    continue;
                }

                let subdir_files = read_dir(&dir_path).await;

                // Ignore the error condition at this point
                // We'll use what we have
                if let Ok(mut subdir_files) = subdir_files {
                    raw_file_contents.insert(dir_name.clone(), BTreeMap::new());

                    // We again ignore the error condition to give other files a chance
                    // If no files are found, then we can "throw" a custom error `NoFilesFound` back to the caller
                    // That confition is checked later and separately.
                    while let Ok(Some(json_file)) = subdir_files.next_entry().await {
                        let file_name = json_file.file_name().into_string().unwrap();
                        let map = raw_file_contents.get_mut(&dir_name).unwrap();
                        let fcontents_result = fetch_file_contents(&json_file.path()).await;

                        // If file contents couldn't be read,
                        // We'll print a soft error log only.
                        // Because other files might be ok.
                        match fcontents_result {
                            Ok(file_contents) => {
                                map.insert(file_name.clone(), file_contents);
                                file_count += 1;
                            }
                            Err(err) => {
                                eprintln!(
                                    "{ERR_PFX} {MOD}{ERR_FN}: Can't read contents of file {}: {}. Continuing without it...",
                                    &file_name, err
                                );
                            }
                        }
                    }

                    // If no files found in the subdir to be of use,
                    // We delete the entry to keep data clean
                    let fetched_contents = raw_file_contents.get_mut(&dir_name).unwrap();
                    if fetched_contents.is_empty() {
                        raw_file_contents.remove(&dir_name);
                    } else {
                        directory_count += 1;
                    }
                }
            }
        }
    }

    // If we have no data to return, something is wrong.
    // Signalling the caller to stop progress at this point
    // and spit an error is a good idea.
    if raw_file_contents.is_empty() {
        return Err(JsonFetcherError::NoFilesFound.into());
    }

    println!(
        "{MOD}: json fetch ended with elapsed milliseconds: {}",
        now.elapsed().as_millis()
    );

    println!(
        "{MOD}: contents of {} json files from {} directories converted to strings.",
        file_count, directory_count
    );

    Ok(raw_file_contents)
}

// Subdir name must be in the format for example: 2017-18 or 2017
// Because this will later be treated as categorization data
fn validate_subdir_name(subdir_name: &str) -> bool {
    let split = subdir_name.split("-");

    split.enumerate().all(|(index, item)| {
        let parsed = item.parse::<u16>();
        parsed.is_ok_and(|num| {
            (index == 0 && num.count_digits() == 4) || (index == 1 && num.count_digits() == 2)
        })
    })
}

async fn fetch_file_contents(path: &Path) -> anyhow::Result<String> {
    const ERR_FN: &str = "::fetch_file_contents";

    let mut file_handle = File::open(path).await.with_context(|| {
        format!(
            "{ERR_PFX} {MOD}{ERR_FN}: File open failed with path: {}",
            path.to_str().unwrap()
        )
    })?;

    let mut contents = String::new();
    file_handle
        .read_to_string(&mut contents)
        .await
        .with_context(|| {
            format!(
                "{ERR_PFX} {MOD}{ERR_FN}: File contents could not be read: {}",
                path.to_str().unwrap()
            )
        })?;

    Ok(contents)
}
