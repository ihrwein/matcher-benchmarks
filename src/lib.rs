#![feature(plugin,const_fn)]
extern crate actiondb;
extern crate regex;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate log;

pub mod suites;

use actiondb::matcher::PatternLoader;
use actiondb::matcher::MatcherFactory;
use actiondb::matcher::MatcherSuite;
use actiondb::matcher::Matcher;
use actiondb::matcher::trie::TrieMatcherSuite;
use suites::RegexMatcherSuite;

pub fn create_matcher<T>(path: &str) -> T::Matcher
    where T: MatcherSuite {
    let result = PatternLoader::from_json_file::<T::MatcherFactory>(path);
    if result.is_err() {
        println!("{:?}", &result);
    }
    result.ok().expect("Failed to create a Matcher object")
}

pub struct TrieMatcherSuiteWithRegexParsers;

impl MatcherSuite for TrieMatcherSuiteWithRegexParsers {
    type Matcher = <TrieMatcherSuite as MatcherSuite>::Matcher;
    type ParserFactory = <RegexMatcherSuite as MatcherSuite>::ParserFactory;
    type MatcherFactory = <TrieMatcherSuite as MatcherSuite>::MatcherFactory;
}

pub mod test_datas {
    pub mod pattern_500 {
        pub const TEST_MESSAGE_WHICH_MATCHES: &'static str = "Authorized to pirar01, krb5 principal pirar01@LOCAL (krb5_kuserok)";
        pub const TEST_MESSAGE_WHICH_DOES_NOT_MATCH: &'static str = r#"[origin software="rsyslogd" swVersion="7.4.4" x-pid="665" x-info="http://www.rsyslog.com"] start"#;
        pub const PATTERN_FILE: &'static str = "/home/tibi/Documents/Diplomamunka/samples/default_500.json";
        pub const EXPECTED_UUID: &'static str = "71e3b37b-87a8-4901-824e-50bae6e789a0";
    }
    pub mod pattern_400 {
        pub const TEST_MESSAGE_WHICH_MATCHES: &'static str = "Authorized to pirar01, krb5 principal pirar01@LOCAL (krb5_kuserok)";
        pub const TEST_MESSAGE_WHICH_DOES_NOT_MATCH: &'static str = r#"[origin software="rsyslogd" swVersion="7.4.4" x-pid="665" x-info="http://www.rsyslog.com"] start"#;
        pub const PATTERN_FILE: &'static str = "/home/tibi/Documents/Diplomamunka/samples/default_400.json";
    }
    pub mod pattern_300 {
        pub const TEST_MESSAGE_WHICH_MATCHES: &'static str = "Authorized to pirar01, krb5 principal pirar01@LOCAL (krb5_kuserok)";
        pub const TEST_MESSAGE_WHICH_DOES_NOT_MATCH: &'static str = r#"[origin software="rsyslogd" swVersion="7.4.4" x-pid="665" x-info="http://www.rsyslog.com"] start"#;
        pub const PATTERN_FILE: &'static str = "/home/tibi/Documents/Diplomamunka/samples/default_300.json";
    }
    pub mod pattern_200 {
        pub const TEST_MESSAGE_WHICH_MATCHES: &'static str = "Authorized to pirar01, krb5 principal pirar01@LOCAL (krb5_kuserok)";
        pub const TEST_MESSAGE_WHICH_DOES_NOT_MATCH: &'static str = r#"[origin software="rsyslogd" swVersion="7.4.4" x-pid="665" x-info="http://www.rsyslog.com"] start"#;
        pub const PATTERN_FILE: &'static str = "/home/tibi/Documents/Diplomamunka/samples/default_200.json";
    }
    pub mod pattern_100 {
        pub const TEST_MESSAGE_WHICH_MATCHES: &'static str = "Authorized to pirar01, krb5 principal pirar01@LOCAL (krb5_kuserok)";
        pub const TEST_MESSAGE_WHICH_DOES_NOT_MATCH: &'static str = r#"[origin software="rsyslogd" swVersion="7.4.4" x-pid="665" x-info="http://www.rsyslog.com"] start"#;
        pub const PATTERN_FILE: &'static str = "/home/tibi/Documents/Diplomamunka/samples/default_100.json";
    }
}

#[cfg(test)]
mod tests {
    pub use actiondb::matcher::result::MatchResult;
    pub use actiondb::matcher::Matcher;
    pub use actiondb::matcher::trie::suite::TrieMatcherSuite;
    pub use suites::RegexMatcherSuite;
    pub use suites::SuffixArrayMatcherSuite;
    pub use ::TrieMatcherSuiteWithRegexParsers;

macro_rules! test_matching {
    ($test_name:ident, $test_message:expr, $suite:ty, $path:expr, $uuid:expr) => {
        #[allow(non_snake_case)]
        #[no_mangle]
        #[test]
        fn $test_name() {
            let matcher = create_matcher::<$suite>($path);
            let result: Option<MatchResult> = matcher.parse($test_message);
            let got_uuid: String = result.unwrap().pattern().uuid().to_hyphenated_string();
            assert_eq!(&got_uuid, $uuid);
        }
    }
}

    #[cfg(test)]
    mod pattern_500 {
        use ::create_matcher;
        use super::*;

        use test_datas::pattern_500::*;
        test_matching!(TrieMatcherSuite_matches, TEST_MESSAGE_WHICH_MATCHES, TrieMatcherSuite, PATTERN_FILE, EXPECTED_UUID);
        test_matching!(TrieMatcherSuiteWithRegexParsers_matches, TEST_MESSAGE_WHICH_MATCHES, TrieMatcherSuiteWithRegexParsers, PATTERN_FILE, EXPECTED_UUID);
        test_matching!(SuffixArrayMatcherSuite_matches, TEST_MESSAGE_WHICH_MATCHES, SuffixArrayMatcherSuite, PATTERN_FILE, EXPECTED_UUID);
        test_matching!(RegexMatcherSuite_matches, TEST_MESSAGE_WHICH_MATCHES, RegexMatcherSuite, PATTERN_FILE, EXPECTED_UUID);
    }
}
