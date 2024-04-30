pub mod json_parser {

    use crate::config::config_structs::*;
    use crate::libs::errors::{output_error, CustomError};
    use json::JsonValue;
    use log::warn;
    use relative_path::RelativePath;
    use shared_config::ShareableConfiguration;
    use std::env::current_dir;
    use std::path::{Path, PathBuf};
    use std::{collections::HashMap, fs};
    use std::fmt::Error;
    /// Parses a map of JSON information into a HashMap<String,String>
    ///
    /// Iterates through a JSON hashmap and parses its data into a HashMap<String,String>
    ///
    fn parse_json_map(json: &JsonValue) -> HashMap<String, String> {
        // println!("{:#?}", json);
        let mut map = HashMap::new();
        for key_value in json.entries() {
            map.insert(key_value.0.to_string(), key_value.1.to_string());
        }
        // println!("{:#?}", json);
        if map.is_empty() {
            warn!("No mappable values found in json hashmap {:#?}", json);
            return map;
        }
        map
    }
    /// Take an input file name and parse the file to a JsonValue
    /// 
    pub fn file_to_json(filename: &str) -> JsonValue {
        let file_contents = fs::read_to_string(filename).unwrap_or_else(|err| {
            output_error(CustomError::PARSINGERROR(Box::new(err)));
            panic!("How did this even get triggered? I just put this in to make the compiler happy.")
        });
        let parsed_data = json::parse(&file_contents).unwrap_or_else(|err| {
            output_error(CustomError::PARSINGERROR(Box::new(err)));
            panic!("How did this even get triggered? I just put this in to make the compiler happy.")
        });
        return parsed_data;
    }

    fn parse_shared_config(json: &JsonValue) -> ShareableConfiguration {
        let root = &current_dir().unwrap();
        // let backend = check_backend(json);
        ShareableConfiguration::new(
            get_mapped_property(json, "metadata"),
            get_property(json, "title"),
            get_mapped_property(json, "tags"),
            get_property(json, "language").unwrap(),
            get_property(json, "image"),
            get_property(json, "backend").unwrap(),
            get_path_as_string(get_property(json, "output_directory"), root, true),
            get_path_as_string(get_property(json, "source_directory"), root, false),
        )
    }

    fn get_property(json: &JsonValue, property: &str) -> Option<String> {
        if json[property].is_null() {
            match property {
                "backend" => {
                    Some("bash".to_string())
                },
                "language" => {
                    Some("Python".to_string())
                },
                _ => None
            }
        } else {
            match property {
                _ => Some(json[property].to_string())

            }
        }
    }

    fn get_mapped_property(json: &JsonValue, property: &str) -> Option<HashMap<String, String>> {
        if json[property].is_null() {
            None
        } else {
            Some(parse_json_map(&json[property]))
        }
    }

    fn get_path_as_string(path: Option<String>, root: &PathBuf, is_output: bool) -> String {
        if is_output {
            match path {
                Some(path) => {
                    RelativePath::new(&path)
                        .to_path(root)
                        .to_str()
                        .unwrap()
                        .to_string()
                }
                None => {
                    RelativePath::new("./dist/cider")
                        .to_path(root)
                        .to_str()
                        .unwrap()
                        .to_string()
                }
            }
        } else {
            match path {
                Some(path) => {
                    if path.starts_with('/') || path.contains(":") {
                        Path::new(&path)
                            .to_owned()
                            .to_str()
                            .unwrap()
                            .to_owned()
                    } else {
                        RelativePath::new(&path)
                            .to_path(&root)
                            .to_str()
                            .unwrap()
                            .to_string()
                    }
                }
                None => {
                    RelativePath::new("./")
                        .to_path(&root)
                        .to_str()
                        .unwrap()
                        .to_string()
                }
            }
        }
    }


    #[cfg(test)]
    mod tests {
        use crate::libs::parsing_new::json_parser::*;
        #[test]
        #[should_panic]
        fn test_json_parse_failure() {
            let result = file_to_json("Pathtofilethatdoesnnotexist");
        }
    }
}




