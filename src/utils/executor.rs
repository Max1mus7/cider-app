#![warn(missing_docs)]


use crate::utils::config::{Action, Condition, Step};
use chrono::Utc;
use csv::Writer;
use log::{debug, error, info, warn};
use relative_path::RelativePath;
/**
 * Module used to clean input and execute actions
 * Eventually, this module will also be used to separate pipeline executions and handle conditional logic
 * May also be split into modules on an action/pipeline level in the future
 */
use std::fs::File;
use std::io::Write;
use std::process::{Command, Output, Stdio};
use std::time::SystemTime;
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
        "bash" => run_bash_scripts(&exec_info),
        "batch" => run_batch_script(&exec_info),
        "bat" => run_batch_script(&exec_info),
        "docker" => run_with_docker(exec_info),
        &_ => {
            panic!("Specified backend not supported");
        }
    }
}

fn generate_dockerignore(info: &ExecInfo) -> File {
    let mut file = File::create(format!("{}/.dockerignore", &info.source)).unwrap_or_else(|_| {
            error!("There was an issue creating a dockerignore for your docker backend.\nMake sure there are no files in your project named \".dockerignore\".");
            panic!("There was an issue regarding your dockerignore. Please check your logs for more information.");
        }
    );
    let mut ignored_dirs = String::new();
    if cfg!(windows){
        for dir in info.ignore_dirs.as_ref().unwrap() {
            ignored_dirs += format!("{}\r\n",dir.rsplit_once(".\\").unwrap().1).as_str();
        }
    } else{
        for dir in info.ignore_dirs.as_ref().unwrap() {
            ignored_dirs += format!("{}\r\n",dir.rsplit_once("./").unwrap().1).as_str();
        }
    }
    file.write_fmt(format_args!("{}", ignored_dirs)).unwrap_or_else(|_| {
        error!("There was an issue creating a dockerignore for your docker backend.\nMake sure there are no files in your project named \".dockerignore\".");
        panic!("There was an issue regarding your dockerignore. Please check your logs for more information.");
    });
    file
}
fn generate_dockerfile(info: &ExecInfo) -> File {
    let mut file = File::create(format!("{}/Dockerfile", info.source)).unwrap_or_else(|_| {
            error!("There was an issue creating a dockerfile for your docker backend.\nMake sure there are no files in your project named \"DOCKERFILE\".");
            panic!("There was an issue regarding your dockerfile. Please check your logs for more information.");
        }
    );
    let mut str = format_args!("FROM {}\r\n", info.image.as_ref().unwrap()).to_string();
    str += "WORKDIR /cider/app\r\n";
    str += "COPY . ./\r\n";
    str += "RUN ";
    for step in info.manual.iter() {
        if step != info.manual.last().unwrap(){
            str += format_args!("{} && \\\r\n    ", step.get_script())
                .to_string()
                .as_ref();
        } else {
            str += format_args!("{}", step.get_script())
                .to_string()
                .as_ref();
        }
    }

    file.write_fmt(format_args!("{}", str)).unwrap_or_else(|_| {
        error!("There was an issue creating a dockerfile for your docker backend.\nMake sure there are no files in your project named \"DOCKERFILE\".");
        panic!("There was an issue regarding your dockerfile. Please check your logs for more information.");
    });

    file
}


fn run_batch_script(setup: &ExecInfo) -> Vec<String> {
    let mut outputs = vec![];
    if cfg!(windows) {
        warn!("In order to avoid unexpected behavior, please consider using \"bat\" or \"batch\" backend for windows operating systems.");
        let mut all_steps: Vec<String> = Vec::new();
        let mut command = Command::new("cmd");
        for step in &setup.manual {
            all_steps.append(&mut script_setup(&mut outputs, step));
            if step.get_script() != setup.manual.last().unwrap_or_else(|| {
                error!("{:#?}", "Failed to parse the final Step");
                panic!("{:#?}", "Failed to parse the final Step");
            }).get_script() {
                all_steps.push("&&".to_owned());
            }
        }
        let output = command_setup_windows(&mut command, &mut all_steps, false, setup.source.clone())
                .output()
                .expect(&("Failed to execute: ".to_string() + &all_steps.concat()));
            collect_piped_output(setup, &output, &mut outputs);
    } else {
        error!("As of now, running batch scripts is unsupported on non-windows systems.");
        outputs.push(
            "A batch script was unable to be processed on Linux and was taken care of safely."
                .to_string(),
        );
    }
    outputs
}

