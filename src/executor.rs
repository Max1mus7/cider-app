/**
 * Module used to clean input and execute actions
 * Eventually, this module will also be used to separate pipeline executions and handle conditional logic
 * May also be split into modules on an action/pipeline level in the future
 */
use std::fs::File;
use std::io::Write;
use std::{
    collections::HashMap, 
    env::current_dir,
};
use std::process::Command;
use crate::config::{
    Action, 
    Condition, 
    Step
};
use log::{info, 
    warn,
    error
};
use relative_path::RelativePath;


/// Small wrapper used to gather output of multiple actions and run actions programatically
pub fn exec_actions(action_vec: &Vec<Action>) -> Vec<Vec<String>> {
    let mut all_output = vec![];
    for action in action_vec {
        all_output.push(exec_action(action))
    }
    // println!("All output: {:#?}", &all_output);
    all_output
}

/// Determines how to perform steps defined by an Action
fn exec_action(action: &Action) -> Vec<String> {
    let exec_info = ExecInfo::new(action);
    match exec_info.backend.to_lowercase().as_str() {
        "bash" => {
            run_bash_scripts(exec_info.manual)
        },
        "docker" => {
            run_with_docker(exec_info)
        }
        &_ => {
            panic!("Specified backend not supported");
        }
    }
}

fn generate_dockerfile(info: &ExecInfo) -> File {
    let mut file = File::create("Dockerfile").unwrap_or_else(|_| {
            error!("There was an issue creating a dockerfile for your docker backend.\nMake sure there are no files in your project named \"DOCKERFILE\".");
            panic!("There was an issue regarding your dockerfile. Please check your logs for more information.");
        }
    );
    let mut str = format_args!("FROM {}\r\n", info.image.as_ref().unwrap()).to_string();
    str += "WORKDIR /cider/app\r\n";
    str += "COPY . ./\r\n";
    for step in info.manual.iter() {
        str += format_args!("RUN {}\r\n", step.get_script()).to_string().as_ref();
    }
    
    file.write_fmt(format_args!("{}", str)).unwrap_or_else(|_| {
        error!("There was an issue creating a dockerfile for your docker backend.\nMake sure there are no files in your project named \"DOCKERFILE\".");
        panic!("There was an issue regarding your dockerfile. Please check your logs for more information.");
    });
    
    file
}

fn run_with_docker(setup: ExecInfo) -> Vec<String> {
    let mut setup = setup;
    let mut outputs = vec![];
    if setup.image.is_none() {
        setup.image = Some("alpine:latest".to_string());
        warn!("There was no image detected in a configured action.");
        outputs.push("There was no docker image found to build off of. Using Alpine Linux by default.".to_string());
    } 
    generate_dockerfile(&setup);

    let output = Command::new("cmd").args(["/C", "docker", "pull", &setup.image.unwrap()]).current_dir(current_dir().unwrap()).output().expect("There was an error building your docker environment.");
    let stdout = String::from_utf8(output.stdout).expect("Could not parse command output as a String.");
    let stderr = String::from_utf8(output.stderr).expect("Could not parse command output as a String.");
    outputs.push(if stdout.is_empty() {
        if stderr.is_empty() {
            "No standard output detected. Check to see if it was piped to another file.".to_string()
        } else {
            error!("Standard error from dockerfile creation: {}", stderr);
            stderr
        }
    } else {
        info!("Standard output from step : {}", stdout);
        stdout
    });

    let rmout = Command::new("cmd").args(["/C", "docker", "image", "rm", "-f", "cider-image"]).current_dir(current_dir().unwrap()).output().expect("There was an issue removing an old image.");
    if String::from_utf8(rmout.stderr.clone()).unwrap().ne("") {
        warn!("{}", String::from_utf8(rmout.stderr).unwrap());
    }
    info!("{}", String::from_utf8(rmout.stdout).unwrap());
    let output = Command::new("cmd").args(["/C", "docker", "build", "-t", "cider-image", "."]).current_dir(current_dir().unwrap()).output().expect("There was an error building your docker environment.");
    let stdout = String::from_utf8(output.stdout).expect("Could not parse command output as a String.");
    let stderr = String::from_utf8(output.stderr).expect("Could not parse command output as a String.");
    outputs.push(if stdout.is_empty() {
        if stderr.is_empty() {
            "No standard output detected. Check to see if it was piped to another file.".to_string()
        } else {
            error!("Standard output from dockerfile creation: {}", stderr);
            stderr
        }
    } else {
        info!("Standard output from step : {}", stdout);
        stdout
    });

    // let output = Command::new("cmd").args(["/C", "docker", "run", "--rm", "--name", "cider-container", "-d", "cider-image"]).current_dir(current_dir().unwrap()).output().expect("There was an error building your docker environment.");
    // let stdout = String::from_utf8(output.stdout).expect("Could not parse command output as a String.");
    // let stderr = String::from_utf8(output.stderr).expect("Could not parse command output as a String.");
    // outputs.push(if stdout.is_empty() {
    //     if stderr.is_empty() {
    //         "No standard output detected. Check to see if it was piped to another file.".to_string()
    //     } else {
    //         error!("Standard output from dockerfile creation: {}", stderr);
    //         stderr
    //     }
    // } else {
    //     info!("Standard output from step : {}", stdout);
    //     stdout
    // });

    outputs
}

