pub fn confirm() -> bool {
    println!("Do you want to continue? (Y/N)");
    let mut word = String::from(String::new());
    std::io::stdin().read_line(&mut word).unwrap();
    word = word.trim().to_owned();
    if word == "Y" {
        return true;
    } else if word == ("N") {
        return false;
    } else {
        return confirm();
    }
}
