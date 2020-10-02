const TERM_WIDTH: usize = 140;

fn main() {
    let (from, to) = parse_args();

    let mut s = String::with_capacity(TERM_WIDTH);
    s.push('[');
    {
        let width = TERM_WIDTH - 2;
        let fac = from as f64 / to as f64;
        let progress = (width as f64 * fac) as usize;
        let left = width - progress;

        for _ in 0..progress {
            s.push('#');
        }

        for _ in 0..left {
            s.push(':');
        }
    }
    s.push(']');

    println!("{}", s);
}

pub fn parse_args() -> (usize, usize) {
    let mut args = std::env::args().skip(1);
    if let Some(h) = args.next() {
        if h == "-h" || h == "--help" {
            print_help(0);
        }

        if let Some(w) = args.next() {
            let a = h.parse().unwrap_or_else(|_| print_help(1));
            let b = w.parse().unwrap_or_else(|_| print_help(1));
            return (a, b);
        }
    }

    print_help(1)
}

fn print_help(code: i32) -> ! {
    let helptext = "
progress
author: Nils Martel

usage: progress <from> <to>
";

    println!("{}", helptext);
    std::process::exit(code);
}
