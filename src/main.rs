pub mod utils;

use cider::executor::*;
use cider::parsing::*;
use clap::Parser;
use log::{error, warn};
use simplelog::*;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::exit;

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    config: String,
}

fn main() -> std::io::Result<()> {
    setup_logger().unwrap_or_else(|err| {
        panic!(
            "Logs could not be properly set up due to the following error:\n{}",
            err
        );
    });

    let args: Vec<String> = clean_args(env::args().collect());

    let filename = get_config_file(args).unwrap_or_else(|err| {
        error!("The configuration file was either not specified or does not exist.");
        println!(
            "The configuration file was either not specified or does not exist.\n{}",
            err
        );
        exit(2);
    });

    let conf = JsonParser::new_top_level(&filename);
    let mut file = File::create(curate_filepath(
        conf.get_shared_config().get_output(),
        "main_test.txt",
    ))?;
    file.write_fmt(format_args!("{:#?}", exec_actions(&conf.get_all_actions())))?;
    let mut file = File::create("./dist/output/config_output.txt")?;
    file.write_fmt(format_args!("{:#?}", conf))?;

    Ok(())
}

fn clean_args(args: Vec<String>) -> Vec<String> {
    if args.len() > 1 {
        args[1..].to_vec()
    } else {
        vec![]
    }
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

fn get_config_file(args: Vec<String>) -> Result<String, &'static str> {
    if !args.is_empty() && args[0].contains(".json") {
        Ok(args[0].clone())
    } else if Path::new("config.json").exists() {
        Ok("config.json".to_string())
    } else {
        for i in 0..args.len() {
            if i > (args.len() - 2) {
                return Err(display_help());
            }
            if args[i].eq("-c") {
                warn!("It is better to define the config file as the first argument passed to the application upon invoking it.");
                return Ok(args[i + 1].clone());
            }
        }
        Err(display_help())
    }
}

fn display_help() -> &'static str {
    r#"
    
    The correct usage of this application is as follows:

    cider.exe CONFIG [OPTIONS]
    
    OPTIONS include:

    -c <filename>: Allows you to specify the location of the config file without having it as the first argument.
    "#
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

    #[test]
    fn test_arg_cleaning_with_args() {
        let args = vec![
            "test1".to_string(),
            "test2".to_string(),
            "test3".to_string(),
        ];
        assert_eq!(
            vec!["test2".to_string(), "test3".to_string()],
            clean_args(args)
        );
    }

    //There will always be at least 1 arg.
    #[test]
    fn test_arg_cleaning_without_args() {
        let args = vec!["test1".to_string()];
        assert_eq!(Vec::<String>::new(), clean_args(args));
    }

    #[test]
    fn test_parsing_input_file() {
        let args = clean_args(vec!["test1".to_string(), "test_config.json".to_string()]);
        JsonParser::parse_json_string(&args[0]);
    }
}
