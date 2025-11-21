use anyhow::anyhow;
use ffmpeg_next::{self as ffmpeg, decoder, format::context::Input};

pub struct VideoInput {
    pub ictx: Input,
    pub decoder: decoder::Video,
    pub video_stream_index: usize,
}

impl VideoInput {
    pub fn by_path(file_path: &str) -> anyhow::Result<VideoInput> {
        ffmpeg::init().unwrap();

        let ictx = ffmpeg::format::input(&file_path)?;

        // Find the video stream
        let input = ictx
            .streams()
            .best(ffmpeg::media::Type::Video)
            .ok_or_else(|| anyhow!("No video stream found"))?;
        let video_stream_index = input.index();
        let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
        let decoder = context_decoder.decoder().video()?;

        Ok(VideoInput {
            ictx,
            decoder,
            video_stream_index,
        })
    }
}
