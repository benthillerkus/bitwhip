use super::Source;
use anyhow::{anyhow, Result};
use ffmpeg_next::{
    filter::{self, Graph},
    frame,
};
use ffmpeg_sys_next::AVFrame;
use image::GenericImageView;

pub struct ImageFile {}

impl ImageFile {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}

impl Source for ImageFile {
    fn get_frame(&mut self) -> Result<frame::Video> {
        let img = image::open("EBU_Colorbars.png")
            .map_err(|e| anyhow!("Failed to open image: {}", e))?;
        let (width, height) = img.dimensions();
        let buffer = img.to_rgb8();
        let mut frame = frame::Video::new(
            ffmpeg_next::format::Pixel::RGB24,
            (width as i32)
                .try_into()
                .expect("Image width to fit in an i32"),
            (height as i32)
                .try_into()
                .expect("Image height to fit in an i32"),
        );
        frame.data_mut(0).copy_from_slice(&buffer);

        let mut graph = Graph::new();
        graph.add(
            &filter::find("buffer").unwrap(),
            "in",
            &format!(
                "video_size={}x{}:pix_fmt=rgb24:time_base=1/1:pixel_aspect=1/1",
                width, height
            ),
        )?;
        graph.add(&filter::find("buffersink").unwrap(), "out", "")?;
        graph
            .output("in", 0)?
            .input("out", 0)?
            .parse("format=pix_fmts=yuv420p")?;
        graph.validate()?;

        graph
            .get("in")
            .expect("Graph to have stage 'in'")
            .source()
            .add(&frame)?;

        let mut frame = frame::Video::empty();
        graph
            .get("out")
            .expect("Graph to have stage 'out'")
            .sink()
            .frame(&mut frame)?;

        Ok(frame)
    }
}
