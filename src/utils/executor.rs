use crate::utils::config::{Action, Condition, Step};
use log::{error, info, warn};
use relative_path::RelativePath;
use std::time::{SystemTime};
use chrono::{DateTime, Utc};


/**
 * Module used to clean input and execute actions
 * Eventually, this module will also be used to separate pipeline executions and handle conditional logic
 * May also be split into modules on an action/pipeline level in the future
 */
use std::fs::{File};
use std::io::Write;
use std::process::{Command, Output, Stdio};
use std::{collections::HashMap, env::current_dir};

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
        "bash" => run_bash_scripts(exec_info.manual),
        "batch" => run_batch_script(exec_info.manual),
        "bat" => run_batch_script(exec_info.manual),
        "docker" => run_with_docker(exec_info),
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
        str += format_args!("RUN {}\r\n", step.get_script())
            .to_string()
            .as_ref();
    }

    file.write_fmt(format_args!("{}", str)).unwrap_or_else(|_| {
        error!("There was an issue creating a dockerfile for your docker backend.\nMake sure there are no files in your project named \"DOCKERFILE\".");
        panic!("There was an issue regarding your dockerfile. Please check your logs for more information.");
    });

    file
}

/**
 * Runs a batch script (WINDOWS ONLY RIGHT NOW)
 */
fn run_batch_script(manual: Vec<Step>) -> Vec<String> {
    let mut outputs = vec![];

    if cfg!(windows) {
        warn!("In order to avoid unexpected behavior, please consider using \"bat\" or \"batch\" backend for windows operating systems.");
        for step in manual {
            let mut command = Command::new("cmd");
            let mut script = script_setup(&mut outputs, &step);
            let output = command_setup_windows(&mut command, &mut script, false)
                .output()
                .expect(&("Failed to execute: ".to_string() + &script.concat()));
            collect_piped_output(&step, &output, &mut outputs);
        }
        return outputs;
    } else {
        error!("As of now, running batch scripts is unsupported on non-windows systems.");
        outputs.push(
            "A batch script was unable to be processed on Linux and was taken care of accordingly."
                .to_string(),
        );
    }
    outputs
}

fn run_with_docker(setup: ExecInfo) -> Vec<String> {
    let mut setup = setup;
    let mut outputs = vec![];
    image_setup(&mut setup, &mut outputs);
    generate_dockerfile(&setup);
    if cfg!(windows) {
        let log_time = Utc::now().format("%d-%m_%H%M%S");
        let log_file = "./metrics/win/".to_string() + log_time.to_string().as_str() + ".txt";
        let mut metrics_file = File::create(log_file).unwrap_or_else(|err| {
            error!("{}", err);
            panic!("{}", err);
        });
        let image_pull_time = SystemTime::now();
        let mut cmd = Command::new("cmd");
        let mut process = docker_setup_windows(&mut cmd, &setup.image.unwrap(), true)
            .spawn()
            .expect("There was an error building your docker environment.");
        process.wait().unwrap_or_else(|err| {
            error!("{:#?}", err);
            panic!("{:#?}", err);
        });
        info!("{:#?}", image_pull_time.elapsed().unwrap());
        metrics_file.write(format!("Image pull time: {:#?}\n", image_pull_time.elapsed().unwrap()).as_bytes()).unwrap_or_else(|err| {
            error!("There was an issue writing the image pull time to a file.");
            0
        });
        metrics_file.flush().unwrap();
        let image_rm_time = SystemTime::now();
        let mut cmd = Command::new("cmd");
        let mut process = docker_clean_windows(&mut cmd, true)
            .spawn()
            .expect("There was an error building your docker environment.");
        process.wait().unwrap_or_else(|err| {
            error!("{:#?}", err);
            panic!("{:#?}", err);
        });
        info!("{:#?}", image_rm_time.elapsed().unwrap());
        metrics_file.write(format!("Image remove time: {:#?}\n", image_rm_time.elapsed().unwrap()).as_bytes()).unwrap_or_else(|err| {
            error!("There was an issue writing the image removal time to a file: {}", err);
            0
        });
        metrics_file.flush().unwrap();
        let image_build_time = SystemTime::now();
        let mut cmd = Command::new("cmd");
        let mut process = docker_build_windows(&mut cmd, true)
            .spawn()
            .expect("There was an error building your docker environment.");
        process.wait().unwrap_or_else(|err| {
            error!("{:#?}", err);
            panic!("{:#?}", err);
        });
        info!("{:#?}", image_build_time.elapsed().unwrap());
        metrics_file.write(format!("Image build time: {:#?}\n", image_build_time.elapsed().unwrap()).as_bytes()).unwrap_or_else(|err| {
            error!("There was an issue writing the image build time to a file: {}", err);
            0
        });
        metrics_file.flush().unwrap();

    } else {
        let mut cmd = Command::new("sh");
        let mut process = docker_setup_unix(&mut cmd, &setup.image.unwrap(), true)
            .spawn()
            .expect("There was an error building your docker environment.");
        process.wait().unwrap_or_else(|err| {
            panic!("{:#?}", err);
        });
        let mut cmd = Command::new("sh");
        let mut process = docker_clean_unix(&mut cmd, true)
            .spawn()
            .expect("There was an error building your docker environment.");
        process.wait().unwrap_or_else(|err| {
            panic!("{:#?}", err);
        });
        let mut cmd = Command::new("sh");
        let mut process = docker_build_unix(&mut cmd, true)
            .spawn()
            .expect("There was an error building your docker environment.");
        process.wait().unwrap_or_else(|err| {
            panic!("{:#?}", err);
        });
    }

    outputs
}

