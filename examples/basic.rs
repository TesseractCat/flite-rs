use std::{io::{Cursor, BufWriter}};

use flite_rs::*;
use rodio::{Decoder, OutputStream, source::Source};
use hound::{WavWriter, WavSpec, SampleFormat};

fn main() {
    let custom = Voice::from_file("./examples/cmu_us_eey.flitevox").unwrap();
    let mut slt = Voice::new(BuiltinVoice::Slt);
    slt.duration_stretch = Some(0.6);
    let mut kal = Voice::new(BuiltinVoice::Kal);
    kal.tone_mean = Some(180.);

    let conversation = [
        custom.text_to_speech("Hello world, this is some basic text"),
        slt.text_to_speech("Hi, kal, how are you doing today?"),
        kal.text_to_speech("I'm doing just fine!"),
    ];

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    for data in conversation {
        // Write data to WAV format
        let mut wav_data: Vec<u8> = Vec::new();
        let mut wav_writer = WavWriter::new(BufWriter::new(Cursor::new(&mut wav_data)), WavSpec {
            channels: data.num_channels as u16,
            sample_rate: data.sample_rate as u32,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int
        }).expect("Failed to create wav writer");
        data.samples.iter().copied().for_each(|sample| wav_writer.write_sample(sample).expect("Failed to write sample"));
        wav_writer.finalize().expect("Failed to finalize wav");
    
        // Play on the default audio device
        let source = Decoder::new(Cursor::new(wav_data)).unwrap();
        stream_handle.play_raw(source.convert_samples()).expect("Failed to play");
    
        std::thread::sleep(data.duration());
    }
}