use std::fs::File;
use cider::parsing::*;
use std::io::prelude::*;
use cider::executor::*;
use simplelog::*;

fn main() -> std::io::Result<()> {

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::max(), Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create(curate_filepath("dist/logs/","runtime_log.txt")).unwrap()),
        ]
    ).unwrap();
    // let conf = ShareableConfiguration::new(None).unwrap();
    // println!("{:#?}", conf);
    // let conf = conf;
    let conf = Parser::new_top_level("example_docker_config.json");
    // println!("{:#?}", curate_filepath(conf.get_shared_config().get_output(), Some("main_test")));
    let mut file = File::create(curate_filepath(conf.get_shared_config().get_output(), "main_test.txt"))?;
    file.write_fmt(format_args!("{:#?}", executor::exec_actions(conf.get_actions())))?;
    let mut file = File::create("./dist/logs/config_output.txt")?;
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

fn curate_filepath(path: &str, filename: &str) -> String{
    let filepath = {
        if !path.is_empty() {
            if !path.chars().nth(path.len()-1).unwrap().eq(&'/') {
                path.to_string() + "\\"
            } else {
                path.to_string()
            }
        } else {
            panic!("No filename provided.");
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