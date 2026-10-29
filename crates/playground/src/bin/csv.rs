fn main() -> Result<(), csv::Error> {
    let data = "name,year\nRust,2015\nGo,2009\n";
    let mut reader = csv::Reader::from_reader(data.as_bytes());

    for record in reader.records() {
        let record = record?;
        println!("{} first appeared in {}", &record[0], &record[1]);
    }
    Ok(())
}
