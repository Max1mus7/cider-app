/// Parses Json information into a program-readable configuration
pub mod json_parser {

    use crate::utils::config::*;
    use json::JsonValue;
    use log::{debug, error, info, warn};
    use relative_path::RelativePath;
    use std::env::current_dir;
    use std::path::Path;
    use std::{collections::HashMap, fs};

    /// Parses a map of JSON information into a HashMap<String,String>
    ///
    /// Iterates through a JSON hashmap and parses its data into a HashMap<String,String>
    fn parse_json_map(json: &JsonValue) -> HashMap<String, String> {
        // println!("{:#?}", json);
        let mut map = HashMap::new();
        for key_value in json.entries() {
            debug!("{:#?}", &key_value);
            map.insert(key_value.0.to_string(), key_value.1.to_string());
        }
        // println!("{:#?}", json);
        if map.is_empty() {
            warn!("No mappable values found in json hashmap {:#?}", json);
            return map;
        }
        map
    }

    /// Parses a map of JSON information into a Vector of Condition objects
    ///
    /// Iterates through a JSON hashmap and parses its data into a Vec<Condition>
    fn parse_json_to_conditions(json: &JsonValue) -> Vec<Condition> {
        debug!("Converting \n{:#?} into Conditions", json);
        let mut conditions = vec![];
        for key_value in json.entries() {
            let condition = Condition::new(
                key_value.0.to_string(),
                key_value.1.to_string(),
            );
            debug!("Condition created: {:#?}", &condition);
            conditions.push(condition);
        }
        conditions
    }

    /// Parses a map of JSON information into a Vector of Step objects
    ///
    /// Iterates through a JSON hashmap and parses its data into a Vec<Step>
    fn parse_json_to_steps(json: &JsonValue) -> Vec<Step> {
        debug!("Converting \n{:#} into Steps.", json);
        let mut steps = vec![];
        for key_value in json.entries() {
            let step = Step::new(key_value.0.to_string(), key_value.1.to_string());
            debug!("Step parsed: {:#?}", &step);
            steps.push(step);
        }
        steps
    }

    fn parse_json_vector(json: &JsonValue) -> Vec<String> {
        debug!("Converting \n{:#} into a String vector.", json);
        let mut vec = vec![];
        for value in json.members() {
            vec.push(value.to_string())
        }
        if vec.is_empty() {
            warn!("No mappable values found in json vector {:#?}", json);
            return vec;
        }
        vec
    }

    fn parse_action_defs(
        shared_config: &ShareableConfiguration,
        action_defs: &Vec<String>,
        data: &JsonValue,
    ) -> Vec<Action> {
        let mut actions = vec![];
        for action_name in action_defs {
            debug!("Parsing action {}", action_name);
            actions.push(parse_action(shared_config, &data[action_name], action_name));
        }
        actions
    }

