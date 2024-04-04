mod converter;
mod palette_read;
mod palette_set;
mod png;
mod viewer;
mod wrapper;

use crate::converter::convert_format;
use crate::palette_read::palette_extract;
use crate::palette_set::palette_set;
use crate::png::to_png;
use crate::viewer::view;
use clap::{Parser, Subcommand};
use color_eyre::Result;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Convert PNGs, JPGs, etc to ICIs. Animation is not supported
    Convert {
        #[arg(value_name = "FILE", help = "Source image file (png, bmp, etc)")]
        input: PathBuf,
        #[arg(
            short = 'o',
            long = "output",
            value_name = "FILE",
            help = "Output file name"
        )]
        output: Option<PathBuf>,
    },
    /// Convert ICI to PNG. Animation is not supported
    Png {
        #[arg(value_name = "FILE", help = "Source image file (ici)")]
        input: PathBuf,
        #[arg(
            short = 'o',
            long = "output",
            value_name = "FILE",
            help = "Output file name"
        )]
        palette: Option<PathBuf>,
        #[arg(
            short = 'p',
            long = "palette",
            value_name = "FILE",
            help = "Palette file name"
        )]
        output: Option<PathBuf>,
    },
    /// Display ICI or ICA file
    View {
        #[arg(value_name = "FILE", help = "Image file to view")]
        input: PathBuf,
        #[arg(
            short = 'p',
            long = "palette",
            value_name = "FILE",
            help = "Replacement palette"
        )]
        palette: Option<PathBuf>,
    },
    /// Read or alter palettes
    Palette {
        #[command(subcommand)]
        command: PaletteCommands,
    },
}

#[derive(Subcommand, Debug)]
enum PaletteCommands {
    /// Copy palette and save to file
    Extract {
        #[arg(value_name = "FILE", help = "Image file")]
        input: PathBuf,
        #[arg(
            short = 'o',
            long = "output",
            value_name = "FILE",
            help = "Palette file name"
        )]
        output: Option<PathBuf>,
    },
    /// Copy palette from one file and apply to others
    Set {
        #[arg(value_name = "FILE", help = "Image or palette source file")]
        input: PathBuf,
        #[arg(
            short = 'o',
            long = "output",
            value_name = "FILE",
            help = "ICI files to update"
        )]
        files: Vec<PathBuf>,
    },
}

pub fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    match args.command {
        Commands::Convert { input, output } => convert_format(input, output)?,
        Commands::View { input, palette } => view(input, palette)?,
        Commands::Png {
            input,
            palette,
            output,
        } => to_png(input, palette, output)?,
        Commands::Palette { command } => match command {
            PaletteCommands::Extract { input, output } => palette_extract(input, output)?,
            PaletteCommands::Set { input, files } => palette_set(input, files)?,
        },
    }

    Ok(())
}
