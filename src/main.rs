use std::fs;

fn parse_cli_args() -> String{
    let arg1 = match std::env::args().nth(1){
        Some(x) => if x.contains("package.json") {x} else {( x + "/package.json").to_string()},
        None => String::from("./package.json"),
    };
    return arg1;
}


fn main() {
    let arg1 = parse_cli_args();
    println!("Checking package.json in location {arg1}");
    let package_json = try_read_package_json(arg1);
    print!("Parsing {0}", package_json.name)
}


struct PackageJson{
    name: String,
}
fn try_read_package_json(path: String) -> PackageJson{
    let contents = fs::read_to_string(path)
    .expect("Should have been able to read the file");

    let person: Result<PackageJson, ()> = serde_json::from_str(contents.as_str()).unwrap();

    println!("With text:\n{contents}");
    return {
        PackageJson { name: "".to_string() }
    }
}