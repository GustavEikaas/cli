mod utils;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;
use utils::read_package_json::PackageJson;
use utils::table_print::print_table;

use utils::parse_args::parse_cli_args;

use terminal_utils::confirm::confirm;

use crate::utils::read_package_json::try_read_package_json;

fn main() {
    let arg1 = parse_cli_args();

    let should_continue = confirm();
    if should_continue {
        println!("Lets proceed");
    }

    let package_json = try_read_package_json(arg1);

    print!(
        "Parsing {0} dependencies in {1}@{2}",
        package_json.dependencies.len(),
        package_json.name,
        package_json.version
    );
    print_depdencies(package_json);
}

fn print_depdencies(package_json: PackageJson) {
    let headers = vec!["Name", "Version"];
    let data = hashmap_to_vec_vec(&package_json.dependencies);

    print_table(headers, data);
}

fn hashmap_to_vec_vec(hashmap: &HashMap<String, String>) -> Vec<Vec<&str>> {
    hashmap
        .iter()
        .map(|(key, value)| vec![key.as_str(), value.as_str()])
        .collect()
}
