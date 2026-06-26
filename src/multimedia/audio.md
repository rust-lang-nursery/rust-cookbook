## Decode audio samples from a file

[![symphonia-badge]][symphonia] [![cat-multimedia-badge]][cat-multimedia]

Measuring a clip's length or feeding it to an analyzer means decoding its samples,
whatever the container. [`symphonia`][symphonia] probes the format, then turns
packets into audio buffers. [`get_probe`] identifies the container from a
[`Hint`] and a [`MediaSourceStream`], returning a [`FormatReader`].
[`default_track`] picks the audio track and [`get_codecs`] builds a decoder for
its [`CodecParameters`]. Each packet from [`next_packet`] goes to [`Decoder::decode`],
whose buffer reports its frame count. The `wav` feature compiles in only the WAV
reader; enable the features matching the formats you need.

```rust,no_run
{{#include ../../crates/multimedia/audio/src/bin/decode.rs::71}}
```

[`get_probe`]: https://docs.rs/symphonia/*/symphonia/default/fn.get_probe.html
[`Hint`]: https://docs.rs/symphonia-core/*/symphonia_core/formats/probe/struct.Hint.html
[`MediaSourceStream`]: https://docs.rs/symphonia-core/*/symphonia_core/io/struct.MediaSourceStream.html
[`FormatReader`]: https://docs.rs/symphonia-core/*/symphonia_core/formats/trait.FormatReader.html
[`default_track`]: https://docs.rs/symphonia-core/*/symphonia_core/formats/trait.FormatReader.html#method.default_track
[`get_codecs`]: https://docs.rs/symphonia/*/symphonia/default/fn.get_codecs.html
[`CodecParameters`]: https://docs.rs/symphonia-core/*/symphonia_core/codecs/enum.CodecParameters.html
[`next_packet`]: https://docs.rs/symphonia-core/*/symphonia_core/formats/trait.FormatReader.html#tymethod.next_packet
[`Decoder::decode`]: https://docs.rs/symphonia-core/*/symphonia_core/codecs/audio/trait.AudioDecoder.html#method.decode

## Play an audio file

[![rodio-badge]][rodio] [![cat-multimedia-badge]][cat-multimedia]

Playing a notification sound means opening an output device and handing it a
decoded source. [`rodio`][rodio] does both.
[`DeviceSinkBuilder::open_default_sink`] opens the system's default output,
[`play`] decodes the file and starts it on the device's [`mixer`], and
[`Player::sleep_until_end`] blocks until playback finishes. Dropping the sink
stops anything still playing, so rodio warns about it on drop;
[`MixerDeviceSink::log_on_drop`] turns the warning off once the wait has returned.

```rust,no_run
{{#include ../../crates/multimedia/playback/src/bin/play.rs}}
```

[`DeviceSinkBuilder::open_default_sink`]: https://docs.rs/rodio/*/rodio/stream/struct.DeviceSinkBuilder.html#method.open_default_sink
[`play`]: https://docs.rs/rodio/*/rodio/stream/fn.play.html
[`mixer`]: https://docs.rs/rodio/*/rodio/stream/struct.MixerDeviceSink.html#method.mixer
[`MixerDeviceSink::log_on_drop`]: https://docs.rs/rodio/*/rodio/stream/struct.MixerDeviceSink.html#method.log_on_drop
[`Player::sleep_until_end`]: https://docs.rs/rodio/*/rodio/player/struct.Player.html#method.sleep_until_end

{{#include ../links.md}}
