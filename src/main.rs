// Command line tool to take in a time in seconds, minutes, hours, days (or all combined) and
// return a status bar with time remaining, once it's over you should be able to pipe commands after it's done
// ======================================================
// test-blah 35m
// test-blah 1h
// test-blah 15s
// test-blah 123m
// test-blah 1h35m15s
// test-blah 1h35m15s "hello, world" | notify-send
// ------------------
// Remaining time: 15 minutes and 35 seconds
// 0% [########-------------------------------] 100%

// ------------------
use humantime;
use std::{
    env,
    io::{self, Write},
    thread,
    time::Duration,
};
// use std::time::{Duration, SystemTime};

const TICK_DURATION: Duration = Duration::from_secs(1);

// TODO: Questions (stuff to learn?):
// - Difference between &str and String????
// - Traits?
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_time = &args[1];

    let input_duration =
        humantime::parse_duration(input_time).expect("Should parse user input to duration");

    let mut result = input_duration.checked_sub(TICK_DURATION).expect("hello");
    let mut seconds = input_duration.as_secs();

    while seconds > 0 {
        let label = humantime::format_duration(result).to_string();

        // Is this the most idiomatic way to print characters on the same line? I DUNNO!
        print!("\rTime remaining {}\t", label);
        let _ = io::stdout().flush();

        result = result.checked_sub(TICK_DURATION).expect("hello");
        seconds = result.as_secs();

        // repeat every second
        thread::sleep(TICK_DURATION);
    }
}
