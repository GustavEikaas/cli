use std::{collections::HashMap, fs};

use serde_derive::Deserialize;

use super::ansi_log;

#[derive(Debug, Deserialize)]
struct RawPackageJson {
    name: Option<String>,
    version: Option<String>,
    dependencies: Option<std::collections::HashMap<String, String>>,
}

pub struct PackageJson {
    pub name: String,
    pub version: String,
    pub dependencies: HashMap<String, String>,
}
pub(crate) fn try_read_package_json(path: String) -> PackageJson {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let package_json: RawPackageJson =
        serde_json::from_str(&contents).expect("Unable to parse JSON.");

    let sanitized_package_json: PackageJson = PackageJson {
        name: match package_json.name {
            Some(x) => x,
            None => {
                ansi_log::ansi_print("No name in package json", ansi_term::Color::Yellow);
                "Unknown".to_string()
            }
        },
        version: match package_json.version {
            Some(x) => x,
            None => {
                ansi_log::warning("No version in package json");
                "Unknown".to_string()
            }
        },
        dependencies: package_json.dependencies.unwrap_or(HashMap::default()),
    };

    sanitized_package_json
}
