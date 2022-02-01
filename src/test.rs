fn main() {
    use regex::Regex;
    let re = Regex::new(r"(?i)abb").unwrap();
    let result = re.replace_all("Abbot", "xxx");
    println!("{}", result); // => "xxxxx xxxxx!"
}