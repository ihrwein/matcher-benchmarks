use super::interface::{
    SuffixArray,
    Entry,
    LiteralEntry,
    ParserEntry
};

use actiondb::parsers::Parser;
use actiondb::matcher::{
    Matcher,
    Pattern
};
use actiondb::matcher::compiled_pattern::TokenType;
use actiondb::matcher::result::MatchResult;
use actiondb::utils::CommonPrefix;

use std::borrow::Borrow;

#[derive(Debug, Clone)]
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
    fn longest_common_prefix_around_pos(&self, value: &str, pos: usize) -> (usize, usize) {
        let mut min_pos = pos;
        let mut min_len = 0;
        if pos > 0 {
            min_pos = pos - 1;
        }

        for i in min_pos..pos+1 {
            if let Some(entry) = self.literal_entries.get(i) {
                let len = entry.literal().common_prefix_len(value);
                if len > min_len {
                    min_pos = i;
                    min_len = len;
                }
            }
        }
        (min_pos, min_len)
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

    fn longest_common_prefix(&self, value: &str) -> Option<(usize, usize)> {
        let result = self.literal_entries.binary_search_by(|probe| {
            let s: &str = probe.literal().borrow();
            s.cmp(value)
        });
        match result {
            Ok(pos) => {
                let child = self.literal_entries.get(pos).expect("Failed to get() a literal entry");
                let common_prefix_len = child.literal().common_prefix_len(value);
                Some((pos, common_prefix_len))
            },
            Err(pos) => {
                let (min_pos, min_len) = self.longest_common_prefix_around_pos(value, pos);
                if min_len > 0 {
                    Some((min_pos, min_len))
                } else {
                    None
                }
            },
        }
    }

}

#[derive(Debug)]
pub struct ParserE {
    pattern: Option<Pattern>,
    parser: Box<Parser>,
    child: Option<SuffixTable>
}

impl Clone for ParserE {
    fn clone(&self) -> ParserE {
        ParserE {
            pattern: self.pattern.clone(),
            parser: self.parser.boxed_clone(),
            child: self.child.clone()
        }
    }
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
}
impl ParserEntry for ParserE {
    fn parser(&self) -> &Box<Parser> {
        &self.parser
    }
    fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<MatchResult<'a, 'b>> {
        if let Some(kvpair) = self.parser.parse(value) {
            let value = value.ltrunc(kvpair.value().len());

            match self.child() {
                Some(child) => {
                    if let Some(mut result) = child.parse(value) {
                        result.insert(kvpair);
                        Some(result)
                    } else {
                        None
                    }
                },
                None => {
                    if value.is_empty() {
                        let mut result = MatchResult::new(self.pattern().expect("Failed to get the pattern"));
                        result.insert(kvpair);
                        Some(result)
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
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
}

impl LiteralEntry for LiteralE {
    fn literal(&self) -> &String {
        &self.literal
    }
}

impl Matcher for SuffixTable {
    fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<MatchResult<'a, 'b>> {
        match self.longest_common_prefix(value) {
            Some((pos, len)) => {
                let child = self.literal_entries.get(pos).expect("Failed to get() a literal entry");
                if len == value.len() {
                    if let Some(pattern) = child.pattern() {
                        Some(MatchResult::new(pattern))
                    } else {
                        None
                    }
                } else if len < value.len() {
                    let value = value.ltrunc(len);
                    if let Some(child) = child.child() {
                        child.parse(value)
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            None => {
                for parser in &self.parser_entries {
                    if let Some(result) = parser.parse(value) {
                        return Some(result);
                    }
                }
                None
            }
        }
    }
    fn add_pattern(&mut self, pattern: Pattern) {
        self.insert(pattern);
    }
    fn boxed_clone(&self) -> Box<Matcher> {
        Box::new(self.clone())
    }
}
