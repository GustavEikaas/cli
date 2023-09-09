pub(crate) fn parse_cli_args() -> String {
    let arg1 = match std::env::args().nth(1) {
        Some(x) => {
            if x.contains("package.json") {
                x
            } else {
                (x + "/package.json").to_string()
            }
        }
        None => String::from("./package.json"),
    };
    return arg1;
}
