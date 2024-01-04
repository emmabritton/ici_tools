use clap::Parser;
use pixels_graphics_lib::buffer_graphics_lib::Graphics;
use pixels_graphics_lib::prelude::{IndexedImage, KeyCode, Timing, WHITE};
use pixels_graphics_lib::{run, Options, System};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Displays ICI files", long_about = None)]
struct Args {
    #[arg(value_name = "FILE", help = "Image to show")]
    input: PathBuf,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let filename = args
        .input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Image");
    let file = File::open(&args.input)?;
    let mut reader = BufReader::new(file);
    let mut bytes = vec![];
    reader.read_to_end(&mut bytes)?;
    let image = IndexedImage::from_file_contents(&bytes)?.0;

    run(
        image.width() as usize,
        image.height() as usize,
        filename,
        ImageDisplayer::new(image),
        Options::default(),
    )?;

    Ok(())
}

#[derive(Debug)]
struct ImageDisplayer {
    image: IndexedImage,
    exit: bool,
}

impl ImageDisplayer {
    pub fn new(image: IndexedImage) -> Box<ImageDisplayer> {
        Box::new(Self { image, exit: false })
    }
}

impl System for ImageDisplayer {
    fn update(&mut self, _timing: &Timing) {
        //nothing
    }

    fn render(&mut self, graphics: &mut Graphics) {
        graphics.clear(WHITE);
        graphics.draw_indexed_image((0, 0), &self.image);
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
