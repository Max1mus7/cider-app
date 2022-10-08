pub mod executor {
    use std::{collections::HashMap, env::current_dir};
    use std::process::Command;
    use crate::config::{Action, Condition, Step};
    use log::{info, warn, error};
    use relative_path::RelativePath;
    
    pub fn exec_actions(action_vec: &Vec<Action>) -> Vec<Vec<String>> {
        let mut all_output = vec![];
        for action in action_vec {
            all_output.push(exec_action(action))
        }
        // println!("All output: {:#?}", &all_output);
        all_output
    }

    fn exec_action(action: &Action) -> Vec<String> {
        let exec_info = ExecInfo::new(action);
        match exec_info.backend.to_lowercase().as_str() {
            "bash" => {
                run_bash_scripts(exec_info.manual)
            },
            "docker" => {
                run_bash_scripts(exec_info.manual)
            }
            &_ => {
                panic!("Specified backend not supported");
            }
        }
    }

    fn run_bash_scripts(manual: Vec<Step>) -> Vec<String> {

        let mut outputs = vec![];
        
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
        } else {
            for step in manual {
                let mut command = Command::new("sh");
                command.current_dir("/");
                let output_str = format_args!("Running {}",step.get_name()).to_string();
                info!("{}",output_str);
                println!("{}",output_str);
                outputs.push(output_str);
                let script = step.get_script().to_string();
                println!("{script}");
                let script = step.get_script().to_string();
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

    struct ExecInfo {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    // #[test]
    // fn echo_with_command_windows() {
    //     let output = Command::new("cmd").args(["/C","echo", "Hello World"]).output().expect("Failed to execute command.");
    //     assert_eq!(b"\"Hello World\"\r\n", output.stdout.as_slice());
    // }

    // #[test]
    // fn create_file_with_command() {
    //     Command::new("cmd").args(["/C","echo Hello World >> log1.txt"]).output().expect("Failed to execute command.");
    // }
}