///Runs bash scripts defined in an Action's Manual
fn run_bash_scripts(manual: Vec<Step>) -> Vec<String> {
    let mut outputs = vec![];
    
    if cfg!(windows) {
        warn!("In order to avoid unexpected behavior, please consider using \"bat\" or \"batch\" backend for windows operating systems.");
        for step in manual {
            let mut command = Command::new("cmd");
            let mut script = script_setup(&mut outputs, &step);
            let output = command_setup_windows(&mut command, &mut script, false)
                .output()
                .expect(&("Failed to execute: ".to_string() + &script.concat()));
            collect_piped_output(&step, &output, &mut outputs);
        }
        outputs
    } else {
        for step in manual {
            let mut command = Command::new("sh");
            let mut script = script_setup(&mut outputs, &step);
            let output = command_setup_unix(&mut command, &mut script, false)
                .output()
                .expect(&("Failed to execute: ".to_string() + &script.concat()));
            collect_piped_output(&step, &output, &mut outputs)
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
    script
        .split(' ')
        .map(|item| {
            if item.contains("../") || item.contains("./") {
                RelativePath::new(&item)
                    .to_path(&root)
                    .to_str()
                    .unwrap()
                    .to_string()
            } else {
                item.to_string()
            }
        })
        .collect()
}

/// Contains data necessary to perform specific actions in a configurable manner
pub struct ExecInfo {
    pub backend: String,
    pub image: Option<String>,
    pub title: Option<String>,
    pub tags: Option<HashMap<String, String>>,
    pub metadata: Option<HashMap<String, String>>,
    pub output: String,
    pub source: String,
    pub conditions: Option<Vec<Condition>>,
    pub manual: Vec<Step>,
    pub retries: i8,
    pub allowed_failure: bool,
}

/**
 * Functions to be used by the ExecInfo struct.
 * Should only contain a constructor and/or cleanup scripts.
 */
impl ExecInfo {
    fn new(action: &Action) -> Self {
        ExecInfo {
            backend: action.get_shared_config().get_backend().to_string(),
            image: action.get_shared_config().get_image(),
            title: action.get_shared_config().get_title(),
            tags: action.get_shared_config().get_tags(),
            metadata: action.get_shared_config().get_metadata(),
            output: action.get_shared_config().get_output().to_string(),
            source: action.get_shared_config().get_source().to_string(),
            conditions: action.get_action_config().get_conditions(),
            manual: action.get_action_config().get_manual().to_vec(),
            retries: *action.get_action_config().get_retries(),
            allowed_failure: *action.get_action_config().get_allowed_failure(),
        }
    }
}

fn command_setup_windows<'a>(
    cmd: &'a mut Command,
    args: &mut Vec<String>,
    inherit: bool,
) -> &'a mut Command {
    //pass command first?

    args.insert(0, "/C".to_string());
    if inherit {
        return set_output_inherit(cmd.args(args).current_dir(current_dir().unwrap()));
    }
    set_output_piped(cmd.args(args).current_dir(current_dir().unwrap()))
}


