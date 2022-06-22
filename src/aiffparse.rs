use extended::Extended;
use nom::{IResult, combinator::rest, sequence::{preceded, tuple}, branch::alt};
use nom::number::complete::{be_i32, be_i16, be_u32};
use nom::bytes::complete::{take_until, take, tag};

const AIFF_MAGIC_COMMON: [u8;4] = [0x43, 0x4F, 0x4D, 0x4D];
const AIFF_MAGIC: [u8;4] = [0x41, 0x49, 0x46, 0x46];
const AIFC_MAGIC: [u8;4] = [0x41, 0x49, 0x46, 0x43];

#[derive(Debug)]
pub struct AiffCommonChunk {
    pub ck_size: i32,
    pub num_channels: i16,
    pub num_frames: u32,
    pub sample_size: i16,
    pub sample_rate: Extended,
}

fn extended_from_slice(s:&[u8]) -> Extended {
    let mut a = [0; 10];
    a.copy_from_slice(s);
    Extended::from_be_bytes(a)
}

pub fn parse_common(d:&[u8]) -> IResult<&[u8], AiffCommonChunk> {
    let (_, d) = preceded(
        preceded(
            alt((take_until(AIFF_MAGIC.as_slice()), take_until(AIFC_MAGIC.as_slice()))),
            take_until(AIFF_MAGIC_COMMON.as_slice()),
        ),
        rest
    )(d)?;

    let (input, (_, ck_size, num_channels, num_frames, sample_size, sample_rate)) =
        tuple((tag(AIFF_MAGIC_COMMON), be_i32, be_i16, be_u32, be_i16, take(10usize)))(d)?;

    Ok((input, AiffCommonChunk {
        ck_size,
        num_channels,
        num_frames,
        sample_size,
        sample_rate: extended_from_slice(sample_rate)
    }))
}
