extern crate either;
extern crate structopt;
extern crate urlq;

use std::io::BufRead;
use std::io::stdin;
use std::io::Stdin;

use atty::isnt;
use either::Either::{Left, Right};
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name = "urlq")]
struct Opt {
    /// Percent decode instead of encode
    #[structopt(
    short = "d",
    long = "decode",
    conflicts_with = "url",
    conflicts_with = "path",
    conflicts_with = "path-segment",
    conflicts_with = "query",
    conflicts_with = "userinfo",
    conflicts_with = "fragment"
    )]
    decode: bool,

    /// Try to parse the string as an url and encode each part appropriately
    #[structopt(
    short = "u",
    long = "url",
    conflicts_with = "decode",
    conflicts_with = "path",
    conflicts_with = "path-segment",
    conflicts_with = "query",
    conflicts_with = "userinfo",
    conflicts_with = "fragment"
    )]
    url: bool,

    /// Encode the input as the path part of a URI
    #[structopt(
    short = "p",
    long = "path",
    conflicts_with = "decode",
    conflicts_with = "url",
    conflicts_with = "path-segment",
    conflicts_with = "query",
    conflicts_with = "userinfo",
    conflicts_with = "fragment",
    conflicts_with = "plus"
    )]
    path: bool,

    /// Encode the input as a path segment part of a URI
    #[structopt(
    short = "s",
    long = "path-segment",
    conflicts_with = "decode",
    conflicts_with = "url",
    conflicts_with = "path",
    conflicts_with = "query",
    conflicts_with = "userinfo",
    conflicts_with = "fragment",
    conflicts_with = "plus"
    )]
    path_segment: bool,

    /// Encode the input as the query part of a URI
    #[structopt(
    short = "q",
    long = "query",
    conflicts_with = "decode",
    conflicts_with = "url",
    conflicts_with = "path",
    conflicts_with = "path-segment",
    conflicts_with = "userinfo",
    conflicts_with = "fragment"
    )]
    query: bool,

    /// Encode the input as the userinfo part of a URI
    #[structopt(
    short = "i",
    long = "userinfo",
    conflicts_with = "decode",
    conflicts_with = "url",
    conflicts_with = "path",
    conflicts_with = "path-segment",
    conflicts_with = "query",
    conflicts_with = "fragment",
    conflicts_with = "plus"
    )]
    userinfo: bool,

    /// Encode the input as the fragment part of a URI
    #[structopt(
    short = "f",
    long = "fragment",
    conflicts_with = "decode",
    conflicts_with = "url",
    conflicts_with = "path",
    conflicts_with = "path-segment",
    conflicts_with = "query",
    conflicts_with = "userinfo",
    conflicts_with = "plus"
    )]
    fragment: bool,

    /// Decode + to space or encode space to +
    #[structopt(short = "+", long = "plus")]
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
        if !self.input_args.is_empty() {
            return Some(Left(self.input_args.into_iter()));
        }
        if isnt(atty::Stream::Stdin) {
            return Some(
                Right(
                    self.stdin.lock().lines()
                        .map(|line| line.expect("IO error"))
                ));
        }
        None
    }
}

fn encode_url_plus(url: &str) -> String {
    urlq::encode_url_plus(url).unwrap_or(format!("Failed to parse \"{}\" as url", url))
}

fn encode_url(url: &str) -> String {
    urlq::encode_url(url).unwrap_or(format!("Failed to parse \"{}\" as url", url))
}

fn get_handler(opt: &Opt) -> fn(&str) -> String {
    if opt.decode {
        if opt.plus {
            return urlq::decode_plus;
        } else {
            return urlq::decode;
        }
    }
    if opt.url {
        if opt.plus {
            return encode_url_plus;
        } else {
            return encode_url;
        }
    }
    if opt.query {
        if opt.plus {
            return urlq::encode_query_plus;
        } else {
            return urlq::encode_query;
        }
    }
    if opt.path {
        return urlq::encode_path;
    }
    if opt.path_segment {
        return urlq::encode_path_segment;
    }
    if opt.userinfo {
        return urlq::encode_userinfo;
    }
    if opt.fragment {
        return urlq::encode_fragment;
    }
    urlq::encode_all_reserved
}

fn main() {
    let opt = Opt::from_args();
    let handler = get_handler(&opt);
    let i = stdin();
    let input = Input::from(opt.strings, &i);

    // Yuck
    input.iterator()
        .map_or_else(|| println!("Missing input (\"urlq --help\" for help)"),
                     |a| a.for_each(|b| println!("{}", handler(b.as_str()))));
}
