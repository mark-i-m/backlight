use std::fs::{read_to_string, write};

use clap::clap_app;

const FILE: &str = "/sys/class/backlight/intel_backlight/brightness";

fn main() {
    let matches = clap_app! { backlight =>
        (about: "Update the brightness of the screen. Needs to run as root.")
        (@group dir =>
            (@attributes +required)
            (@arg INC: --inc +takes_value {is_usize}
             "Increase by INC points")
            (@arg DEC: --dec +takes_value {is_usize}
             "Increase by DEC points")
        )
    }
    .get_matches();

    let change = matches
        .value_of("INC")
        .map(|v| v.parse::<usize>().unwrap() as isize)
        .or(matches
            .value_of("DEC")
            .map(|v| -(v.parse::<usize>().unwrap() as isize)))
        .unwrap();

    let current = read_to_string(FILE)
        .expect("Unable to read current brightness")
        .trim()
        .parse::<isize>()
        .expect("Unable to parse current brightness");

    let new = current + change;

    let new_str = format!("{}", new);

    write(FILE, new_str).expect("Unable to write new brightness");
}

fn is_usize(val: String) -> Result<(), String> {
    val.parse::<usize>()
        .map(|_| ())
        .map_err(|e| format!("{}", e))
}
