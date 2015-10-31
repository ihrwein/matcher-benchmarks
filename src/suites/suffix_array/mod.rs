use actiondb::matcher::trie::parser_factory::TrieParserFactory;
use actiondb::matcher::MatcherFactory;

use actiondb::matcher::MatcherSuite;
use self::impls::SuffixTable;

mod interface;
mod impls;
#[cfg(test)]
mod test;

pub struct SuffixArrayMatcherFactory;

impl MatcherFactory for SuffixArrayMatcherFactory {
    type Matcher = SuffixTable;

    fn new_matcher() -> Self::Matcher {
        SuffixTable::new()
    }
}

pub struct SuffixArrayMatcherSuite;

impl MatcherSuite for SuffixArrayMatcherSuite {
    type Matcher = SuffixTable;
    type ParserFactory = TrieParserFactory;
    type MatcherFactory = SuffixArrayMatcherFactory;
}
