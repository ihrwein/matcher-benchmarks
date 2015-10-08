#![feature(test)]
extern crate test;
extern crate matcher_benchmarks;
extern crate actiondb;

use actiondb::matcher::GenericFactory;
use actiondb::matcher::Matcher;
use actiondb::matcher::MatcherFactory;
use matcher_benchmarks::*;
use test::Bencher;
use actiondb::parsers::ParserFactory;

fn create_matcher<T>() -> T::Matcher
    where T: MatcherSuite {
    //let path = "benches/ssh_ok.json";
    let path = "/home/tibi/Documents/Diplomamunka/samples/default.json";
    let result = GenericFactory::from_json_file::<T::MatcherFactory>(path);
    if result.is_err() {
        println!("{:?}", &result);
    }
    result.ok().expect("Failed to create a Matcher object")
}

use matcher_benchmarks::suites::TrieMatcherSuite;
use actiondb::matcher::trie::factory::TrieMatcherFactory;
use matcher_benchmarks::suites::RegexMatcherSuite;
use matcher_benchmarks::suites::regex::RegexParserFactory;

macro_rules! bench {
    ($test_name:ident, $test_message:expr, $suite:ty) => {
        #[bench]
        fn $test_name(b: &mut Bencher) {
            let matcher = create_matcher::<$suite>();
            b.iter(|| {
                matcher.parse($test_message);
            });
        }
    }
}

const TEST_MESSAGE_WHICH_MATCHES: &'static str = "martian source 10.0.106.201 from 10.0.69.167, on dev eth0";
bench!(bench_trie_matcher_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, TrieMatcherSuite);
bench!(bench_trie_matcher_with_regex_parsers_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, TrieMatcherSuiteWithRegexParsers);
bench!(bench_regex_matcher_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, RegexMatcherSuite);

const TEST_MESSAGE_WHICH_DOES_NOT_MATCH: &'static str = r#"type=1400 audit(1444191210.403:63): apparmor="STATUS" operation="profile_replace" profile="unconfined" name="docker-default" pid=2782 comm="apparmor_parser"#;
bench!(bench_trie_matcher_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, TrieMatcherSuite);
bench!(bench_trie_matcher_with_regex_parsers_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, TrieMatcherSuiteWithRegexParsers);
bench!(bench_regex_matcher_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, RegexMatcherSuite);

struct TrieMatcherSuiteWithRegexParsers;

impl MatcherSuite for TrieMatcherSuiteWithRegexParsers {
    type Matcher = <TrieMatcherSuite as MatcherSuite>::Matcher;
    type ParserFactory = <RegexMatcherSuite as MatcherSuite>::ParserFactory;
    type MatcherFactory = <TrieMatcherSuite as MatcherSuite>::MatcherFactory;

    fn parser_factory() -> Self::ParserFactory {
        RegexParserFactory
    }
    fn matcher_factory() -> Self::MatcherFactory {
        TrieMatcherFactory
    }
}
