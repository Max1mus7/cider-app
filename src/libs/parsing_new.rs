pub mod json_parser {

    use crate::config::config_structs::*;
    use crate::libs::errors::{output_error, CustomError};
    use json::JsonValue;
    use log::warn;
    use relative_path::RelativePath;
    use shared_config::ShareableConfiguration;
    use std::env::current_dir;
    use std::path::Path;
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
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        #[should_panic]
        fn test_json_parse_failure() {
            let result =file_to_json("Pathtofilethatdoesnnotexist");
        }
    }
}

