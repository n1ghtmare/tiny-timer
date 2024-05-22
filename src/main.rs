use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::{env, process::Command, thread, time::Duration};

const TICK_DURATION: Duration = Duration::from_secs(1);

const HELP_MESSAGE: &str = "Usage:
    tiny-timer [duration] \"[command to execute when done]\"

EXAMPLES:
    tiny-timer 47m11s
    tiny-timer 10s
    tiny-timer 1d
    tiny-timer 1y
    tiny-timer 1h43m9s \"notify-send hello\"
    tiny-timer 1h43m9s \"notify-send 'hello world'\"

META OPTIONS:
    -?, --help    print this help message
";

fn generate_progress_message(duration: Duration, command: &str) -> String {
    let humanized_duration = humantime::format_duration(duration).to_string();
    if command.is_empty() {
        format!("Time remaining {humanized_duration}")
    } else {
        format!("Time remaining {humanized_duration} | Payload: {command}")
    }
}

fn create_progress_bar() -> ProgressBar {
    let progress_bar = ProgressBar::new(100);
    progress_bar
        .set_style(ProgressStyle::with_template("\n{wide_bar} {percent}%\n-\n{msg}").unwrap());
    progress_bar
}

fn get_percentage_completed(total_duration: Duration, current_duration: Duration) -> u64 {
    let total_seconds: f32 = total_duration.as_secs_f32();
    let current_seconds: f32 = current_duration.as_secs_f32();
    let completion = 100.0 - ((current_seconds / total_seconds) * 100.0);
    completion as u64
}

// This should take care of cases where the command contains multiple words with single quotes, which we should threat as a single argument
// Example: The command "notify-send --some-argument --another-one 'hello twitch chat'", should produce:
// [notify-send, --some-argument, --another-one, 'hello twitch chat']
fn parse_command_fragments(command: &str) -> Vec<&str> {
    // should match anything within '' (including the quotes we're getting rid of them later)
    let quotes_regex = Regex::new(r"'([^']*?)'").unwrap();

    let arguments: Vec<&str> = quotes_regex
        .split(command)
        .flat_map(|x| x.split(' '))
        .filter(|x| !x.is_empty())
        .collect();

    let arguments_quoted: Vec<&str> = quotes_regex
        .captures_iter(command)
        .map(|x| x.get(0).map_or("", |m| m.as_str()).trim_matches('\''))
        .collect();

    [arguments, arguments_quoted].concat()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 || &args[1] == "--help" || &args[1] == "-?" {
        println!("{HELP_MESSAGE}");
        return;
    }

    let input_command = &args[1];
    let input_string = if args.len() > 2 { &args[2] } else { "" };

    // Parse the duration from passed by the user
    let duration: Duration =
        humantime::parse_duration(input_command).expect("Can't parse user input");

    // Add a second to the duration to make sure we don't miss the last second
    let duration = duration + Duration::from_secs(1);

    let mut subtracted_duration = duration
        .checked_sub(TICK_DURATION)
        .expect("Can't subtract a second from duration");
    let mut seconds = duration.as_secs();

    let progress_bar = create_progress_bar();

    while seconds > 0 {
        let percentage_completion = get_percentage_completed(duration, subtracted_duration);
        progress_bar.set_position(percentage_completion);

        let status = generate_progress_message(subtracted_duration, input_string);
        progress_bar.set_message(status.to_string());

        if seconds == 1 {
            seconds = 0;
        } else {
            subtracted_duration = subtracted_duration
                .checked_sub(TICK_DURATION)
                .expect("Can't subtract a second from duration");

            seconds = subtracted_duration.as_secs();
        }

        thread::sleep(TICK_DURATION);
    }

    if input_string.is_empty() {
        progress_bar.finish_with_message("Done!");
        return;
    }

    let command_fragments: Vec<&str> = parse_command_fragments(input_string);

    Command::new(command_fragments[0])
        .args(&command_fragments[1..])
        .status()
        .expect("Can't process command");

    progress_bar.finish_with_message("Done!");
}
