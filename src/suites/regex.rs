pub mod parsers {
    use actiondb::parsers::{
        ObjectSafeHash,
        OptionalParameter,
        Parser,
        ParseResult,
        ParserFactory
    };

    use std::borrow::Borrow;
    use std::hash::{
        Hash,
        Hasher,
        SipHasher,
    };

    use regex::Regex;

    #[derive(Debug, Clone)]
    pub struct SetParser {
        name: Option<String>,
        regex: Regex,
        min_len: Option<usize>,
        max_len: Option<usize>,
    }

    impl SetParser {
        pub fn set_min_length(&mut self, value: usize) {
            self.min_len = Some(value);
        }
        pub fn set_max_length(&mut self, value: usize) {
            self.max_len = Some(value);
        }
    }

    macro_rules! regex_parser_parse {
        () => {
            fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<ParseResult<'a, 'b>> {
                if let Some((_, end)) = self.regex.find(value) {
                    Some(ParseResult::new(self, &value[..end]))
                } else {
                    None
                }
            }
        }
    }

    macro_rules! regex_parser_name {
        () => {
            fn name(&self) -> Option<&str> {
                self.name.as_ref().map(|name| name.borrow())
            }
            fn set_name(&mut self, name: Option<String>) {
                self.name = name;
            }
        }
    }

    impl Parser for SetParser {
        regex_parser_name!();

        fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<ParseResult<'a, 'b>> {
            if let Some((_, end)) = self.regex.find(value) {
                let parsed_value = &value[..end];
                if let Some(min_len) = self.min_len {
                    if parsed_value.len() < min_len {
                        return None;
                    }
                }

                if let Some(max_len) = self.max_len {
                    if parsed_value.len() > max_len {
                        return None;
                    }
                }

                Some(ParseResult::new(self, &value[..end]))
            } else {
                None
            }
        }

        fn boxed_clone(&self) -> Box<Parser> {
            let parser = SetParser {
                name: self.name.clone(),
                regex: self.regex.clone(),
                min_len: self.min_len.clone(),
                max_len: self.max_len.clone()
            };
            Box::new(parser)
        }
    }

    impl ObjectSafeHash for SetParser {
        fn hash_os(&self) -> u64 {
            let mut hasher = SipHasher::new();
            "parser:set".hash(&mut hasher);
            hasher.finish()
        }
    }

    #[derive(Debug)]
    pub struct IntParser {
        delegate: SetParser
    }

    impl Parser for IntParser {
        fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<ParseResult<'a, 'b>> {
            self.delegate.parse(value)
        }
        fn name(&self) -> Option<&str> {
            self.delegate.name()
        }
        fn set_name(&mut self, name: Option<String>) {
            self.delegate.set_name(name)
        }
        fn boxed_clone(&self) -> Box<Parser> {
            let parser = IntParser {
                delegate: self.delegate.clone()
            };
            Box::new(parser)
        }
    }

    impl ObjectSafeHash for IntParser {
        fn hash_os(&self) -> u64 {
            let mut hasher = SipHasher::new();
            "parser:int".hash(&mut hasher);
            hasher.finish()
        }
    }

    #[derive(Debug)]
    pub struct GreedyParser {
        name: Option<String>,
        end_string: Option<String>,
        regex: Regex
    }

    impl GreedyParser {
        pub fn new(name: Option<&str>, end_string: Option<&str>) -> GreedyParser {
            let regex = match end_string {
                Some(end_string) => {
                    Regex::new(&format!("{}", end_string)).ok().expect("Failed to create a GreedyParser")
                },
                None => {
                    Regex::new(".*").ok().expect("Failed to create a GreedyParser")
                }
            };
            GreedyParser {
                name: name.map(|name| name.to_string()),
                end_string: end_string.map(|end| end.to_string()),
                regex: regex
            }
        }
    }

    impl Parser for GreedyParser {
        regex_parser_name!();

        fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<ParseResult<'a, 'b>> {
            if self.end_string.is_none() {
                return Some(ParseResult::new(self, value));
            }
            if let Some((_, end)) = self.regex.find(value) {
                Some(ParseResult::new(self, &value[..end]))
            } else {
                None
            }
        }

        fn boxed_clone(&self) -> Box<Parser> {
            let parser = GreedyParser {
                name: self.name.clone(),
                end_string: self.end_string.clone(),
                regex: self.regex.clone()
            };
            Box::new(parser)
        }
    }

    impl ObjectSafeHash for GreedyParser {
        fn hash_os(&self) -> u64 {
            let mut hasher = SipHasher::new();
            "parser:greedy".hash(&mut hasher);
            hasher.finish()
        }
    }

    pub struct RegexParserFactory;

    macro_rules! set_optinal_param {
        ($parser:expr, $param:expr) => {
            match $param {
                OptionalParameter::Int(key, value) => {
                    match key {
                        "min_len" => {
                            $parser.set_min_length(value);
                        },
                        "max_len" => {
                            $parser.set_max_length(value);
                        },
                        _ => ()
                    }
                }
            }
        }
    }

    macro_rules! set_optional_params {
        ($parser:expr, $opt_params:expr) => {
            if let Some(opt_params) = $opt_params {
                for i in opt_params.into_iter() {
                    set_optinal_param!($parser, i);
                }
            }
        }
    }

    impl ParserFactory for RegexParserFactory {
        fn new_set<'a>(set: &str, name: Option<&str>, opt_params: Option<Vec<OptionalParameter<'a>>>) -> Box<Parser> {
            let regex = Regex::new(&format!("^[{}]+", set)).ok().expect("Failed to create a SetParser");
            let mut parser = SetParser {
                name: name.map(|name| name.to_string()),
                regex: regex,
                min_len: None,
                max_len: None
            };

            set_optional_params!(&mut parser, opt_params);
            Box::new(parser)
        }
        fn new_int<'a>(name: Option<&str>, opt_params: Option<Vec<OptionalParameter<'a>>>) -> Box<Parser> {
            let regex = Regex::new(&format!("^[0123456789]+")).ok().expect("Failed to create an IntParser");
            let mut parser = IntParser {
                delegate: SetParser {
                    name: name.map(|name| name.to_string()),
                    regex: regex,
                    min_len: None,
                    max_len: None
                }
            };
            set_optional_params!(&mut parser.delegate, opt_params);
            Box::new(parser)
        }
        fn new_greedy<'a>(name: Option<&str>, end_string: Option<&str>) -> Box<Parser> {
            let parser = GreedyParser::new(name, end_string);
            Box::new(parser)
        }
    }
}

