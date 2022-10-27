A simple and tiny CLI timer written in rust. You provide a duration in a humanized format (such as `1h35m`, `25m` or `10s` etc.) and (optionally) a command that you'd like to be executed after the duration is over.

**Usage:**

```bash
$ timer-blah [duration] "[command to execute when done]"
```

Examples:

```bash
$ timer-blah 47m11s
$ timer-blah 10s
$ timer-blah 1d
$ timer-blah 1y
$ timer-blah 1h43m9s "notify-send hello"
$ timer-blah 1h43m9s "notify-send 'hello world'"
```

Nothing much to it. Install it through cargo running: 

```bash
cargo install timer-blah
```

