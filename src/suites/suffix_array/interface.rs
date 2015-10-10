use actiondb::matcher::Pattern;
use actiondb::parsers::Parser;
use actiondb::matcher::result::MatchResult;

pub trait SuffixArray: Clone {
    type LiteralEntry: LiteralEntry;
    type ParserEntry: ParserEntry;
    fn insert(&mut self, pattern: Pattern);
    fn longest_common_prefix(&self, value: &str) -> Option<(usize, usize)>;
}

pub trait Entry {
    type SA: SuffixArray;
    fn pattern(&self) -> Option<&Pattern>;
    fn set_pattern(&mut self, pattern: Option<Pattern>);
    fn child(&self) -> Option<&Self::SA>;
    fn child_mut(&mut self) -> Option<&mut Self::SA>;
    fn set_child(&mut self, child: Option<Self::SA>);
}

pub trait LiteralEntry: Entry + Clone {
    fn literal(&self) -> &String;

}

pub trait ParserEntry: Entry + Clone {
    fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<MatchResult<'a, 'b>>;
    fn parser(&self) -> &Box<Parser>;
}
