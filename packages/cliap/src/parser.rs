use combine::{
    eof, optional,
    parser::repeat::sep_by1,
    parser::{
        char::{alpha_num, char, digit, lower, string},
        range::take_while1,
    },
    Parser,
};

use crate::builder::LabelKind;

fn dash<'a>() -> impl Parser<&'a str> {
    char('-')
}

fn dashes<'a>() -> impl Parser<&'a str> {
    string("--")
}

fn short_identifier<'a>() -> impl Parser<&'a str, Output = char> {
    alpha_num()
}

fn long_identifier<'a>() -> impl Parser<&'a str, Output = String> {
    let separator = dash();
    let parser = lower().or(digit());
    let both = sep_by1(parser, separator).map(|chars: Vec<char>| chars.into_iter().collect());
    both
}

fn rest1<'a>() -> impl Parser<&'a str, Output = &'a str> {
    take_while1(|_| true)
}

//todo - quotes: ='argument' ' ""
fn argument<'a>() -> impl Parser<&'a str, Output = String> {
    char('=').with(rest1()).map(|str| str.to_string())
}

pub fn flag<'a>() -> impl Parser<&'a str, Output = ((String, LabelKind), Option<String>)> {
    let short_flag = dash()
        .with(short_identifier())
        .map(|char| char.to_string())
        .map(|identifier| (identifier, LabelKind::Short));

    let long_flag = dashes()
        .with(long_identifier())
        .map(|identifier| (identifier, LabelKind::Long));

    let flag = short_flag
        .or(long_flag)
        .and(optional(argument()))
        .skip(eof());

    flag
}
