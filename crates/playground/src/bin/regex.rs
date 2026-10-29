use regex::Regex;

fn main() -> Result<(), regex::Error> {
    let re = Regex::new(r"(?<year>\d{4})-(?<month>\d{2})-(?<day>\d{2})")?;
    let text = "released on 2015-05-15, updated 2024-02-22";

    for caps in re.captures_iter(text) {
        println!("{} / {} / {}", &caps["year"], &caps["month"], &caps["day"]);
    }
    println!("{}", re.replace_all(text, "$day.$month.$year"));
    Ok(())
}
