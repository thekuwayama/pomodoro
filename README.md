# pomodoro

[![CI](https://github.com/thekuwayama/pomodoro/actions/workflows/ci.yml/badge.svg)](https://github.com/thekuwayama/pomodoro/actions/workflows/ci.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-brightgreen.svg)](https://raw.githubusercontent.com/thekuwayama/pomodoro/main/LICENSE.txt)
[![dependency status](https://deps.rs/repo/github/thekuwayama/pomodoro/status.svg)](https://deps.rs/repo/github/thekuwayama/pomodoro)

`pomodoro` is Pomodoro Timer CLI.


## Install

You can install `pomodoro` with the following:

```sh-session
$ cargo install --git https://github.com/thekuwayama/pomodoro.git --branch main
```


## Usage

```sh-session
$ pomodoro --help
pomodoro 0.1.0
command-line pomodoro timer

USAGE:
    pomodoro [ARGS]

ARGS:
    <WORKING_TIME>    working time (minutes) [default: 25]
    <BREAK_TIME>      break time (minutes) [default: 5]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```


## License

The CLI is available as open source under the terms of the [MIT License](http://opensource.org/licenses/MIT).