fn image_setup(setup: &mut ExecInfo, outputs: &mut Vec<String>) {
    if setup.image.is_none() {
        setup.image = Some("alpine:latest".to_string());
        warn!("There was no image detected in a configured action.");
        outputs.push(
            "There was no docker image found to build off of. Using Alpine Linux by default."
                .to_string(),
        );
    }
}

fn docker_setup_unix<'a>(cmd: &'a mut Command, image: &str, inherit: bool) -> &'a mut Command {
    cmd.arg("-c")
        .arg(format_args!("docker pull {}", image).to_string().as_str());
    if inherit {
        return set_output_inherit(cmd);
    }
    set_output_piped(cmd)
}

fn docker_setup_windows<'a>(cmd: &'a mut Command, image: &str, inherit: bool) -> &'a mut Command {
    cmd.args(vec!["/C", "docker", "pull", image])
        .current_dir(current_dir().unwrap());
    if inherit {
        return set_output_inherit(cmd);
    }
    set_output_piped(cmd)
}

fn docker_clean_unix<'a>(cmd: &'a mut Command, inherit: bool) -> &'a mut Command {
    cmd.arg("-c").arg("docker image rm -f cider-image");
    if inherit {
        return set_output_inherit(cmd);
    }
    set_output_piped(cmd)
}

fn docker_clean_windows<'a>(cmd: &'a mut Command, inherit: bool) -> &'a mut Command {
    cmd.args(vec!["/C", "docker", "image", "rm", "-f", "cider-image"])
        .current_dir(current_dir().unwrap());
    if inherit {
        return set_output_inherit(cmd);
    }
    set_output_piped(cmd)
}

fn docker_build_unix<'a>(cmd: &'a mut Command, inherit: bool) -> &'a mut Command {
    cmd.arg("-c").arg("docker build -t cider-image .");
    if inherit {
        return set_output_inherit(cmd);
    }
    set_output_piped(cmd)
}

fn docker_build_windows<'a>(cmd: &'a mut Command, inherit: bool) -> &'a mut Command {
    cmd.args(["/C", "docker", "build", "-t", "cider-image", "."]);
    if inherit {
        return set_output_inherit(cmd);
    }
    set_output_piped(cmd)
}

fn command_setup_unix<'a>(
    cmd: &'a mut Command,
    args: &mut Vec<String>,
    inherit: bool,
) -> &'a mut Command {
    let mut arg_string = String::new();
    for arg in args {
        arg_string += &(arg.to_owned() + " ");
    }

    arg_string = arg_string.trim().to_string();
    if inherit {
        return set_output_inherit(cmd.arg("-c").arg(arg_string));
    }
    return set_output_piped(cmd.arg("-c").arg(arg_string));
}

/// Potential issues:
/// Some success outputs may be read as failures on Linux environments. Look into this more.
fn collect_piped_output(step: &Step, output: &Output, outputs: &mut Vec<String>) {
    let stdout = String::from_utf8(output.stdout.clone())
        .expect("Could not parse command output as a String.");
    let stderr = String::from_utf8(output.stderr.clone())
        .expect("Could not parse command output as a String.");

    println!("stdout from {}: {stdout}", step.get_name());
    println!("stderr from {}: {stderr}", step.get_name());

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

fn set_output_inherit(command: &mut Command) -> &mut Command {
    command.stdout(Stdio::inherit()).stderr(Stdio::inherit())
}

fn set_output_piped(command: &mut Command) -> &mut Command {
    command.stdout(Stdio::piped()).stderr(Stdio::piped())
}

fn script_setup(outputs: &mut Vec<String>, step: &Step) -> Vec<String> {
    let output_str = format_args!("Running {}", step.get_name()).to_string();
    info!("{}", output_str);
    println!("{}", output_str);
    outputs.push(output_str);
    let script = step.get_script().to_string();
    println!("{script}");
    clean_script_pathing(&script)
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

    #[test]
    fn create_command_windows() {
        //
        let input1 = "input";
        let input2 = "input";
        assert!(input1 == input2);
    }
}
