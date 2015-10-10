use actiondb::matcher::trie::ParserTrie;
use actiondb::matcher::trie::parser_factory::TrieParserFactory;
use actiondb::matcher::trie::factory::TrieMatcherFactory;

use MatcherSuite;

pub struct TrieMatcherSuite;

impl MatcherSuite for TrieMatcherSuite {
    type Matcher = ParserTrie;
    type ParserFactory = TrieParserFactory;
    type MatcherFactory = TrieMatcherFactory;
}