    fn parse_action(
        shared_config: &ShareableConfiguration,
        json: &JsonValue,
        name: &str,
    ) -> Action {
        let root = current_dir().unwrap();
        if json.is_null() {
            panic!(
                "Could not find action defined with appropriate tag: {}",
                name
            )
        }
        let backend = {
            if json["backend"].is_null() {
                shared_config.get_backend().to_string()
            } else {
                json["backend"].to_string()
            }
        };
        let new_shared_config = ShareableConfiguration::new(
            {
                if json["metadata"].is_null() {
                    None
                } else {
                    Some(parse_json_map(&json["metadata"]))
                }
            },
            Some(name.to_string()),
            {
                if json["tags"].is_null() {
                    None
                } else {
                    Some(parse_json_map(&json["tags"]))
                }
            },
            {
                if json["language"].is_null() {
                    shared_config.get_language().to_string()
                } else {
                    json["language"].to_string()
                }
            },
            {
                if !backend.to_lowercase().eq("docker")
                    && !backend.is_empty()
                    && backend != "bash"
                    && backend != "batch"
                {
                    warn!("Image cannot be set if docker is not the backend.");
                    None
                } else if json["image"].is_null() {
                    shared_config.get_image()
                } else {
                    Some(json["image"].to_string())
                }
            },
            backend,
            {
                if json["output_directory"].is_null() {
                    shared_config.get_output().to_string()
                } else {
                    RelativePath::new(&json["output_directory"].to_string())
                        .to_path(&root)
                        .to_str()
                        .unwrap()
                        .to_string()
                }
            },
            {
                if json["source_directory"].is_null() {
                    shared_config.get_source().to_string()
                } else if json["source_directory"].to_string().starts_with('/') || json["source_directory"].to_string().contains(":") {
                    Path::new(&json["source_directory"].to_string())
                        .to_owned()
                        .to_str()
                        .unwrap()
                        .to_owned()
                } else {
                    RelativePath::new(&json["source_directory"].to_string())
                        .to_path(&root)
                        .to_str()
                        .unwrap()
                        .to_string()
                }
            },
            {
                if json["ignore_directories"].is_null() {
                    shared_config.get_ignore_dirs()
                } else {
                    let mut ignore_dirs: Vec<String> = vec![];
                    for dir in json["ignore_directories"].members() {
                        if dir.as_str().unwrap().to_owned().starts_with('/') || dir.as_str().unwrap().to_string().contains(":") {
                            ignore_dirs.push(Path::new(&dir.as_str().unwrap().to_string())
                            .to_owned()
                            .to_str()
                            .unwrap()
                            .to_owned());
                        } else {
                            ignore_dirs.push(RelativePath::new(&dir.as_str().unwrap().to_string())
                                .to_path(&root)
                                .to_str()
                                .unwrap()
                                .to_string());
                        }
                    }
                    if !ignore_dirs.is_empty() {
                        Some(ignore_dirs)
                    } else {
                        None
                    }
                }
            }
        );

        let action_config = ActionConfig::new(
            {
                let conditions = parse_json_to_conditions(&json["conditions"]);
                if conditions.is_empty() {
                    None
                } else {
                    Some(conditions)
                }
            },
            {
                if json["retries"].is_null() {
                    Some(0)
                } else {
                    Some(json["retries"].as_i8().unwrap_or_else(|| {
                            error!("There was no valid value for retries in the configuration. Error occured in Action: {}", name);
                            panic!("There was no valid value for retries in the configuration. Error occured in Action: {}", name);
                        }))
                }
            },
            {
                if json["allowed_failure"].is_null() {
                    Some(false)
                } else {
                    Some(json["allowed_failure"].as_bool().unwrap_or_else(|| {
                            error!("There was no valid value for failure allowance in the configuration. Please provide a boolean value. Error occured in Action: {}", name);
                            panic!("There was no valid value for failure allowance in the configuration. Please provide a boolean value. Error occured in Action: {}", name);
                            }
                        ))
                }
            },
            {
                let manual = parse_json_to_steps(&json["manual"]);
                if manual.is_empty() {
                    error!("Actions require at least one step in their manual. Error occured in Action: {}", name);
                    panic!("Actions require at least one step in their manual. Error occured in Action: {}", name);
                }
                manual
            },
        );
        Action::new(new_shared_config, action_config)
    }

    /**
     *
     */
    fn parse_pipeline_defs(
        shared_config: &ShareableConfiguration,
        json: &JsonValue,
        pipeline_defs: &Vec<String>,
    ) -> Vec<Pipeline> {
        let mut pipelines = vec![];
        for str in pipeline_defs {
            pipelines.push(parse_pipeline(shared_config, &json[str], str));
        }
        pipelines
    }

