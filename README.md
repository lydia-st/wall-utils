# wall-util

A helpful utility to have your wallpaper changed automatically.

## Installation

You can either manually build the project, or (soon) download the pre-built binaries.

### Manual Build

```bash
git clone https://gitlab.com/lydia-st/wall-util.git
cd wall-util
cargo build --release
```

## Usage

```bash

wall-util --help
  wall-util 0.1.0
  A wallpaper setter written in Rust, using pywal and swww
  
  USAGE:
      wall-util [OPTIONS] --path <path>...
  
  FLAGS:
      -h, --help       Prints help information
      -V, --version    Prints version information
  
  OPTIONS:
      -c, --config <config>        The path to a config file containing the interval and paths. NOTE: This is not yet
                                   implemented
      -i, --interval <interval>    How often to change the wallpaper, in seconds, minutes, hours, days, or weeks. Example:
                                   15m, 1h, 2d, 1w [default: 15m]
      -p, --path <path>...         The path to the image or directories containing images to use as wallpapers. You can
                                   provide up to three paths, one for each time of day: morning, afternoon, and evening
```

### Time-based Wallpapers

`wall-utils` lets you set different wallpapers for different times of the day. You can provide up to three paths, 
one for each time of day: morning, afternoon, and evening, eg.:

`--path ~/Pictures/morning ~/Pictures/afternoon ~/Pictures/evening`

Where morning is from 06:00 to 12:00, afternoon is from 12:00 to 18:00, and evening is from 18:00 to 06:00.
Once support for config files is added, you will be able to set your own times and do this in a more flexible, 
_probably_ more user-friendly way :)


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details


