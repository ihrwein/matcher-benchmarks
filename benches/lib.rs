#![feature(test)]
extern crate test;
extern crate matcher_benchmarks;
extern crate actiondb;

use actiondb::matcher::GenericFactory;
use actiondb::matcher::MatcherFactory;
use actiondb::matcher::MatcherSuite;
use actiondb::parsers::ParserFactory;

pub use actiondb::matcher::Matcher;
pub use test::Bencher;
pub use actiondb::matcher::trie::TrieMatcherSuite;
pub use matcher_benchmarks::suites::RegexMatcherSuite;
pub use matcher_benchmarks::suites::SuffixArrayMatcherSuite;
pub use matcher_benchmarks::create_matcher;
pub use matcher_benchmarks::TrieMatcherSuiteWithRegexParsers;

macro_rules! bench {
    ($test_name:ident, $test_message:expr, $suite:ty, $path:expr) => {
        #[allow(non_snake_case)]
        #[bench]
        fn $test_name(b: &mut Bencher) {
            let matcher = create_matcher::<$suite>($path);
            b.iter(|| {
                matcher.parse($test_message);
            });
        }
    }
}


mod test_500_patterns {
    use super::*;
    use matcher_benchmarks::test_datas::pattern_500::*;

    bench!(bench_TrieMatcherSuite_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, TrieMatcherSuite, PATTERN_FILE);
    bench!(bench_TrieMatcherSuite_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, TrieMatcherSuite, PATTERN_FILE);

    bench!(bench_TrieMatcherSuiteWithRegexParsers_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, TrieMatcherSuiteWithRegexParsers, PATTERN_FILE);
    bench!(bench_TrieMatcherSuiteWithRegexParsers_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, TrieMatcherSuiteWithRegexParsers,PATTERN_FILE);

    bench!(bench_RegexMatcherSuite_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, RegexMatcherSuite, PATTERN_FILE);
    bench!(bench_RegexMatcherSuite_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, RegexMatcherSuite, PATTERN_FILE);

    bench!(bench_SuffixArrayMatcherSuite_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, SuffixArrayMatcherSuite, PATTERN_FILE);
    bench!(bench_SuffixArrayMatcherSuite_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, SuffixArrayMatcherSuite, PATTERN_FILE);
}

mod test_400_patterns {
    use super::*;
    use matcher_benchmarks::test_datas::pattern_400::*;

    bench!(bench_TrieMatcherSuite_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, TrieMatcherSuite, PATTERN_FILE);
    bench!(bench_TrieMatcherSuite_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, TrieMatcherSuite, PATTERN_FILE);

    bench!(bench_TrieMatcherSuiteWithRegexParsers_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, TrieMatcherSuiteWithRegexParsers, PATTERN_FILE);
    bench!(bench_TrieMatcherSuiteWithRegexParsers_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, TrieMatcherSuiteWithRegexParsers,PATTERN_FILE);

    bench!(bench_RegexMatcherSuite_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, RegexMatcherSuite, PATTERN_FILE);
    bench!(bench_RegexMatcherSuite_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, RegexMatcherSuite, PATTERN_FILE);

    bench!(bench_SuffixArrayMatcherSuite_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, SuffixArrayMatcherSuite, PATTERN_FILE);
    bench!(bench_SuffixArrayMatcherSuite_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, SuffixArrayMatcherSuite, PATTERN_FILE);
}

mod test_300_patterns {
    use super::*;
    use matcher_benchmarks::test_datas::pattern_300::*;

    bench!(bench_TrieMatcherSuite_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, TrieMatcherSuite, PATTERN_FILE);
    bench!(bench_TrieMatcherSuite_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, TrieMatcherSuite, PATTERN_FILE);

    bench!(bench_TrieMatcherSuiteWithRegexParsers_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, TrieMatcherSuiteWithRegexParsers, PATTERN_FILE);
    bench!(bench_TrieMatcherSuiteWithRegexParsers_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, TrieMatcherSuiteWithRegexParsers,PATTERN_FILE);

    bench!(bench_RegexMatcherSuite_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, RegexMatcherSuite, PATTERN_FILE);
    bench!(bench_RegexMatcherSuite_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, RegexMatcherSuite, PATTERN_FILE);

    bench!(bench_SuffixArrayMatcherSuite_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, SuffixArrayMatcherSuite, PATTERN_FILE);
    bench!(bench_SuffixArrayMatcherSuite_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, SuffixArrayMatcherSuite, PATTERN_FILE);
}

mod test_200_patterns {
    use super::*;
    use matcher_benchmarks::test_datas::pattern_200::*;

    bench!(bench_TrieMatcherSuite_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, TrieMatcherSuite, PATTERN_FILE);
    bench!(bench_TrieMatcherSuite_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, TrieMatcherSuite, PATTERN_FILE);

    bench!(bench_TrieMatcherSuiteWithRegexParsers_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, TrieMatcherSuiteWithRegexParsers, PATTERN_FILE);
    bench!(bench_TrieMatcherSuiteWithRegexParsers_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, TrieMatcherSuiteWithRegexParsers,PATTERN_FILE);

    bench!(bench_RegexMatcherSuite_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, RegexMatcherSuite, PATTERN_FILE);
    bench!(bench_RegexMatcherSuite_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, RegexMatcherSuite, PATTERN_FILE);

    bench!(bench_SuffixArrayMatcherSuite_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, SuffixArrayMatcherSuite, PATTERN_FILE);
    bench!(bench_SuffixArrayMatcherSuite_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, SuffixArrayMatcherSuite, PATTERN_FILE);
}

mod test_100_patterns {
    use super::*;
    use matcher_benchmarks::test_datas::pattern_100::*;

    bench!(bench_TrieMatcherSuite_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, TrieMatcherSuite, PATTERN_FILE);
    bench!(bench_TrieMatcherSuite_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, TrieMatcherSuite, PATTERN_FILE);

    bench!(bench_TrieMatcherSuiteWithRegexParsers_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, TrieMatcherSuiteWithRegexParsers, PATTERN_FILE);
    bench!(bench_TrieMatcherSuiteWithRegexParsers_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, TrieMatcherSuiteWithRegexParsers,PATTERN_FILE);

    bench!(bench_RegexMatcherSuite_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, RegexMatcherSuite, PATTERN_FILE);
    bench!(bench_RegexMatcherSuite_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, RegexMatcherSuite, PATTERN_FILE);

    bench!(bench_SuffixArrayMatcherSuite_when_message_matches, TEST_MESSAGE_WHICH_MATCHES, SuffixArrayMatcherSuite, PATTERN_FILE);
    bench!(bench_SuffixArrayMatcherSuite_when_message_does_not_match, TEST_MESSAGE_WHICH_DOES_NOT_MATCH, SuffixArrayMatcherSuite, PATTERN_FILE);
}
