pub mod utils;

use cider::executor::*;
use cider::parsing::*;
// use cider::watcher::*;

use clap::Parser;
use log::error;
use simplelog::*;
use tokio::fs as tfs;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
// use std::path::PathBuf;
use std::time::SystemTime;
use std::{thread, time};

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    config: Option<String>,

    #[arg(short, long, default_value_t = false)]
    watch: bool,
}
#[tokio::main]
async fn main() -> std::io::Result<()> {
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

    let conf = json_parser::new_top_level(&filename);
    let mut file = File::create(curate_filepath(conf.s_config.get_output(), "main_test.txt"))?;

    let mut start_mod_time = tfs::metadata(conf.s_config.get_source())
        .await?
        .modified()
        .unwrap()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    loop {
        if args.watch {
            thread::sleep(time::Duration::from_millis(2000));
            let now_mod_time = tfs::metadata(conf.s_config.get_source())
                .await?
                .modified()
                .unwrap()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            if start_mod_time < now_mod_time {
                start_mod_time = now_mod_time.clone();
                println!("Changes detected in source directory.");
                file.write_fmt(format_args!("{:#?}", exec_actions(&conf.get_all_actions())))?;
            } else {
                error!("{:#?}, {:#?}", start_mod_time, now_mod_time);
                println!("Waiting for changes to be made to source directory.");
            }
        } else {
            file.write_fmt(format_args!("{:#?}", exec_actions(&conf.get_all_actions())))?;
            break;
        }
    }

    error!(
        "{:#?}",
        tfs::metadata(conf.s_config.get_source())
            .await?
            .modified()
            .unwrap()
    );

    let mut file = File::create("./dist/output/config_output.txt")?;
    file.write_fmt(format_args!("{:#?}", conf))?;

    Ok(())
}

/**
 * Sets up a logger to be used by the program. This will have more functionality in the future
 * /*!TODO: Allow multiple verbosity options. */
 * /*!TODO: Allow for custom file pathing for logs. */
 */
fn setup_logger() -> std::io::Result<()> {
    fs::create_dir_all("dist/logs")?;
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
