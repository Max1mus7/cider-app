pub mod utils;

//package imports
use cider::executor::*;
use cider::parsing::*;

//arg parser
use clap::Parser;

use log::debug;
use log::warn;
//logger
use log::{info, error};
use simplelog::*;

//std library imports
use std::collections::HashMap;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;
use std::time::UNIX_EPOCH;
use std::{thread, time};

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    config: Option<String>,
    #[arg(short, long, default_value_t = false)]
    watch: bool,
    #[arg(short, long, default_value_t = String::from("Warn"))]
    loglevel: String

}

fn main() -> std::io::Result<()> {
    let args = Arguments::parse();
    let filename = if args.config.is_none() {
        "cider_config.json".to_string()
    } else {
        args.config.unwrap()
    };
    setup_logger(args.loglevel).unwrap_or_else(|err| {
        panic!(
            "Logs could not be properly set up due to the following error:\n{}",
            err
        );
    });



    let conf = json_parser::new_top_level(&filename);
    let mut output_file = File::create(curate_filepath(
        conf.s_config.get_output(),
        "cider_output.txt",
    ))?;

    let source_dir = Path::new(conf.s_config.get_source());

    if args.watch {
        let mut elapsed_times = HashMap::<OsString, Duration>::new();
        let mut recent_file_changed = get_least_time(&elapsed_times);
        loop {
            get_files_time_elapsed_since_changed(&mut elapsed_times, source_dir, &conf.s_config.get_ignore_dirs())?;
            let checked_time = get_least_time(&elapsed_times);
            if checked_time < recent_file_changed {
                let conf = json_parser::new_top_level(&filename);
                recent_file_changed = checked_time;
                output_file
                    .write_fmt(format_args!("{:#?}", exec_actions(&conf.get_all_actions())))?;
            } else {
                recent_file_changed = checked_time;
                debug!(
                    "File in watched directory most recently changed {:#?} ago.",
                    recent_file_changed
                );
                // println!("Waiting for changes to be made to source directory.");
            }
            thread::sleep(time::Duration::from_millis(2000));
        }
    } else {
        output_file.write_fmt(format_args!("{:#?}", exec_actions(&conf.get_all_actions())))?;
    }

    let mut file = File::create("./dist/output/config_output.txt")?;
    file.write_fmt(format_args!("{:#?}", conf))?;

    Ok(())
}

fn get_least_time(elapsed_times: &HashMap<OsString, Duration>) -> Duration {
    let mut least_time = UNIX_EPOCH.elapsed().unwrap();
    for entry in elapsed_times {
        if entry.1 < &least_time {
            least_time = *entry.1;
            debug!("The file with the newest changes is {:#?} with the last change {:#?} ago",entry.0, entry.1);
        }
    }
    debug!(
        "Most recent time in a which a file was changed: {:#?}",
        least_time
    );
    least_time
}

fn get_files_time_elapsed_since_changed<'a>(
    elapsed_times: &'a mut HashMap<OsString, Duration>,
    path: &'a Path,
    ignore_dirs: & Option<Vec<String>>
) -> std::io::Result<()> {
    info!("Getting elapsed time for files within {:#?}", path);
    for entry in fs::read_dir(path)? {
        if Path::new(&entry.as_ref().unwrap().file_name()).extension().and_then(OsStr::to_str) == Some("class") || entry.as_ref().unwrap().file_name() == "package-lock.json" {
            continue;
        }
        if !elapsed_times.contains_key(&entry.as_ref().unwrap().file_name()) {
            elapsed_times.insert(
                entry.as_ref().unwrap().file_name().to_os_string().clone(),
                entry
                    .as_ref()
                    .unwrap()
                    .metadata()?
                    .modified()?
                    .elapsed()
                    .unwrap(),
            );
        } else {
            elapsed_times.insert(
                entry.as_ref().unwrap().file_name().clone(),
                entry
                    .as_ref()
                    .unwrap()
                    .metadata()?
                    .modified()?
                    .elapsed()
                    .unwrap(),
            );
        }
        if entry.as_ref().unwrap().metadata()?.is_dir() && match ignore_dirs {
            Some(ignore_dirs) => !ignore_dirs.contains(&String::from(&entry.as_ref().unwrap().path().as_os_str().to_str().unwrap().to_owned())),
            None => {
                panic!("ignore_dirs not set properly. This should have a default value, but this is not getting set. Currently set to: {:#?}. Check debug logs for more info.", ignore_dirs);
            }
        }
        {
            get_files_time_elapsed_since_changed(
                elapsed_times,
                entry.as_ref().unwrap().path().as_path(),
                ignore_dirs
            )
            .unwrap_or_else(|err| {
                warn!("Error: {:#?}", err);
                warn!("Failed to find directory {:#?} on filesystem. Please only use paths that exist.", entry.as_ref().unwrap().file_name())
            });
        }
    }
    debug!("Times since last directory modification: {:#?}", elapsed_times.clone());
    Ok(())
}

