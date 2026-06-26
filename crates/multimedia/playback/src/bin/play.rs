use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;

fn main() -> Result<()> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/tone.wav");

    let mut stream = rodio::DeviceSinkBuilder::open_default_sink()?;
    let file = BufReader::new(File::open(&path)?);

    let player = rodio::play(stream.mixer(), file)?;
    player.sleep_until_end();

    stream.log_on_drop(false);
    Ok(())
}
