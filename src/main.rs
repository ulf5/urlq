extern crate either;
extern crate structopt;
extern crate urlq;

use std::io::BufRead;
use std::io::stdin;
use std::io::Stdin;

use either::Either;
use structopt::StructOpt;
use atty::isnt;

#[derive(StructOpt, Debug)]
#[structopt(name = "urlq")]
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

    fn iterator(self) -> Option<impl Iterator<Item=String> + 'a> {
        if self.input_args.len() != 0 {
            return Some(Either::Left(self.input_args.into_iter()));
        }
        if isnt(atty::Stream::Stdin) {
            return Some(Either::Right(
                self.stdin.lock().lines()
                    .map(|line| line.expect("IO error"))
                    .into_iter()
            ));
        }
        None
    }
}


fn all_encoder(string: &str) -> String {
    String::new()
}

fn encoder(string: &str) -> String {
    String::from("Hej")
}

fn get_handler(decode: bool, all: bool, plus: bool) -> fn(&str) -> String {
    if decode && plus {
        urlq::decode_plus
    } else if decode {
        urlq::decode
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
    let handler = get_handler(opt.decode, opt.all, opt.plus);
    input.iterator()
        .map_or_else(|| println!("Missing input (\"urlq --help\" for help)"),
                     |a|
                         a.for_each(|b| println!("{}", handler(b.as_str()))));
}
