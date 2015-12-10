extern crate matcher_benchmarks;
extern crate actiondb;

use actiondb::matcher::Matcher;
use matcher_benchmarks::create_matcher;
use matcher_benchmarks::suites::SuffixArrayMatcherSuite;

use matcher_benchmarks::test_datas::pattern_500;

fn main() {
    let matcher = create_matcher::<SuffixArrayMatcherSuite>(pattern_500::PATTERN_FILE);
    matcher.parse(pattern_500::TEST_MESSAGE_WHICH_MATCHES);
}
