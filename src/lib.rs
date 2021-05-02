use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct AudioMetadata {
    pub sample_rate: u32,
    pub sample_length: u32,
}

#[wasm_bindgen]
pub fn read_metadata(buffer: &[u8]) -> Option<AudioMetadata> {
    let mut reader = buffer;
    if let Ok(reader) = hound::WavReader::new(&mut reader) {
        let spec = reader.spec();

        let sample_rate = spec.sample_rate;
        let sample_length = reader.duration();

        return Some(AudioMetadata {
            sample_rate,
            sample_length,
        });
    }

    if let Ok(meta) = mp3_metadata::read_from_slice(buffer) {
        let sample_rate = meta.frames[0].sampling_freq as u32;

        // im not that proud of this one but that's the best we can do here
        let sample_length = (meta.duration.as_secs_f32() * sample_rate as f32).round() as u32;

        return Some(AudioMetadata {
            sample_rate,
            sample_length,
        });
    }

    None
}