fn run_with_docker(setup: ExecInfo) -> Vec<String> {
    let mut setup = setup;
    let mut outputs = vec![];
    image_setup(&mut setup, &mut outputs);
    generate_dockerignore(&setup);
    generate_dockerfile(&setup);

    let csv_headers = vec!["Image_pull_time", "Image_remove_time", "Image_build_time"];
    let mut csv_data: Vec<&str> = vec![];

    if cfg!(windows) {

        let log_time = Utc::now().format("%d-%m_%H%M%S");
        let log_file = "./metrics/win/".to_string() + log_time.to_string().as_str() + ".csv";
        let mut csv_wtr = Writer::from_path(log_file).unwrap_or_else(|err| {
            error!("{}", err);
            panic!("{}", err);
        });

        let image_pull_time = SystemTime::now();
        let mut cmd = Command::new("cmd");
        let mut process = docker_setup_windows(&mut cmd, &setup, true)
            .spawn()
            .expect("There was an error building your docker environment.");
        process.wait().unwrap_or_else(|err| {
            error!("{:#?}", err);
            panic!("{:#?}", err);
        });
        info!("{:#?}", &image_pull_time.elapsed().unwrap());

        let image_pull_string = format!("{:?}", image_pull_time.elapsed().unwrap());
        csv_data.push(&image_pull_string);

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

        let image_rm_string = format!("{:?}", image_rm_time.elapsed().unwrap());
        csv_data.push(&image_rm_string);

        let image_build_time = SystemTime::now();
        let mut cmd = Command::new("cmd");
        let mut process = docker_build_windows(&mut cmd, &setup, true)
            .spawn()
            .expect("There was an error building your docker environment.");
        process.wait().unwrap_or_else(|err| {
            error!("{:#?}", err);
            panic!("{:#?}", err);
        });
        info!("{:#?}", image_build_time.elapsed().unwrap());

        let image_build_string = format!("{:?}", image_build_time.elapsed().unwrap());
        csv_data.push(&image_build_string);

        csv_wtr.write_record(&csv_headers).unwrap();
        csv_wtr.write_record(&csv_data).unwrap();
        csv_wtr.flush().unwrap();

    } else {
        let mut cmd = Command::new("sh");
        let mut process = docker_setup_unix(&mut cmd, &setup, true)
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
        let mut process = docker_build_unix(&mut cmd, &setup, true)
            .spawn()
            .expect("There was an error building your docker environment.");
        process.wait().unwrap_or_else(|err| {
            panic!("{:#?}", err);
        });
    }

    outputs
}