pub mod matcher {
    use actiondb::Matcher;
    use actiondb::matcher::factory::MatcherFactory;
    use actiondb::matcher::result::MatchResult;
    use actiondb::matcher::pattern::Pattern;
    use actiondb::matcher::compiled_pattern::TokenType;

    #[derive(Debug)]
    pub struct RegexMatcher {
        patterns: Vec<Pattern>
    }

    impl Matcher for RegexMatcher {
        fn parse<'a, 'b>(&'a self, text: &'b str) -> Option<MatchResult<'a, 'b>> {
            for pattern in &self.patterns {
                let mut remaining_text = text;
                let mut match_result = MatchResult::new(pattern);
                for token in pattern.pattern() {
                    match *token {
                        TokenType::Literal(ref literal) => {
                            if remaining_text.starts_with(literal) {
                                remaining_text = &remaining_text[literal.len() ..];
                            } else {
                                break;
                            }
                        },
                        TokenType::Parser(ref parser) => {
                            if let Some(result) = parser.parse(remaining_text) {
                                remaining_text = &remaining_text[result.value().len() ..];
                                match_result.insert(result);
                            } else {
                                break;
                            }
                        }
                    };
                    if remaining_text.is_empty() {
                        return Some(match_result);
                    }
                }
            }
            None
        }
        fn add_pattern(&mut self, pattern: Pattern) {
            self.patterns.push(pattern);
        }
        fn boxed_clone(&self) -> Box<Matcher> {
            let clone = RegexMatcher {
                patterns: self.patterns.clone()
            };
            Box::new(clone)
        }
    }

    pub struct RegexMatcherFactory;

    impl MatcherFactory for RegexMatcherFactory {
        type Matcher = RegexMatcher;

        fn new_matcher() -> Self::Matcher {
            RegexMatcher {
                patterns: Vec::new()
            }
        }
    }
}

pub use self::parsers::RegexParserFactory;
pub use self::matcher::{
    RegexMatcher,
    RegexMatcherFactory
};

use actiondb::matcher::MatcherSuite;

pub struct RegexMatcherSuite;

impl MatcherSuite for RegexMatcherSuite {
    type Matcher = RegexMatcher;
    type ParserFactory = RegexParserFactory;
    type MatcherFactory = RegexMatcherFactory;
}
