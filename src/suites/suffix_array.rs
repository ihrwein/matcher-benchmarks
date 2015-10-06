mod suffix_array {
    use actiondb::matcher::Pattern;
    use actiondb::parsers::Parser;
    use actiondb::matcher::result::MatchResult;

    pub trait SuffixArray: Sized {
        type LiteralEntry: LiteralEntry;
        type ParserEntry: ParserEntry;
        fn insert(&mut self, pattern: Pattern);
        fn parse(&self, value: &str) -> Option<MatchResult>;
    }

    pub trait Entry {
        type SA: SuffixArray;
        fn pattern(&self) -> Option<&Pattern>;
        fn parse(&self, value: &str) -> Option<MatchResult>;
        fn set_pattern(&mut self, pattern: Option<Pattern>);
        fn child(&self) -> Option<&Self::SA>;
        fn child_mut(&mut self) -> Option<&mut Self::SA>;
        fn set_child(&mut self, child: Option<Self::SA>);
    }

    pub trait LiteralEntry: Entry {
        fn literal(&self) -> &String;
    }

    pub trait ParserEntry: Entry {
        fn parser(&self) -> &Box<Parser>;
    }

    mod impls {
        use super::{
            SuffixArray,
            Entry,
            LiteralEntry,
            ParserEntry
        };

        use actiondb::parsers::Parser;
        use actiondb::matcher::Pattern;
        use actiondb::matcher::compiled_pattern::TokenType;
        use actiondb::matcher::result::MatchResult;
        use actiondb::utils::CommonPrefix;

        use std::borrow::Borrow;
        use std::cmp::Ordering;

        pub struct SuffixTable {
            literal_entries: Vec<LiteralE>,
            parser_entries: Vec<ParserE>,
        }

        impl SuffixTable {
            pub fn new() -> SuffixTable {
                SuffixTable {
                    literal_entries: Vec::new(),
                    parser_entries: Vec::new()
                }
            }

            pub fn parse_recurse(&self, value: &str) -> Option<MatchResult> {
                let result = self.literal_entries.binary_search_by(|probe| {
                    let s: &str = probe.literal().borrow();
                    s.cmp(value)
                });
                match result {
                    Ok(pos) => {
                        //self.literal_entries.get(pos).expect("Failed to remove")
                        None
                    },
                    Err(pos) => {
                        None
                    }
                }
            }
        }

        impl SuffixArray for SuffixTable {
            type LiteralEntry = LiteralE;
            type ParserEntry = ParserE;

            fn insert(&mut self, mut pattern: Pattern) {
                if let Some(token) = pattern.pop_first_token() {
                    let mut entry: &mut Entry<SA=SuffixTable> = match token {
                        TokenType::Literal(literal) => {
                            let result = self.literal_entries.binary_search_by(|probe| probe.literal().cmp(&literal));
                            match result {
                                Ok(pos) => {
                                    self.literal_entries.get_mut(pos).expect("Failed to remove")
                                },
                                Err(pos) => {
                                    let entry = LiteralE::new(literal);
                                    self.literal_entries.insert(pos, entry);
                                    self.literal_entries.get_mut(pos).expect("Failed to remove")
                                }
                            }
                        },
                        TokenType::Parser(parser) => {
                            let pos = self.parser_entries.iter().position(|x| {
                                x.parser.hash_os() == parser.hash_os()
                            });
                            if let Some(pos) = pos {
                                self.parser_entries.get_mut(pos).expect("Failed to remove parser entry")
                            } else {
                                let parser = ParserE::new(parser);
                                self.parser_entries.push(parser);
                                self.parser_entries.last_mut().expect("Failed to last_mut freshly inserted parser entry")
                            }
                        }
                    };
                    if !pattern.pattern().is_empty() {
                        let sa = SuffixTable::new();
                        entry.set_child(Some(sa));
                        entry.child_mut().expect("Failed to get a child").insert(pattern)
                    } else {
                        entry.set_pattern(Some(pattern));
                    }
                }
            }

            fn parse(&self, value: &str) -> Option<MatchResult> {
                let result = self.literal_entries.binary_search_by(|probe| {
                    let s: &str = probe.literal().borrow();
                    s.cmp(value)
                });
                match result {
                    Ok(pos) => {
                        let pattern = self.literal_entries.get(pos).expect("Failed to get() a literal entry").pattern();
                        Some(MatchResult::new(pattern))
                    },
                    Err(pos) => {
                        if pos == 0 {
                            for parser in &self.parser_entries {
                                if let Some(result) = parser.parse(value) {
                                    return Some(result);
                                }
                            }
                            None
                        } else {
                            let truncated_value = value.ltrunc()
                            None
                        }
                    }
                }
            }
        }

        pub struct ParserE {
            pattern: Option<Pattern>,
            parser: Box<Parser>,
            child: Option<SuffixTable>
        }

        impl ParserE {
            pub fn new(parser: Box<Parser>) -> ParserE {
                ParserE {
                    pattern: None,
                    parser: parser,
                    child: None
                }
            }
        }

        impl Entry for ParserE {
            type SA = SuffixTable;
            fn pattern(&self) -> Option<&Pattern> {
                self.pattern.as_ref()
            }
            fn set_pattern(&mut self, pattern: Option<Pattern>) {
                self.pattern = pattern;
            }
            fn child(&self) -> Option<&SuffixTable> {
                self.child.as_ref()
            }
            fn child_mut(&mut self) -> Option<&mut SuffixTable> {
                self.child.as_mut()
            }
            fn set_child(&mut self, child: Option<Self::SA>) {
                self.child = child;
            }
            fn parse(&self, value: &str) -> Option<MatchResult> {
                None
            }
        }
        impl ParserEntry for ParserE {
            fn parser(&self) -> &Box<Parser> {
                &self.parser
            }
        }

        pub struct LiteralE {
            pattern: Option<Pattern>,
            literal: String,
            child: Option<SuffixTable>
        }

        impl LiteralE {
            pub fn new(literal: String) -> LiteralE {
                LiteralE {
                    literal: literal,
                    pattern: None,
                    child: None
                }
            }
        }
        impl Entry for LiteralE {
            type SA = SuffixTable;
            fn pattern(&self) -> Option<&Pattern> {
                self.pattern.as_ref()
            }
            fn set_pattern(&mut self, pattern: Option<Pattern>) {
                self.pattern = pattern;
            }
            fn child(&self) -> Option<&SuffixTable> {
                self.child.as_ref()
            }
            fn child_mut(&mut self) -> Option<&mut SuffixTable> {
                self.child.as_mut()
            }
            fn set_child(&mut self, child: Option<Self::SA>) {
                self.child = child;
            }
            fn parse(&self, value: &str) -> Option<MatchResult> {
                None
            }
        }

        impl LiteralEntry for LiteralE {
            fn literal(&self) -> &String {
                &self.literal
            }
        }
    }
}
