// Command line tool to take in a time in seconds, minutes, hours, days (or all combined) and
// return a status bar with time remaining, once it's over you should be able to pipe commands after it's done
// ======================================================
// test-blah 35s
// test-blah 1h
// test-blah 15s
// test-blah 123m
// test-blah 1h35m15s
// test-blah 1h35m15s "hello, world" | notify-send
// ------------------
// Remaining time: 15 minutes and 35 seconds
// 0% [########-------------------------------] 100%

// ------------------
// Help notices:
// test-blah --help
// test-blah
// =========================================
// You should provide an argument in the following format ...
// ---------------------------
use humantime;
use std::{
    env,
    io::{self, Write},
    process::Command,
    thread,
    time::Duration,
};
// use std::time::{Duration, SystemTime};

const TICK_DURATION: Duration = Duration::from_secs(1);

// TODO: Questions (stuff to learn?):
// - Difference between &str and String????
// - Why do I need as_str for the argument
// - Traits?
// BUG: If we run it with 1s as an argument it panics
fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    if args.len() == 1 || &args[1] == "--help" {
        println!("Oh noes");
        return;
    }

    let input_time = &args[1];

    let input_duration =
        humantime::parse_duration(input_time).expect("Should parse user input to duration");

    let mut result = input_duration
        .checked_sub(TICK_DURATION)
        .expect("Should subtract a second from duration");
    let mut seconds = input_duration.as_secs();

    dbg!(seconds);

    while seconds > 0 {
        let label = humantime::format_duration(result).to_string();

        let status = format!("\rTime remaining {label}\t");
        // Is this the most idiomatic way to print characters on the same line? I DUNNO!
        print!("{status}");
        let _ = io::stdout().flush();

        if seconds == 1 {
            seconds = 0;
        } else {
            result = result.checked_sub(TICK_DURATION).expect("hello 2");
            seconds = result.as_secs();
        }

        // repeat every second
        thread::sleep(TICK_DURATION);
    }

    if args.len() == 2 {
        return;
    }

    let input_string = &args[2];
    let input_string_fragments: Vec<&str> = input_string.split(" ").collect();
    let user_command = input_string_fragments[0];

    dbg!(&input_string_fragments);

    let mut command = Command::new(user_command);

    for fragment in &input_string_fragments[1..] {
        command.arg(fragment);
    }
    // let arguments = input_string_fragments.into_iter().map(|x| command.arg(x));

    command.status().expect("Error ls cmd");

    // TODO: Write this back to the terminal so that it can be piped to another tool
    // io::stdout().write()
}
