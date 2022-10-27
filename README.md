tiny-timer
==


A simple and tiny CLI timer written in rust. You provide a duration in a humanized format (such as `1h35m`, `25m` or `10s` etc.) and (optionally) a command that you'd like to be executed after the duration is over.

**Usage:**

```bash
$ tiny-timer [duration] "[command to execute when done]"
```

Examples:

```bash
$ tiny-timer 47m11s
$ tiny-timer 10s
$ tiny-timer 1d
$ tiny-timer 1y
$ tiny-timer 1h43m9s "notify-send hello"
$ tiny-timer 1h43m9s "notify-send 'hello world'"
```

Nothing much to it. Install it through cargo running: 

```bash
cargo install tiny-timer
```

https://user-images.githubusercontent.com/3255810/198414917-6e894cc4-719e-485d-acae-ecaa9ab4e032.mp4