/**
 * Sets up a logger to be used by the program. This will have more functionality in the future
 * /*!TODO: Allow multiple verbosity options to be input by users. */
 * /*!TODO: Allow for custom file pathing for logs. */
 */
fn setup_logger(term_log_level: String) -> std::io::Result<()> {
    fs::create_dir_all("dist/logs")?;
    fs::create_dir_all("dist/cider")?;
    fs::create_dir_all("dist/output")?;
    fs::create_dir_all("metrics/win")?;
    fs::create_dir_all("metrics/combined_reports")?;
    // fs::create_dir_all("metrics/deb")?;
    // fs::create_dir_all("metrics/rhel")?;
    let term_log_level_filter = {
        match term_log_level.as_str() {
            "Warn"  | "warn"  | "WARN"  => LevelFilter::Warn,
            "Info"  | "info"  | "INFO"  => LevelFilter::Info,
            "Debug" | "debug" | "DEBUG" => LevelFilter::Debug,
            "Trace" | "trace" | "TRACE" => LevelFilter::Trace,
            "Off"   | "off"   | "OFF"   => LevelFilter::Off,
            &_ => {
                error!("{} {} {}", "Failed to parse log level.", term_log_level, "provided. Please select warn, info, debug, trace, or off.");
                panic!("Please choose an appropriate log level.");
            }
        }
    };

    CombinedLogger::init(vec![
        TermLogger::new(
            term_log_level_filter,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::max(),
            Config::default(),
            File::create(curate_filepath("dist/logs/", "verbose_runtime_log.txt")).unwrap(),
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create(curate_filepath("dist/logs/", "trace_runtime_log.txt")).unwrap(),
        ),
        WriteLogger::new(
            LevelFilter::Error,
            Config::default(),
            File::create(curate_filepath("dist/logs/", "error_runtime_log.txt")).unwrap(),
        ),
        WriteLogger::new(
            LevelFilter::Warn,
            Config::default(),
            File::create(curate_filepath("dist/logs/", "warn_runtime_log.txt")).unwrap(),
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(curate_filepath("dist/logs/", "info_runtime_log.txt")).unwrap(),
        ),
    ])
    .unwrap();
    Ok(())
}

/**
 * Curates filepaths to properly be able to link to files in a user-friendly way
 * Example: path/nested_dir -> path/nested_dir/
 */
fn curate_filepath(path: &str, filename: &str) -> String {
    let filepath = {
        if !path.is_empty() {
            if cfg!(windows) {
                if !path.chars().nth(path.len() - 1).unwrap().eq(&'\\') {
                    path.to_string() + "\\"
                } else {
                    path.to_string()
                }
            } else if !path.chars().nth(path.len() - 1).unwrap().eq(&'/') {
                path.to_string() + "/"
            } else {
                path.to_string()
            }
        } else {
            panic!("No path provided provided.");
        }
    };
    {
        if !filename.is_empty() {
            filepath + filename
        } else {
            filepath + "default_output.txt"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filepath_curation() {
        if cfg!(windows) {
            assert_eq!(
                "test\\log1.txt".to_owned(),
                curate_filepath("test", "log1.txt")
            );
        } else {
            assert_eq!(
                "test/log1.txt".to_owned(),
                curate_filepath("test", "log1.txt")
            );
        }
    }

    ///This test intends to ensure that proper filepath endings are implemented on the proper operating systems.
    /**For example, test/ on linux should not become test*/
    #[test]
    fn test_filepath_overcuration() {
        if cfg!(windows) {
            assert_eq!(
                "test\\log1.txt".to_owned(),
                curate_filepath("test\\", "log1.txt")
            );
        } else {
            assert_eq!(
                "test/log1.txt".to_owned(),
                curate_filepath("test/", "log1.txt")
            );
        }
    }
}