///Runs bash scripts defined in an Action's Manual
fn run_bash_scripts(setup: &ExecInfo) -> Vec<String> {
    let mut outputs = vec![];

    if cfg!(windows) {
        warn!("In order to avoid unexpected behavior, please consider using \"bat\" or \"batch\" backend for windows operating systems.");
        let mut all_steps: Vec<String> = Vec::new();
        let mut command = Command::new("cmd");
        for step in &setup.manual {
            all_steps.append(&mut script_setup(&mut outputs, step));
            if step.get_script() != setup.manual.last().unwrap_or_else(|| {
                error!("{:#?}", "Failed to parse the final Step");
                panic!("{:#?}", "Failed to parse the final Step");
            }).get_script() {
                all_steps.push("&&".to_owned());
            }
        }
        let output = command_setup_windows(&mut command, &mut all_steps, false, setup.source.clone())
                .output()
                .expect(&("Failed to execute: ".to_string() + &all_steps.concat()));
            collect_piped_output(setup, &output, &mut outputs);
    } else {
        for step in &setup.manual {
            let mut command = Command::new("sh");
            let mut script = script_setup(&mut outputs, step);
            let output = command_setup_unix(&mut command, &mut script, false, setup.source.clone())
                .output()
                .expect(&("Failed to execute: ".to_string() + &script.concat()));
            collect_piped_output(setup, &output, &mut outputs)
        }
    }
    outputs
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
/// Combines information from both [`crate::utils::config::ShareableConfiguration`] and [`crate::utils::config::ActionConfig`]
/// See [`crate::utils::config`] for more information.
#[derive(Debug)]
pub struct ExecInfo {
    /// See [`crate::utils::config::ShareableConfiguration`] for more information.
    pub backend: String,
    /// See [`crate::utils::config::ShareableConfiguration`] for more information.
    pub image: Option<String>,
    /// See [`crate::utils::config::ShareableConfiguration`] for more information.
    pub title: Option<String>,
    /// See [`crate::utils::config::ShareableConfiguration`] for more information.
    pub tags: Option<HashMap<String, String>>,
    /// See [`crate::utils::config::ShareableConfiguration`] for more information.
    pub metadata: Option<HashMap<String, String>>,
    /// See [`crate::utils::config::ShareableConfiguration`] for more information.
    pub output: String,
    /// See [`crate::utils::config::ShareableConfiguration`] for more information.
    pub source: String,
    /// See [`crate::utils::config::ActionConfig`] for more information.
    pub conditions: Option<Vec<Condition>>,
    /// See [`crate::utils::config::ActionConfig`] for more information.
    pub manual: Vec<Step>,
    /// See [`crate::utils::config::ActionConfig`] for more information.
    pub retries: i8,
    /// See [`crate::utils::config::ActionConfig`] for more information.
    pub allowed_failure: bool,
    /// See [`crate::utils::config::ShareableConfiguration`] for more information.
    pub ignore_dirs: Option<Vec<String>>
}

/**
 * Functions to be used by the ExecInfo struct.
 * Should only contain a constructor and/or cleanup scripts.
 */
impl ExecInfo {
    fn new(action: &Action) -> Self {
        ExecInfo {
            backend: action.shared_config.get_backend().to_string(),
            image: action.shared_config.get_image(),
            title: action.shared_config.get_title(),
            tags: action.shared_config.get_tags(),
            metadata: action.shared_config.get_metadata(),
            output: action.shared_config.get_output().to_string(),
            source: action.shared_config.get_source().to_string(),
            conditions: action.action_config.get_conditions(),
            manual: action.action_config.get_manual().to_vec(),
            retries: *action.action_config.get_retries(),
            allowed_failure: *action.action_config.get_allowed_failure(),
            ignore_dirs: action.shared_config.get_ignore_dirs()
        }
    }
}

fn command_setup_windows<'a>(
    cmd: &'a mut Command,
    args: &mut Vec<String>,
    inherit: bool,
    source: String
) -> &'a mut Command {
    //pass command first?

    args.insert(0, "/C".to_string());
    if inherit {
        return set_output_inherit(cmd.args(args).current_dir(source));
    }
    set_output_piped(cmd.args(args).current_dir(source))
}

fn command_setup_unix<'a>(
    cmd: &'a mut Command,
    args: &mut Vec<String>,
    inherit: bool,
    source: String
) -> &'a mut Command {
    let mut arg_string = String::new();
    for arg in args {
        arg_string += &(arg.to_owned() + " ");
    }

    arg_string = arg_string.trim().to_string();
    if inherit {
        return set_output_inherit(cmd.arg("-c").arg(arg_string).current_dir(source));
    }
    set_output_piped(cmd.arg("-c").arg(arg_string).current_dir(source))
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

fn docker_setup_unix<'a>(cmd: &'a mut Command, info: &ExecInfo, inherit: bool) -> &'a mut Command {
    cmd.arg("-c")
        .arg(format_args!("docker pull {}", &info.image.clone().unwrap()).to_string().as_str()).current_dir(&info.source);
    if inherit {
        return set_output_inherit(cmd);
    }
    set_output_piped(cmd)
}

fn docker_setup_windows<'a>(cmd: &'a mut Command, info: &ExecInfo, inherit: bool) -> &'a mut Command {
    cmd.args(vec!["/C", "docker", "pull", &info.image.clone().unwrap()])
        .current_dir(&info.source);
    if inherit {
        return set_output_inherit(cmd);
    }
    set_output_piped(cmd)
}

fn docker_clean_unix(cmd: &mut Command, inherit: bool) -> &mut Command {
    cmd.arg("-c").arg("docker image rm -f cider-image");
    debug!("Running {:#?}",cmd);
    if inherit {
        return set_output_inherit(cmd);
    }
    set_output_piped(cmd)
}

fn docker_clean_windows(cmd: &mut Command, inherit: bool) -> &mut Command {
    cmd.args(vec!["/C", "docker", "image", "rm", "-f", "cider-image"]);
    debug!("Running {:#?}",cmd);
    if inherit {
        return set_output_inherit(cmd);
    }
    set_output_piped(cmd)
}

