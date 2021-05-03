use std::io::Cursor;

use claxon::{FlacReader, FlacReaderOptions};
use lewton::audio::get_decoded_sample_count;
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

    let mut reader = ogg::PacketReader::new(Cursor::new(buffer));
    if let Ok(((ident_hdr, _, setup_hdr), _)) = lewton::inside_ogg::read_headers(&mut reader) {
        let sample_rate = ident_hdr.audio_sample_rate;
        let mut sample_length = 0;

        while let Ok(Some(packet)) = reader.read_packet() {
            sample_length +=
                get_decoded_sample_count(&ident_hdr, &setup_hdr, &packet.data).unwrap_or(0) as u32;
        }

        return Some(AudioMetadata {
            sample_rate,
            sample_length,
        });
    }

    let reader = buffer;
    if let Ok(reader) = FlacReader::new_ext(
        reader,
        FlacReaderOptions {
            metadata_only: true,
            read_vorbis_comment: false,
        },
    ) {
        let meta = reader.streaminfo();

        return Some(AudioMetadata {
            sample_rate: meta.sample_rate,
            sample_length: meta.samples.unwrap_or(0) as u32,
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
