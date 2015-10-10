#![feature(plugin,const_fn)]
#![plugin(stainless)]
extern crate actiondb;
extern crate regex;
#[macro_use]
extern crate maplit;

pub mod suites;

use actiondb::Matcher;
use actiondb::matcher::MatcherFactory;
use actiondb::parsers::{
    ParserFactory
};

pub trait MatcherSuite {
    type Matcher: Matcher;
    type ParserFactory: ParserFactory;
    type MatcherFactory: MatcherFactory<Matcher=Self::Matcher>;
}