    /**
     *
     *
     */
    fn parse_pipeline(
        shared_config: &ShareableConfiguration,
        json: &JsonValue,
        name: &str,
    ) -> Pipeline {
        let root = current_dir().unwrap();
        if json.is_null() {
            panic!("No pipeline found with the name: {}", name);
        }
        let backend = {
            if json["backend"].is_null() {
                shared_config.get_backend().to_string()
            } else {
                json["backend"].to_string()
            }
        };

        let new_shared_config = ShareableConfiguration::new(
            {
                if json["metadata"].is_null() {
                    None
                } else {
                    Some(parse_json_map(&json["metadata"]))
                }
            },
            Some(name.to_string()),
            {
                if json["tags"].is_null() {
                    None
                } else {
                    Some(parse_json_map(&json["tags"]))
                }
            },
            {
                if json["language"].is_null() {
                    shared_config.get_language().to_string()
                } else {
                    json["language"].to_string()
                }
            },
            {
                if !backend.to_lowercase().eq("docker")
                    && !backend.is_empty()
                    && backend != "bash"
                    && backend != "batch"
                {
                    warn!("Image cannot be set if docker is not the backend.");
                    None
                } else if json["image"].is_null() {
                    shared_config.get_image()
                } else {
                    Some(json["image"].to_string())
                }
            },
            backend,
            {
                if json["output_directory"].is_null() {
                    shared_config.get_output().to_string()
                } else {
                    RelativePath::new(&json["output_directory"].to_string())
                        .to_path(&root)
                        .to_str()
                        .unwrap()
                        .to_string()
                }
            },
            {
                if json["source_directory"].is_null() {
                    shared_config.get_source().to_string()
                } else if json["source_directory"].to_string().starts_with('/') || json["source_directory"].to_string().contains(":") {
                    Path::new(&json["source_directory"].to_string())
                        .to_owned()
                        .to_str()
                        .unwrap()
                        .to_owned()
                } else {
                    RelativePath::new(&json["source_directory"].to_string())
                        .to_path(&root)
                        .to_str()
                        .unwrap()
                        .to_string()
                }
            },
            {
                if json["ignore_directories"].is_null() {
                    shared_config.get_ignore_dirs()
                } else {
                    let mut ignore_dirs: Vec<String> = vec![];
                    //TODO: Error when value exists but is not a directory.
                    for dir in json["ignore_directories"].members() {
                        if dir.as_str().unwrap().to_owned().starts_with('/') || dir.as_str().unwrap().to_string().contains(":") {
                            ignore_dirs.push(Path::new(&dir.as_str().unwrap().to_string())
                            .to_owned()
                            .to_str()
                            .unwrap()
                            .to_owned());
                        } else {
                            ignore_dirs.push(RelativePath::new(&dir.as_str().unwrap().to_string())
                                .to_path(&root)
                                .to_str()
                                .unwrap()
                                .to_string());
                        }
                    }
                    if !ignore_dirs.is_empty() {
                        Some(ignore_dirs)
                    } else {
                        None
                    }
                }
            }
        );

        let pipeline_config = PipelineConfig::new(
            {
                let conditions = parse_json_to_conditions(&json["conditions"]);
                if conditions.is_empty() {
                    None
                } else {
                    Some(conditions)
                }
            },
            {
                if json["actions"].is_null() {
                    panic!("No list of action definitions found!");
                } else {
                    parse_json_vector(&json["actions"])
                }
            },
            parse_action_defs(
                &new_shared_config,
                &parse_json_vector(&json["actions"]),
                json,
            ),
            {
                if json["requires"].is_null() {
                    None
                } else {
                    Some(parse_json_vector(&json["requires"]))
                }
            },
        );
        Pipeline::new(new_shared_config, pipeline_config)
    }


    fn parse_shared_config(json: &JsonValue) -> ShareableConfiguration {
        let root = current_dir().unwrap();
        let backend = {
            if json["backend"].is_null() {
                "bash".to_string()
            } else {
                json["backend"].to_string()
            }
        };
        let new_shared_config = ShareableConfiguration::new(
            {
                if json["metadata"].is_null() {
                    None
                } else {
                    Some(parse_json_map(&json["metadata"]))
                }
            },
            Some(json["title"].to_string()),
            {
                if json["tags"].is_null() {
                    None
                } else {
                    Some(parse_json_map(&json["tags"]))
                }
            },
            {
                if json["language"].is_null() {
                    "Python".to_string()
                } else {
                    json["language"].to_string()
                }
            },
            {
                if !backend.to_lowercase().eq("docker")
                    && !backend.is_empty()
                    && backend != "bash"
                    && backend != "batch"
                {
                    warn!("Image cannot be set if docker is not the backend.");
                    None
                } else if json["image"].is_null() {
                    None
                } else {
                    Some(json["image"].to_string())
                }
            },
            backend,
            {
                if json["output_directory"].is_null() {
                    debug!("{}", "No output directory specified. Defaulting to output dir ./dist/cider/");
                    RelativePath::new("./dist/cider/")
                        .to_path(&root)
                        .to_str()
                        .unwrap()
                        .to_string()
                } else {
                    debug!("{}{}", "Output directory specified: ", &json["output_directory"]);
                    RelativePath::new(&json["output_directory"].to_string())
                        .to_path(&root)
                        .to_str()
                        .unwrap()
                        .to_string()
                }
            },
            {
                if json["source_directory"].is_null() {
                    RelativePath::new("./")
                        .to_path(&root)
                        .to_str()
                        .unwrap()
                        .to_string()
                } else if json["source_directory"].to_string().starts_with('/') || json["source_directory"].to_string().contains(":") {
                    Path::new(&json["source_directory"].to_string())
                        .to_owned()
                        .to_str()
                        .unwrap()
                        .to_owned()
                } else {
                    RelativePath::new(&json["source_directory"].to_string())
                        .to_path(&root)
                        .to_str()
                        .unwrap()
                        .to_string()
                }
            },
            {
                if json["ignore_directories"].is_null() {
                    Some(vec![
                        RelativePath::new("./dist").to_path(&root).to_str().unwrap().to_string(),
                        RelativePath::new("./metrics").to_path(&root).to_str().unwrap().to_string(),
                        RelativePath::new("./target").to_path(&root).to_str().unwrap().to_string(),
                        RelativePath::new("./.git").to_path(&root).to_str().unwrap().to_string(),
                        RelativePath::new("./.github").to_path(&root).to_str().unwrap().to_string(),
                        ]
                    )
                } else {
                    let mut ignore_dirs: Vec<String> = vec![];
                    //TODO: Error when value exists but is not a directory.
                    for dir in json["ignore_directories"].members() {
                        if dir.as_str().unwrap().to_owned().starts_with('/') || dir.as_str().unwrap().to_string().contains(":") {
                            ignore_dirs.push(Path::new(&dir.as_str().unwrap().to_string())
                            .to_owned()
                            .to_str()
                            .unwrap()
                            .to_owned());
                        } else {
                            ignore_dirs.push(RelativePath::new(&dir.as_str().unwrap().to_string())
                                .to_path(&root)
                                .to_str()
                                .unwrap()
                                .to_string());
                        }
                    }
                    if !ignore_dirs.is_empty() {
                        Some(ignore_dirs)
                    } else {
                        Some(vec![String::from("./dist"),String::from("./target"),String::from("./.github"),String::from("./.git"),String::from("./metrics")])
                    }
                }
            }
        );
        debug!("Created new shared config: \n{:#?}", &new_shared_config);
        new_shared_config
    }

