extern crate either;
extern crate structopt;

use std::io::BufRead;
use std::io::stdin;
use std::io::Stdin;
use std::io::StdinLock;

use either::Either;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "url")]
struct Opt {
    /// Url decode instead of encode
    #[structopt(short = "d", long = "decode")]
    decode: bool,

    /// Query encode/decode the entire string
    #[structopt(short = "q", long = "query")]
    all: bool,

    /// Decode + to space or encode space to +
    #[structopt(short = "p", long = "plus")]
    plus: bool,

    /// Strings to url encode/decode
    #[structopt(raw(multiple = "true"))]
    strings: Vec<String>,
}

#[derive(Debug)]
struct Input<'a> {
    input_args: Vec<String>,
    stdin: &'a Stdin,
}

impl<'a> Input<'a> {
    fn from(input_args: Vec<String>, stdin: &Stdin) -> Input {
        Input {
            input_args,
            stdin,
        }
    }

    fn iterator(self) -> impl Iterator<Item=String> + 'a {
        if self.input_args.len() != 0 {
            return Either::Left(self.input_args.into_iter());
        }
        Either::Right(
            self.stdin.lock().lines()
                .map(|line| line.expect("IO error"))
                .into_iter()
        )
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
    let i = stdin();
    let input = Input::from(opt.strings, &i);
    let handler = get_handler(opt.decode, opt.all);
    input.iterator().for_each(|a| println!("{}", handler(a.as_str())));
}
