use std::collections::HashMap;
use std::io;
use std::{thread, time::Duration};
use chrono::Local;
use chrono_tz::Tz;

fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(0);
    (r, g, b)
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn get_ascii_digits() -> HashMap<char, Vec<&'static str>> {
    let mut digits = HashMap::new();
    digits.insert('0', vec!["  000  ", " 0   0 ", " 0   0 ", " 0   0 ", "  000  "]);
    digits.insert('1', vec!["   1   ", "  11   ", "   1   ", "   1   ", "  111  "]);
    digits.insert('2', vec!["  222  ", " 2   2 ", "   2   ", "  2    ", " 22222 "]);
    digits.insert('3', vec!["  333  ", " 3   3 ", "   33  ", " 3   3 ", "  333  "]);
    digits.insert('4', vec![" 4   4 ", " 4   4 ", " 44444 ", "     4 ", "     4 "]);
    digits.insert('5', vec![" 55555 ", " 5     ", " 55555 ", "     5 ", " 55555 "]);
    digits.insert('6', vec!["  666  ", " 6     ", " 66666 ", " 6   6 ", "  666  "]);
    digits.insert('7', vec![" 77777 ", "    7  ", "   7   ", "  7    ", " 7     "]);
    digits.insert('8', vec!["  888  ", " 8   8 ", "  888  ", " 8   8 ", "  888  "]);
    digits.insert('9', vec!["  999  ", " 9   9 ", "  9999 ", "     9 ", "  999  "]);
    digits.insert(':', vec!["       ", "   7   ", "       ", "   7   ", "       "]);
    digits.insert(' ', vec!["       ", "       ", "       ", "       ", "       "]);
    digits.insert('A', vec!["   A   ", "  A A  ", " AAAAA ", " A   A ", " A   A "]);
    digits.insert('M', vec![" M   M ", " MM MM ", " M M M ", " M   M ", " M   M "]);
    digits.insert('P', vec!["  PPP  ", " P   P ", " PPPP  ", " P     ", " P     "]);
    digits
}

fn print_clock(current_time: &str, rgb: (u8, u8, u8), digits: &HashMap<char, Vec<&str>>) {
    clear_screen();
    for i in 0..5 {
        for ch in current_time.chars() {
            let line = digits.get(&ch).unwrap_or(&vec!["       "; 5])[i];
            print!("\x1b[38;2;{};{};{}m{}\x1b[0m ", rgb.0, rgb.1, rgb.2, line);
        }
        println!();
    }
}

fn display_time_in_timezone(tz: Tz) -> String {
    let now = chrono::Utc::now().with_timezone(&tz);
    now.format("%I:%M %p").to_string()
}

fn start_clock(rgb: (u8, u8, u8), digits: &HashMap<char, Vec<&str>>) {
    println!("Enter a time zone (e.g., America/New_York):");
    let mut tz_input = String::new();
    io::stdin().read_line(&mut tz_input).unwrap();
    let tz_input = tz_input.trim();

    match tz_input.parse::<Tz>() {
        Ok(tz) => {
            let mut last_time = String::new();
            loop {
                let current_time = display_time_in_timezone(tz);
                if current_time != last_time {
                    print_clock(&current_time, rgb, digits);
                    last_time = current_time;
                }
                thread::sleep(Duration::from_secs(1));
            }
        }
        Err(_) => {
            println!("Invalid time zone.");
        }
    }
}

fn start_timer(rgb: (u8, u8, u8), digits: &HashMap<char, Vec<&str>>) {
    println!("Enter timer duration (in seconds):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut seconds = input.trim().parse::<u64>().unwrap_or(0);

    while seconds > 0 {
        let h = seconds / 3600;
        let m = (seconds % 3600) / 60;
        let s = seconds % 60;
        let time = format!("{:02}:{:02}:{:02}", h, m, s);
        print_clock(&time, rgb, digits);
        thread::sleep(Duration::from_secs(1));
        seconds -= 1;
    }

    println!("\x1b[38;2;{};{};{}mTimer finished!\x1b[0m", rgb.0, rgb.1, rgb.2);
}

fn start_stopwatch(rgb: (u8, u8, u8), digits: &HashMap<char, Vec<&str>>) {
    let start = Local::now();
    loop {
        let elapsed = Local::now() - start;
        let total_secs = elapsed.num_seconds();
        let h = total_secs / 3600;
        let m = (total_secs % 3600) / 60;
        let s = total_secs % 60;
        let time = format!("{:02}:{:02}:{:02}", h, m, s);
        print_clock(&time, rgb, digits);
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    println!("Enter a HEX color (e.g., #00ff00):");
    let mut hex_color = String::new();
    io::stdin().read_line(&mut hex_color).unwrap();
    let rgb = hex_to_rgb(hex_color.trim());

    let digits = get_ascii_digits();

    println!("Choose a mode:");
    println!("1. Clock");
    println!("2. Timer");
    println!("3. Stopwatch");

    let mut mode_input = String::new();
    io::stdin().read_line(&mut mode_input).unwrap();

    match mode_input.trim() {
        "1" => start_clock(rgb, &digits),
        "2" => start_timer(rgb, &digits),
        "3" => start_stopwatch(rgb, &digits),
        _ => println!("Invalid choice."),
    }
}