    /// Creates a new set of configuration data specific to the top-level of a CIder configuration.
    ///
    /// Parses a JSON file's contents into a set of data that is readable by CIder in order to successfully execute
    /// the instructions provided via said JSON
    ///
    /// ```
    /// use cider::parsing::json_parser;
    /// let config = json_parser::new_top_level("./cider_config.json");
    /// ```
    /// This function will panic when provided with a configuration file that is not found on the host device.
    ///
    pub fn new_top_level(filename: &str) -> TopLevelConfiguration {
        info!("{}", filename);
        let file_contents = fs::read_to_string(filename).unwrap_or_else(|err| {
            eprintln!("{}", err);
            error!(
                "There was an error locating your configuration file: {}",
                err
            );
            panic!("{}", err.to_string());
        });
        debug!("{}", &file_contents);
        let parsed_data = json::parse(&file_contents).unwrap_or_else(|err| {
            eprintln!();
            error!(
                "There was an error parsing your configuration file: {}",
                err
            );
            panic!("{}", err.to_string());
        });
        let s_config = parse_shared_config(&parsed_data);
        let pipeline_defs = {
            if (parsed_data["pipelines"]).is_null() {
                vec![]
            } else {
                parse_json_vector(&parsed_data["pipelines"])
            }
        };
        let pipelines = parse_pipeline_defs(&s_config, &parsed_data, &pipeline_defs);
        let action_defs = {
            if (parsed_data["actions"]).is_null() {
                vec![]
            } else {
                parse_json_vector(&parsed_data["actions"])
            }
        };
        let actions = parse_action_defs(&s_config, &action_defs, &parsed_data);
        TopLevelConfiguration::new(s_config, pipeline_defs, pipelines, action_defs, actions)
    }

    /**
     *
     */
    pub fn overwrite_top_level(
        mut config: TopLevelConfiguration,
        filename: &str,
    ) -> TopLevelConfiguration {
        let file_contents = fs::read_to_string(filename).unwrap_or_else(|err| {
            eprintln!("{}", err);
            error!(
                "There was an error locating your configuration file: {}",
                err
            );
            panic!("{}", err.to_string());
        });
        let parsed_data = json::parse(&file_contents).unwrap_or_else(|err| {
            eprintln!();
            error!(
                "There was an error parsing your configuration file: {}",
                err
            );
            panic!("{}", err.to_string());
        });
        config.s_config = parse_shared_config(&parsed_data);
        config.set_pipeline_defs({
            if (parsed_data["pipelines"]).is_null() {
                vec![]
            } else {
                parse_json_vector(&parsed_data["pipelines"])
            }
        });
        config.set_pipelines(parse_pipeline_defs(
            &config.s_config,
            &parsed_data,
            config.get_pipeline_defs(),
        ));
        config.set_action_defs({
            if (parsed_data["actions"]).is_null() {
                vec![]
            } else {
                parse_json_vector(&parsed_data["actions"])
            }
        });
        config.set_actions(parse_action_defs(
            &config.s_config,
            config.get_action_defs(),
            &parsed_data,
        ));
        config
    }

    ///Created strictly for testing purposes.
    pub fn parse_json_string(filename: &str) -> JsonValue {
        let contents = fs::read_to_string(filename).unwrap();
        let parsed_data = json::parse(&contents);
        // println!("{:#?}", parsed_data.as_ref().unwrap().clone());
        parsed_data.unwrap()
    }
}
