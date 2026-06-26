use std::fs::File;
use std::path::Path;

use anyhow::{Result, anyhow};
use symphonia::core::codecs::CodecParameters;
use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatOptions, TrackType};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

struct Decoded {
    sample_rate: u32,
    channels: usize,
    frames: u64,
}

fn decode(path: &Path) -> Result<Decoded> {
    let file = File::open(path)?;
    let stream = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("wav");
    let mut format = symphonia::default::get_probe().probe(
        &hint,
        stream,
        FormatOptions::default(),
        MetadataOptions::default(),
    )?;

    let track = format
        .default_track(TrackType::Audio)
        .ok_or_else(|| anyhow!("no audio track"))?;
    let track_id = track.id;
    let params = match track.codec_params.as_ref() {
        Some(CodecParameters::Audio(params)) => params.clone(),
        _ => return Err(anyhow!("track is not audio")),
    };
    let sample_rate = params.sample_rate.unwrap_or(0);
    let channels = params.channels.as_ref().map(|c| c.count()).unwrap_or(0);

    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(&params, &AudioDecoderOptions::default())?;

    let mut frames = 0u64;
    while let Some(packet) = format.next_packet()? {
        if packet.track_id != track_id {
            continue;
        }
        let buffer = decoder.decode(&packet)?;
        frames += buffer.frames() as u64;
    }

    Ok(Decoded {
        sample_rate,
        channels,
        frames,
    })
}

fn main() -> Result<()> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/tone.wav");
    let audio = decode(&path)?;

    let seconds = audio.frames as f64 / audio.sample_rate as f64;
    println!(
        "{} channels at {} Hz, {} frames ({:.2}s)",
        audio.channels, audio.sample_rate, audio.frames, seconds
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_tone_track() -> Result<()> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/tone.wav");
        let audio = decode(&path)?;
        assert_eq!(audio.sample_rate, 44_100);
        assert_eq!(audio.channels, 1);
        assert!(audio.frames > 0);
        Ok(())
    }
}
