extern crate matcher_benchmarks;
extern crate actiondb;

use actiondb::matcher::Matcher;
use matcher_benchmarks::create_matcher;
use actiondb::matcher::trie::TrieMatcherSuite;

use matcher_benchmarks::test_datas::pattern_500;

fn main() {
    let matcher = create_matcher::<TrieMatcherSuite>(pattern_500::PATTERN_FILE);
    matcher.parse(pattern_500::TEST_MESSAGE_WHICH_MATCHES);
}
