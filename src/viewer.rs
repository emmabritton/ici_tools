use std::fs;
use pixels_graphics_lib::buffer_graphics_lib::Graphics;
use pixels_graphics_lib::prelude::{KeyCode, Timing, WHITE};
use pixels_graphics_lib::{run, Options, System};
use std::path::PathBuf;
use color_eyre::Result;
use pixels_graphics_lib::prelude::*;
use crate::wrapper::*;

pub fn view(input: PathBuf, palette_file: Option<PathBuf>) -> Result<()> {

    let (bytes, filename) = open_ici_file(input)?;
    let filename = filename.unwrap_or(String::from("Image"));

    let mut image = load(bytes, palette_file.is_none())?;

    if let Some(path) = palette_file {
        let str = fs::read_to_string(path)?;
        let palette = JascPalette::from_file_contents(&str)?;
        image.set_palette(&palette.colors)?;
    }

    run(
        image.width() as usize,
        image.height() as usize,
        &filename,
        ImageDisplayer::new(image),
        Options::default(),
    )?;

    Ok(())
}

#[derive(Debug)]
struct ImageDisplayer {
    image: IndexedWrapper,
    exit: bool,
}

impl ImageDisplayer {
    pub fn new(image: IndexedWrapper) -> Box<ImageDisplayer> {
        Box::new(Self { image, exit: false })
    }
}

impl System for ImageDisplayer {
    fn update(&mut self, timing: &Timing) {
        self.image.update(timing.fixed_time_step);
    }

    fn render(&mut self, graphics: &mut Graphics) {
        graphics.clear(WHITE);
        graphics.draw_wrapped_image((0,0), &self.image);
    }

    fn on_key_down(&mut self, keys: Vec<KeyCode>) {
        if keys.contains(&KeyCode::Escape) {
            self.exit = true;
        }
    }

    fn should_exit(&mut self) -> bool {
        self.exit
    }
}
