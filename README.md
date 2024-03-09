# wall-util

A helpful utility to have your wallpaper changed automatically.

## Installation

You can either manually build the project, or download the pre-built binaries.

### Manual Build

```bash
git clone https://gitlab.com/lydia-st/wall-util.git
cd wall-util
cargo build --release
```

### Pre-built Binaries

Navigate to the [Artifacts](https://gitlab.com/lydia-st/wall-util/-/artifacts) page and download the latest successful 
pipeline's artifacts, as shown in the image below:

![A screenshot of a computer screen showing a list of artifacts titled "Artifacts". The total size of the artifacts is 1.23 MiB. There are two artifacts listed: "wall-util_from_master.zip archive" (size: 1.22 MiB, created 1 minute ago) and "metadata.gz" (size: 190 B).](assets/artifact.png)

Then, extract the zip and find the binary. You can either run it from the terminal, or move it to a directory in your 
`$PATH` to run it from anywhere. You might also need to make it executable with `chmod +x ./wall-util`.

## Usage

### Prerequisites

- [swww](https://github.com/LGFae/swww) - Efficient animated wallpaper daemon for wayland, controlled at runtime
- [Pywal](https://github.com/dylanaraps/pywal) - Generate and change color-schemes on the fly.
- [Pywalfox](https://github.com/Frewacom/pywalfox) -  Dynamic theming of Firefox (and Thunderbird) using your Pywal colors

### Help

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


