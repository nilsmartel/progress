#![feature(exclusive_range_pattern)]
#![feature(half_open_range_patterns)]

const TERM_WIDTH: usize = 140;

fn main() {
    let (from, to) = parse_args();
}

pub fn parse_args() -> (usize, usize) {
    let args = std::env::args();
    let stargs = args.map(|n| n.as_str()).collect::<Vec<&str>>();

    match stargs[..] {
        ["-h", .._rest] | ["--help", .._rest] => print_help(0),
        [from, to] => {
            let a = from.parse().unwrap_or_else(|_| print_help(1));
            let b = to.parse().unwrap_or_else(|_| print_help(1));
            (a, b)
        }
        _ => print_help(1),
    }
}

fn print_help(code: i32) -> ! {
    let helptext = "
progress
author: Nils Martel

usage: progress <from> <to>
";
    std::process::exit(code);
}
