use rkyv::util::AlignedVec;
use rkyv::{Archive, Deserialize, Serialize};
use std::error::Error;
use std::fs;

#[derive(Archive, Serialize, Deserialize, Debug)]
struct Reading {
    sensor: String,
    celsius: f32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let readings = vec![
        Reading {
            sensor: "kitchen".to_string(),
            celsius: 21.5,
        },
        Reading {
            sensor: "garage".to_string(),
            celsius: 8.25,
        },
    ];

    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&readings)?;
    fs::write("readings.rkyv", &bytes)?;

    // `fs::read` gives no alignment guarantee, so copy into an `AlignedVec`.
    let mut buffer = AlignedVec::<16>::new();
    buffer.extend_from_slice(&fs::read("readings.rkyv")?);

    let archived = rkyv::access::<rkyv::Archived<Vec<Reading>>, rkyv::rancor::Error>(&buffer)?;
    for reading in archived.iter() {
        println!("{}: {:.2} °C", reading.sensor, reading.celsius.to_native());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn runs() -> Result<(), Box<dyn std::error::Error>> {
        let dir = std::env::temp_dir().join("rkyv-example-file");
        std::fs::create_dir_all(&dir)?;
        std::env::set_current_dir(&dir)?;
        super::main()
    }
}