fn docker_build_unix<'a>(cmd: &'a mut Command, info: &ExecInfo, inherit: bool) -> &'a mut Command {
    cmd.arg("-c").arg("docker build -t cider-image .").current_dir(&info.source);
    debug!("Running {:#?}",cmd);
    if inherit {
        return set_output_inherit(cmd);
    }
    set_output_piped(cmd)
}

fn docker_build_windows<'a>(cmd: &'a mut Command, info: &ExecInfo, inherit: bool) -> &'a mut Command {
    cmd.args(["/C", "docker", "build", "-t", "cider-image", ".", "--no-cache"]).current_dir(&info.source);
    debug!("Running {:#?}",cmd);
    if inherit {
        return set_output_inherit(cmd);
    }
    set_output_piped(cmd)
}

/// Potential issues:
/// Some success outputs may be read as failures on Linux environments. Look into this more.
fn collect_piped_output(setup: &ExecInfo, output: &Output, outputs: &mut Vec<String>) {
    let stdout = String::from_utf8(output.stdout.clone())
        .expect("Could not parse command output as a String.");
    let stderr = String::from_utf8(output.stderr.clone())
        .expect("Could not parse command output as a String.");

    println!("Output from {:#?}: {stdout}", setup.title.to_owned().unwrap_or_else(|| String::from("Untitled Step")));
    println!("Errors from {:#?}: {stderr}", setup.title.to_owned().unwrap_or_else(|| String::from("Untitled Step")));

    outputs.push(if stdout.is_empty() {
        if stderr.is_empty() {
            "No standard output detected. Check to see if it was piped to another file.".to_string()
        } else {
            error!("Standard output from step {:#?}: {:#?}", setup.title.to_owned().unwrap_or_else(|| String::from("Untitled Step")), stderr);
            stderr.trim_end().to_owned()
        }
    } else {
        info!("Standard output from step {:#?}: {:#?}", setup.title.to_owned().unwrap_or_else(|| String::from("Untitled Step")), stdout);
        stdout.trim_end().to_owned()
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

    use std::process::Command;
    use relative_path::RelativePath;

    use super::*;

    #[test]
    fn test_create_command_path_abs_path() {
        let source = String::from("D:\\Coding Projects");
        let mut command = Command::new("cmd");
        command.current_dir(source.clone());
        // let root = current_dir().unwrap();
        assert_eq!(RelativePath::new(&source)
        .to_path(""), command.get_current_dir().unwrap())
    }

    #[test]
    fn test_script_path_cleaning() {
        let expected = vec![String::from("cat"),String::from("D:\\Coding Projects\\cider-app\\..\\test.txt")];
        let test_script = "cat ../test.txt";
        // let root = current_dir().unwrap();
        let res = clean_script_pathing(test_script);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_set_new_command_directory() {
        let expected_dir = "D:\\Coding Projects";
        let mut steps: Vec<String> = Vec::new();
        if cfg!(windows) {
            steps.push(String::from("cd"));
            let mut command = Command::new("cmd");
            let output = command_setup_windows(&mut command, &mut steps, false, String::from(expected_dir))
                .output()
                .expect(&("Failed to execute: ".to_string() + &steps.concat()));
            assert_eq!(expected_dir.to_owned(), String::from_utf8(output.stdout.clone()).unwrap().to_owned().trim_end());
        } else {
            steps.push(String::from("pwd"));
            let mut command = Command::new("cmd");
            let output = command_setup_unix(&mut command, &mut steps, false, String::from(expected_dir))
                .output()
                .expect(&("Failed to execute: ".to_string() + &steps.concat()));
            assert_eq!(expected_dir.to_owned(), String::from_utf8(output.stdout.clone()).unwrap().to_owned().trim_end());
        }
        // println!("{:#?}, {:#?}, {:#?}", expected_dir,  String::from_utf8(output.stdout.clone()).unwrap().to_owned().trim_end(), String::from_utf8(output.stderr.clone()).unwrap().to_owned());
    }

    // #[test]
    // fn test_create_command_windows() {
    //     //
    //     let input1 = "input";
    //     let input2 = "input";
    //     let mut result = Command::new("cmd").args(["/C", "echo", "get results"]);
    //     assert!(input1 != input2);
    // }
}
