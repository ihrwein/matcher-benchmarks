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
        regex: Regex
    }

    macro_rules! regex_parser_body {
        () => {
            fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<ParseResult<'a, 'b>> {
                if let Some((_, end)) = self.regex.find(value) {
                    Some(ParseResult::new(self, &value[..end]))
                } else {
                    None
                }
            }
            fn name(&self) -> Option<&str> {
                self.name.as_ref().map(|name| name.borrow())
            }
            fn set_name(&mut self, name: Option<String>) {
                self.name = name;
            }
        }
    }

    impl Parser for SetParser {
        regex_parser_body!();

        fn boxed_clone(&self) -> Box<Parser> {
            let parser = SetParser {
                name: self.name.clone(),
                regex: self.regex.clone()
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
        regex: Regex
    }

    impl Parser for GreedyParser {
        regex_parser_body!();

        fn boxed_clone(&self) -> Box<Parser> {
            let parser = GreedyParser {
                name: self.name.clone(),
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

    impl ParserFactory for RegexParserFactory {
        fn new_set<'a>(set: &str, name: Option<&str>, opt_params: Option<Vec<OptionalParameter<'a>>>) -> Box<Parser> {
            let regex = Regex::new(&format!("^[{}]", set)).ok().expect("Failed to create a SetParser");
            let parser = SetParser {
                name: name.map(|name| name.to_string()),
                regex: regex
            };

            Box::new(parser)
        }
        fn new_int<'a>(name: Option<&str>, opt_params: Option<Vec<OptionalParameter<'a>>>) -> Box<Parser> {
            let regex = Regex::new(&format!("^[0123456789]")).ok().expect("Failed to create an IntParser");
            let parser = IntParser {
                delegate: SetParser {
                    name: name.map(|name| name.to_string()),
                    regex: regex
                }
            };
            Box::new(parser)
        }
        fn new_greedy<'a>(name: Option<&str>, end_string: Option<&str>) -> Box<Parser> {
            let regex = match end_string {
                Some(end_string) => {
                    Regex::new(&format!("{}", end_string)).ok().expect("Failed to create a GreedyParser")
                },
                None => {
                    Regex::new(&format!("$")).ok().expect("Failed to create a GreedyParser")
                }
            };
            let parser = GreedyParser {
                name: name.map(|name| name.to_string()),
                regex: regex
            };
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
            let mut remaining_text = text;
            for pattern in &self.patterns {
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

use MatcherSuite;

pub struct RegexMatcherSuite;

impl MatcherSuite for RegexMatcherSuite {
    type Matcher = RegexMatcher;
    type ParserFactory = RegexParserFactory;
    type MatcherFactory = RegexMatcherFactory;

    fn parser_factory() -> Self::ParserFactory {
        RegexParserFactory
    }
    fn matcher_factory() -> Self::MatcherFactory {
        RegexMatcherFactory
    }
}
