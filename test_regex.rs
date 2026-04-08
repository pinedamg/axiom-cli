use regex::Regex;
fn main() {
    let re = Regex::new(r"\w+").unwrap();
    let text = "hello world";
    let rep = re.replace_all(text, |caps: &regex::Captures| {
        let word = &caps[0];
        if word == "world" {
            "[REDACTED]"
        } else {
            word
        }
    });
    println!("{}", rep);
}
