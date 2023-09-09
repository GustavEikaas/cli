use std::fs;
mod utils;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use serde_derive::Deserialize;
use std::collections::HashMap;

use utils::ansi_log;
use utils::parse_args::parse_cli_args;

fn main() {
    let arg1 = parse_cli_args();

    let package_json = try_read_package_json(arg1);
    // ansi_print("Package json successfully located", ansi_term::Color::Green);
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

fn print_table(headers: Vec<&str>, data: Vec<Vec<&str>>) {
    // Calculate the maximum width for each column
    let col_widths: Vec<usize> = headers
        .iter()
        .enumerate()
        .map(|(i, header)| {
            let data_widths: Vec<usize> = data.iter().map(|row| row[i].len()).collect();
            let max_data_width = data_widths.into_iter().max().unwrap_or(0);
            let header_width = header.len();
            max_data_width.max(header_width)
        })
        .collect();

    // Print the headers
    for (i, header) in headers.iter().enumerate() {
        print!("{:<width$} | ", header, width = col_widths[i]);
    }
    println!();

    // Print a horizontal line to separate headers and data
    for &width in &col_widths {
        print!("{:-<width$}-+-", "", width = width);
    }
    println!();

    // Print the data rows
    for row in data {
        for (i, cell) in row.iter().enumerate() {
            print!("{:<width$} | ", cell, width = col_widths[i]);
        }
        println!();
    }
}

#[derive(Debug, Deserialize)]
struct RawPackageJson {
    name: Option<String>,
    version: Option<String>,
    dependencies: Option<std::collections::HashMap<String, String>>,
}

struct PackageJson {
    name: String,
    version: String,
    dependencies: HashMap<String, String>,
}
fn try_read_package_json(path: String) -> PackageJson {
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
