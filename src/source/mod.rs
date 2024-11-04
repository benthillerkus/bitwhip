use anyhow::Result;
use ffmpeg_next::frame::video::Video;

#[cfg(target_os = "windows")]
pub mod dxdup;

pub mod static_image;

pub trait Source {
    fn get_frame(&mut self) -> Result<Video>;
}
