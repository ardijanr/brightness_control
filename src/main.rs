use std::env;
use std::process::{exit, Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    let amount = args[1]
        .parse::<f32>()
        .expect("Error, need a positive or negative floating point number as argument");

    let mut brightness = get_brightness() + amount;

    if brightness < 0.1 {
        brightness = 0.1
    } else if brightness > 1. {
        brightness = 1.
    }

    for monitor in get_monitors() {
        println!("{:?}", monitor);
        let _ = Command::new("xrandr")
            .arg("--output")
            .arg(monitor)
            .arg("--brightness")
            .arg(brightness.to_string())
            .output();
    }
}

fn get_brightness() -> f32 {
    let mut cmd = Command::new("xrandr");
    cmd.arg("--verbose");

    let stdout = String::from_utf8(cmd.output().expect("Error, missing xrandr").stdout)
        .expect("Could not get verbose output from xrandr");

    let brightness_lines = stdout
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| l.starts_with("Brightness"))
        .collect::<Vec<&str>>();

        print!("{:?}", brightness_lines);

    if brightness_lines[0].len() < 1 {
        println!("Error, could not parse xrandr output");
        exit(1);
    }

    brightness_lines[0].split(" ").collect::<Vec<&str>>()[1]
        .parse::<f32>()
        .expect("Error, could not parse brightness number")
}

fn get_monitors() -> Vec<String> {
    let output = String::from_utf8(
        Command::new("xrandr")
            .output()
            .unwrap() // missing xrandr is checked earlier
            .stdout,
    )
    .expect("Could not capture stdout");

    output
        .split("\n")
        .map(|l| l.trim().split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
        .iter()
        .filter(|x| {
            if x.len() < 2 {
                return false;
            } else {
                x[1] == "connected"
            }
        })
        .map(|x| x[0].to_string())
        .collect::<Vec<String>>()
}
