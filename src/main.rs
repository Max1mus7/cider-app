pub mod utils;

use cider::executor::*;
use cider::parsing::*;

use clap::Parser;

use log::info;
use simplelog::*;

use std::collections::HashMap;
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
}

fn main() -> std::io::Result<()> {
    setup_logger().unwrap_or_else(|err| {
        panic!(
            "Logs could not be properly set up due to the following error:\n{}",
            err
        );
    });

    let args = Arguments::parse();

    let filename = if args.config.is_none() {
        "cider_config.json".to_string()
    } else {
        args.config.unwrap()
    };

    let conf = JsonParser::new_top_level(&filename);
    let mut file = File::create(curate_filepath(
        conf.get_shared_config().get_output(),
        "main_test.txt",
    ))?;
    
    let mut elapsed_times = HashMap::<OsString, Duration>::new();

    let source_dir = Path::new(conf.get_shared_config().get_source());
    get_files_elapsed(&mut elapsed_times, source_dir)?;

    let mut recent_file_changed = get_least_time(&elapsed_times);

    println!("{:#?}", args.watch);
    loop {
        if args.watch {
            thread::sleep(time::Duration::from_millis(2000));
            get_files_elapsed(&mut elapsed_times, source_dir)?;
            let checked_time = get_least_time(&elapsed_times);
            if checked_time < recent_file_changed {
                recent_file_changed = checked_time;
                println!("Changes detected in source directory.");
                file.write_fmt(format_args!("{:#?}", exec_actions(&conf.get_all_actions())))?;
            }
            else {
                recent_file_changed = checked_time;
                info!("File changed {:#?} ago.", recent_file_changed);
                println!("Waiting for changes to be made to source directory.");
            }
        } else {
            file.write_fmt(format_args!("{:#?}", exec_actions(&conf.get_all_actions())))?;
            println!("test");
            break;
        }
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
        }
    }
    info!("Most recent time in a which a file was changed: {:#?}",least_time);
    least_time
}

fn get_files_elapsed<'a>(mut elapsed_times: &'a mut HashMap<OsString, Duration>, path: &'a Path) -> std::io::Result<()> {
    info!("Getting elapsed time for files within {:#?}", path);
    for entry in fs::read_dir(path)? {
        if !elapsed_times.contains_key(&entry.as_ref().unwrap().file_name()){
            elapsed_times.insert(entry.as_ref().unwrap().file_name().to_os_string().clone(), entry.as_ref().unwrap().metadata()?.modified()?.elapsed().unwrap());
        } else { 
            elapsed_times.insert(entry.as_ref().unwrap().file_name().clone(), entry.as_ref().unwrap().metadata()?.modified()?.elapsed().unwrap());
        }
        if entry.as_ref().unwrap().metadata()?.is_dir() {
            get_files_elapsed(&mut elapsed_times, entry.as_ref().unwrap().path().as_path()).unwrap();
        }
    }
    info!("Recursive directory info: {:#?}", elapsed_times.clone());
    Ok(())
}

/**
 * Sets up a logger to be used by the program. This will have more functionality in the future
 * /*!TODO: Allow multiple verbosity options. */
 * /*!TODO: Allow for custom file pathing for logs. */
 */
fn setup_logger() -> std::io::Result<()> {
    fs::create_dir_all("dist/logs")?;
    fs::create_dir_all("dist/cider")?;
    fs::create_dir_all("dist/output")?;
    fs::create_dir_all("metrics/win").unwrap();
    fs::create_dir_all("metrics/deb").unwrap();
    fs::create_dir_all("metrics/rhel").unwrap();

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
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
