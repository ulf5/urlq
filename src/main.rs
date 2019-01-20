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

    /// Decode + to space or encode space to +
    #[structopt(short = "p", long = "plus")]
    plus: bool,

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

fn all_decoder(string: &str) -> String {
    String::new()
}

fn decoder(string: &str) -> String {
    String::new()
}

fn all_encoder(string: &str) -> String {
    String::new()
}

fn encoder(string: &str) -> String {
    String::from("Hej")
}

fn get_handler(decode: bool, all: bool) -> fn(&str) -> String {
    if decode && all {
        all_decoder
    } else if decode {
        decoder
    } else if all {
        all_encoder
    } else {
        encoder
    }
}

fn main() {
    let opt = Opt::from_args();
    let input = Input::from(opt.strings);
    let handler = get_handler(opt.decode, opt.all);
    input.iterator().for_each(|a| println!("{}", handler(a.as_str())));
}
