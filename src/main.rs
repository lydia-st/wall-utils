use std::path::PathBuf;
use serde_derive::Deserialize;
use structopt::StructOpt;
use std::process::Command;
use std::thread::sleep;
use chrono;

// TODO: Support loading configs from file

#[derive(Debug, StructOpt)]
#[structopt(name = "wall-util", about = "A wallpaper setter written in Rust, using pywal and swww")]
struct Opt {

    /// How often to change the wallpaper, in seconds, minutes, hours, days, or weeks.
    /// Example: 15m, 1h, 2d, 1w
    #[structopt(short, long, default_value = "15m")]
    interval: String,

    /// The path to the image or directories containing images to use as wallpapers.
    /// You can provide up to three paths, one for each time of day: morning, afternoon, and evening.
    #[structopt(short, long, parse(from_os_str), required = true, min_values = 1, max_values = 3)]
    path: Vec<PathBuf>,

    // /// The path to a config file containing the interval and paths.
    // #[structopt(short, long, parse(from_os_str))]
    // config: Option<PathBuf>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Config {
    interval: String,
    path: Vec<PathBuf>,
}

fn pick_image(path: &PathBuf) -> PathBuf {
    let image = Command::new("sh")
        .arg("-c")
        .arg(format!("ls {} | shuf -n 1", path.to_str().unwrap()))
        .output()
        .expect("failed to execute process");


    let image = String::from_utf8(image.stdout).unwrap();
    let image = image.trim();

    let image = path.join(image);

    if !image.exists() {
        panic!("Image does not exist: {:?}", image);
    }

    image
}

fn verify_paths(paths: Vec<PathBuf>) {
    for path in paths {
        if !path.exists() {
            panic!("Path does not exist: {:?}", path);
        }
    }
}

fn check_dependencies() {
    println!("checking dependencies");
    let dependencies = vec!["swww", "wal", "pywalfox"];
    let mut missing_dependencies = vec![];

    for dependency in dependencies {
        let output = Command::new("which")
            .arg(dependency)
            .output()
            .expect("failed to execute process");

        if !output.status.success() {
            missing_dependencies.push(dependency);
            println!("    missing   {:?}", dependency);
        } else {
            println!("    found     {:?}", dependency);
        }
    }

    if !missing_dependencies.is_empty() {
        panic!("Missing dependencies: {:?}", missing_dependencies);
    }

    println!("    all dependencies found\n");
}

fn set_wallpaper(image: PathBuf) {
    let image = image.to_str().unwrap();

    Command::new("swww")
        .arg("img")
        .arg(image)
        .output()
        .expect("failed to execute process");

    Command::new("wal")
        .arg("-i")
        .arg(image)
        .output()
        .expect("failed to execute process");

    Command::new("pywalfox")
        .arg("update")
        .output()
        .expect("failed to execute process");
}

enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
}

#[derive(Debug)]
enum ParseIntervalError {
    ParseError(std::num::ParseIntError),
    InvalidUnit,
}

fn parse_interval(interval: String) -> Result<u64, ParseIntervalError> {
    let (value_str, unit_str) = interval.split_at(interval.len() - 1);

    let unit = match unit_str {
        "s" => TimeUnit::Seconds,
        "m" => TimeUnit::Minutes,
        "h" => TimeUnit::Hours,
        "d" => TimeUnit::Days,
        "w" => TimeUnit::Weeks,
        _ => return Err(ParseIntervalError::InvalidUnit),
    };

    let value = value_str.parse::<u64>().map_err(ParseIntervalError::ParseError)?;

    match unit {
        TimeUnit::Seconds => Ok(value),
        TimeUnit::Minutes => Ok(value * 60),
        TimeUnit::Hours => Ok(value * 60 * 60),
        TimeUnit::Days => Ok(value * 60 * 60 * 24),
        TimeUnit::Weeks => Ok(value * 60 * 60 * 24 * 7),
    }
}

fn main() {
    let opt = Opt::from_args();
    let interval = parse_interval(opt.interval).unwrap();
    let paths = opt.path;
    verify_paths(paths.clone());

    println!();
    println!("starting with configs:");
    println!("    interval  {}", interval);
    println!("    paths     {:?}", paths);
    println!("    config    {:?}", opt.config);
    println!();

    check_dependencies();


    // check if the first directory is not a directory
    if !paths[0].is_dir() {
        println!("setting wallpaper");
        set_wallpaper(paths[0].clone());
        return;
    } else {

        println!("setting wallpapers");

        loop {
            let time = chrono::Local::now().format("%H%M").to_string();
            let time = time.parse::<u16>().unwrap();

            let mut directory = match time {
                600..=1199 => 0,
                1200..=1799 => 1,
                _ => 2,
            };

            if paths.len() == 1 {
                directory = 0;
            }

            if directory >= paths.len() {
                directory = paths.len() - 1;
            }

            let image = pick_image(&paths[directory]);
            println!("    picked    {:?} from dir {:?}", image, directory);
            set_wallpaper(image.clone());
            sleep(std::time::Duration::from_secs(interval));
        }
    }
}

