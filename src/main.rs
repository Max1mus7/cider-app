use std::process;

use cider::config::*;
use cider::parsing::*;

fn main() {
    // let conf = ShareableConfiguration::new(None).unwrap();
    // println!("{:#?}", conf);
    // let conf = conf;
    // let conf = Parser::parse_overwrite(conf, "example_docker_config.json".to_string()).unwrap_or_else(|err| {
    //     eprintln!("There was an error parsing the file: {}", err);
    //     panic!();
    // });
    // println!("{:#?}", conf);
    // conf.print_filename();
    // let conf = Parser::parse_file(conf, "example_docker_config.json".to_string()).unwrap_or_else(|err| {
    //     eprintln!("There was an error finding the input file: {}", err);
    //     process::exit(1);
    // });
    // conf.print_filename();
    // conf.parse_file("somethingelse.txt");
}