///Runs bash scripts defined in an Action's Manual
fn run_bash_scripts(manual: Vec<Step>) -> Vec<String> {

    let mut outputs = vec![];
    
    /*
        * Performs action using windows-specific configuration.
        */
    if cfg!(windows) {
        warn!("In order to avoid unexpected behavior, please consider using \"bat\" or \"batch\" backend for windows operating systems.");
        
        for step in manual {
            let mut command = Command::new("cmd");
            command.current_dir("/");
            let output_str = format_args!("Running {}",step.get_name()).to_string();
            info!("{}",output_str);
            println!("{}",output_str);
            outputs.push(output_str);
            let script = step.get_script().to_string();
            println!("{script}");
            let script = clean_script_pathing(&script);
            // println!("{:#?}", &script);
            let output = command.args(
                [vec!["/c"], 
                script.iter().map(String::as_str).collect()
                ].concat()).current_dir(
                    current_dir().unwrap()
                ).output().expect(
                    &("Failed to execute: ".to_string() + &script.concat())
            );
            let stdout = String::from_utf8(output.stdout).expect("Could not parse command output as a String.");
            let stderr = String::from_utf8(output.stderr).expect("Could not parse command output as a String.");
            
            // println!("stdout from {}: {stdout}", step.get_name());
            // println!("stderr from {}: {stderr}", step.get_name());

            outputs.push(if stdout.is_empty() {
                if stderr.is_empty() {
                    "No standard output detected. Check to see if it was piped to another file.".to_string()
                } else {
                    error!("Standard output from step {}: {}", step.get_name(), stderr);
                    stderr
                }
            } else {
                info!("Standard output from step {}: {}", step.get_name(), stdout);
                stdout
            });
        }
        outputs
    } 
    /*
        * Performs action using Unix-specific configuration.
        */
    else {
        for step in manual {
            let mut command = Command::new("sh");
            command.current_dir("/");
            let output_str = format_args!("Running {}",step.get_name()).to_string();
            info!("{}",output_str);
            println!("{}",output_str);
            outputs.push(output_str);
            let script = step.get_script().to_string();
            println!("{script}");
            let script = clean_script_pathing(&script);
            let output = command.args([vec!["-c"], script.iter().map(String::as_str).collect()].concat()).current_dir(current_dir().unwrap()).output().expect(&("Failed to execute: ".to_string() + &script.concat()));
            let stdout = String::from_utf8(output.stdout).expect("Could not parse command output as a String.");
            let stderr = String::from_utf8(output.stderr).expect("Could not parse command output as a String.");
            
            outputs.push(if stdout.is_empty() {
                if stderr.is_empty() {
                    "No standard output detected. Check to see if it was piped to another file.".to_string()
                } else {
                    error!("Standard output from step {}:\n{}", step.get_name(), stderr);
                    stderr
                }
            } else {
                info!("Standard output from step {}:\n{}", step.get_name(), stdout);
                stdout
            });
        }
        outputs
    }
}


/// Cleans paths used within scripts.
/// TODO: Fix paths being "overcleaned" i.e. directory/"some other directory"/low_dir being split incorrectly
/// TODO: Fix paths being incorrectly parsed (FIX options: split by OS or split into multiple functions.)
/// 
fn clean_script_pathing(script: &str) -> Vec<String> {
    let root = current_dir().unwrap();
    script.split(' ').map(|item| {
        if item.contains("../") || item.contains("./") {
            RelativePath::new(&item).to_path(&root).to_str().unwrap().to_string()
        } else {
            item.to_string()
        }
    }).collect()
}

/// Contains data necessary to perform specific actions in a configurable manner 
pub struct ExecInfo {
    pub backend: String, 
    pub image: Option<String>,
    pub title: Option<String>,
    pub tags: Option<HashMap<String,String>>,
    pub metadata: Option<HashMap<String,String>>,
    pub output: String,
    pub source: String,
    pub conditions: Option<Vec<Condition>>,
    pub manual: Vec<Step>,
    pub retries: i8,
    pub allowed_failure: bool
}

/**
 * Functions to be used by the ExecInfo struct.
 * Should only contain a constructor and/or cleanup scripts.
 */
impl ExecInfo {
    fn new(action: &Action) -> Self {
        ExecInfo { backend: action.get_shared_config().get_backend().to_string(), 
            image: action.get_shared_config().get_image(), 
            title: action.get_shared_config().get_title(), 
            tags: action.get_shared_config().get_tags(), 
            metadata: action.get_shared_config().get_metadata(), 
            output: action.get_shared_config().get_output().to_string(), 
            source: action.get_shared_config().get_source().to_string(), 
            conditions: action.get_action_config().get_conditions(), 
            manual: action.get_action_config().get_manual().to_vec(), 
            retries: *action.get_action_config().get_retries(), 
            allowed_failure: *action.get_action_config().get_allowed_failure()
        }
    }
}

#[cfg(test)]
mod tests {

// use crate::parsing::Parser;

// use crate::executor::executor;

// #[test]
// fn prove_exec_info() {
//     let test_config = Parser::new_top_level("example_docker_config.json");
//     let actions = test_config.get_all_actions();
//     let exec_info = executor::ExecInfo {}
// }

}