#![feature(test)]
extern crate test;
extern crate matcher_benchmarks;
extern crate actiondb;

use actiondb::matcher::GenericFactory;
use actiondb::matcher::Matcher;
use actiondb::matcher::MatcherFactory;
use matcher_benchmarks::*;
use test::Bencher;

fn create_matcher<T>() -> T::Matcher
    where T: MatcherSuite {
    let path = "benches/ssh_ok.json";
    let result = GenericFactory::from_json_file::<T::MatcherFactory>(path);
    if result.is_err() {
        println!("{:?}", &result);
    }
    result.ok().expect("Failed to create a Matcher object")
}

use matcher_benchmarks::suites::TrieMatcherSuite;

#[bench]
fn bench_trie_matcher_with_ssh_logs(b: &mut Bencher) {
    let matcher = create_matcher::<TrieMatcherSuite>();
    b.iter(|| {
        matcher.parse("Jun 25 14:09:41 lobotomy sshd[26478]: pam_unix(sshd:session): session closed for user zts");
    });
}
