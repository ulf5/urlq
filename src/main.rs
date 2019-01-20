extern crate structopt;
extern crate either;

use std::io::BufRead;
use std::io::stdin;

use either::Either;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "url")]
struct Opt {
    /// Url decode instead of encode
    #[structopt(short = "d", long = "decode")]
    decode: bool,

    /// Url encode/decode the entire string instead of just the query string
    #[structopt(short = "a", long = "all")]
    all: bool,

    /// Strings to url encode/decode
    #[structopt(raw(multiple = "true"))]
    strings: Vec<String>,
}

#[derive(Debug)]
struct Input {
    input_args: Vec<String>
}

impl Input {
    fn from(input_arg: Vec<String>) -> Input {
        Input { input_args: input_arg }
    }

    fn iterator(self) -> impl Iterator<Item=String> {
        if self.input_args.len() != 0 {
            return Either::Left(self.input_args.into_iter());
        }
        // I don't like this.
        let i: Vec<String> = stdin().lock().lines()
            .map(|line| line.expect("IO error")).collect();
        Either::Right(i.into_iter())
    }
}

fn main() {
    let opt = Opt::from_args();
    let input = Input::from(opt.strings);
    input.iterator().for_each(|a| println!("{}", a));
}
