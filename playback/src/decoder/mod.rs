use thiserror::Error;

use crate::metadata::audio::AudioFileFormat;

mod passthrough_decoder;
pub use passthrough_decoder::PassthroughDecoder;

mod symphonia_decoder;
pub use symphonia_decoder::SymphoniaDecoder;

#[derive(Error, Debug)]
pub enum DecoderError {
    #[error("Passthrough Decoder Error: {0}")]
    PassthroughDecoder(String),
    #[error("Symphonia Decoder Error: {0}")]
    SymphoniaDecoder(String),
}

pub type DecoderResult<T> = Result<T, DecoderError>;

#[derive(Error, Debug)]
pub enum AudioPacketError {
    #[error("Decoder Raw Error: Can't return Raw on Samples")]
    Raw,
    #[error("Decoder Samples Error: Can't return Samples on Raw")]
    Samples,
}

pub type AudioPacketResult<T> = Result<T, AudioPacketError>;

pub enum AudioPacket {
    Samples(Vec<f64>),
    Raw(Vec<u8>),
}

impl AudioPacket {
    pub fn samples(&self) -> AudioPacketResult<&[f64]> {
        match self {
            AudioPacket::Samples(s) => Ok(s),
            AudioPacket::Raw(_) => Err(AudioPacketError::Raw),
        }
    }

    pub fn oggdata(&self) -> AudioPacketResult<&[u8]> {
        match self {
            AudioPacket::Raw(d) => Ok(d),
            AudioPacket::Samples(_) => Err(AudioPacketError::Samples),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            AudioPacket::Samples(s) => s.is_empty(),
            AudioPacket::Raw(d) => d.is_empty(),
        }
    }
}

pub trait AudioDecoder {
    fn seek(&mut self, absgp: u64) -> Result<u64, DecoderError>;
    fn next_packet(&mut self) -> DecoderResult<Option<AudioPacket>>;

    fn is_ogg_vorbis(format: AudioFileFormat) -> bool
    where
        Self: Sized,
    {
        matches!(
            format,
            AudioFileFormat::OGG_VORBIS_320
                | AudioFileFormat::OGG_VORBIS_160
                | AudioFileFormat::OGG_VORBIS_96
        )
    }

    fn is_mp3(format: AudioFileFormat) -> bool
    where
        Self: Sized,
    {
        matches!(
            format,
            AudioFileFormat::MP3_320
                | AudioFileFormat::MP3_256
                | AudioFileFormat::MP3_160
                | AudioFileFormat::MP3_96
        )
    }
}

impl From<symphonia::core::errors::Error> for DecoderError {
    fn from(err: symphonia::core::errors::Error) -> Self {
        Self::SymphoniaDecoder(err.to_string())
    }
}
