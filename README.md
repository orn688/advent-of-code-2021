# Advent of Code 2021 Solutions

<https://adventofcode.com/2021>

This is my first significant foray into Rust, do not expect my code to be
pretty :)

## Setup

Requires [Rust and Cargo](https://www.rust-lang.org/tools/install) to be
installed locally.

The code assumes the `AOC_SESSION_ID` environment variable is set to the
value of your AoC "session" cookie (you can retrieve this value by logging
into <adventofcode.com> and looking in the cookies section of your browser's
dev tools). I use [direnv](https://direnv.net) to set `$AOC_SESSION_ID` only
within the directory containing this project.

## Usage

```sh
cargo run $day $part
```

This will automatically download the input for the given day and cache it in
a file in the `.cache` directory so it doesn't need to be re-downloaded every
time.

E.g. to get the solution for day 3, part 2:

```sh
cargo run 3 2
```
