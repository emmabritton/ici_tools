# ICI Tools

Various tools for working with ICI files

## Usage

### General

```
ici_tools <COMMAND>

Commands:
  convert  Convert PNGs, JPGs, etc to ICIs. Animation is not supported
  view     Display ICI or ICA file
  palette  Read or alter palettes
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

### Convert

Converts image files (PNGs, JPGs, etc) to ICIs

```
ici_tools convert [OPTIONS] <FILE>

Arguments:
  <FILE>  Source image file (png, bmp, etc)

Options:
  -o, --output <FILE>  Output file name
  -h, --help           Print help

```

Example: `./ici_tools convert icon.png`

### View

Displays static or animated images

```
ici_tools view [OPTIONS] <FILE>

Arguments:
  <FILE>  Image file to view

Options:
  -p, --palette <FILE>  Replacement palette
  -h, --help            Print help

```

Example: `./ici_tools view image.ici`

### Palette

#### Extract

Save palette from image

```
ici_tools palette extract [OPTIONS] <FILE>

Arguments:
  <FILE>  Image file

Options:
  -o, --output <FILE>  Palette file name
  -h, --help           Print help
```

Example: `./ici_tools palette extract image.ici -o orig.pal`

#### Set

Set palette for images 

```
 ici_tools palette set [OPTIONS] <FILE>

Arguments:
  <FILE>  Image or palette source file

Options:
  -o, --output <FILE>  ICI files to update
  -h, --help           Print help
```

Example: `./ici_tools palette set new.pal -o image1.ici -o image2.ici`