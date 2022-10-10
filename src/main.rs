use std::fs::File;
use std::fs;
use cider::parsing::*;
use std::io::prelude::*;
use cider::executor::*;
use simplelog::*;

fn main() -> std::io::Result<()> {
    setup_logger().unwrap_or_else(|err| {
        panic!("Logs could not be properly set up due to the following error:\n{}", err);
    });
    // let conf = ShareableConfiguration::new(None).unwrap();
    // println!("{:#?}", conf);
    // let conf = conf;
    let conf = Parser::new_top_level("example_docker_config.json");
    // println!("{:#?}", curate_filepath(conf.get_shared_config().get_output(), Some("main_test")));
    let mut file = File::create(curate_filepath(conf.get_shared_config().get_output(), "main_test.txt"))?;
    file.write_fmt(format_args!("{:#?}", executor::exec_actions(&conf.get_all_actions())))?;
    let mut file = File::create("./dist/output/config_output.txt")?;
    file.write_fmt(format_args!("{:#?}", conf))?;
    // conf.print_filename();
    // let conf = Parser::parse_file(conf, "example_docker_config.json".to_string()).unwrap_or_else(|err| {
    //     eprintln!("There was an error finding the input file: {}", err);
    //     process::exit(1);
    // });
    // conf.print_filename();
    // conf.parse_file("somethingelse.txt");
    Ok(())
}

/**
 * Sets up a logger to be used by the program. This will have more functionality in the future
 * /*!TODO: Allow multiple verbosity options. */
 * /*!TODO: Allow for custom file pathing for logs. */
 */
fn setup_logger() -> std::io::Result<()> {
    fs::create_dir_all("dist/logs")?;
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::max(), Config::default(), File::create(curate_filepath("dist/logs/","verbose_runtime_log.txt")).unwrap()),
            WriteLogger::new(LevelFilter::Trace, Config::default(), File::create(curate_filepath("dist/logs/","trace_runtime_log.txt")).unwrap()),
            WriteLogger::new(LevelFilter::Error, Config::default(), File::create(curate_filepath("dist/logs/","error_runtime_log.txt")).unwrap()),
            WriteLogger::new(LevelFilter::Warn, Config::default(), File::create(curate_filepath("dist/logs/","warn_runtime_log.txt")).unwrap()),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create(curate_filepath("dist/logs/","info_runtime_log.txt")).unwrap())
        ]
    ).unwrap();
    Ok(())
}

fn curate_filepath(path: &str, filename: &str) -> String{
    let filepath = {
        if !path.is_empty() {
            if cfg!(windows)
            {
                if !path.chars().nth(path.len()-1).unwrap().eq(&'/') {
                    path.to_string() + "\\"
                } else {
                    path.to_string()
                }
            }
            else if !path.chars().nth(path.len()-1).unwrap().eq(&'/') {
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
        if cfg!(windows){
            assert_eq!("test\\log1.txt".to_owned(), curate_filepath("test", "log1.txt"));
        } else {
            assert_eq!("test/log1.txt".to_owned(), curate_filepath("test", "log1.txt"));
        }
    }

    ///This test intends to ensure that proper filepath endings are implemented on the proper operating systems.
    /**For example, test/ on linux should not become test*/
    #[test]
    fn test_filepath_overcuration() {
        if cfg!(windows){
            assert_eq!("test\\log1.txt".to_owned(), curate_filepath("test", "log1.txt"));
        } else {
            assert_eq!("test/log1.txt".to_owned(), curate_filepath("test", "log1.txt"));
        }
    }
}