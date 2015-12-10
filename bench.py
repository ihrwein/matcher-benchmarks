import re
import sys

SuffixArrayMatcherSuite = 0
TrieMatcherSuiteWithRegexParsers = 1
TrieMatcherSuite = 2
RegexMatcherSuite = 3

def get_suite_number(name):
    if name == "TrieMatcherSuite":
        return TrieMatcherSuite
    elif name == "TrieMatcherSuiteWithRegexParsers":
        return TrieMatcherSuiteWithRegexParsers
    elif name == "RegexMatcherSuite":
        return RegexMatcherSuite
    elif name == "SuffixArrayMatcherSuite":
        return SuffixArrayMatcherSuite
    else:
        return None

def get_matching(match_string):
    if match_string == "does_not_match":
        return False
    else:
        return True

class BenchResult:
    def __init__(self, patterns, suite, is_matching, result, spread):
        self.patterns = patterns
        self.suite = suite
        self.is_matching = is_matching
        self.result = result
        self.spread = spread

    def __str__(self):
        return "patterns={} suite={} is_matching={} result={} spread={}".format(self.patterns, self.suite, self.is_matching, self.result, self.spread)

def extract_information_from_line(line):
    match = re.match(r"test\ test_([0-9]+)_patterns::bench_([a-zA-Z]+)_when_message_([a-z_]+)\ +\.\.\.\ +bench:\ +([0-9,]+)\ ns/iter\ \(\+/-\ ([0-9,]+)\)", line)
    if match:
        patterns = match.group(1)
        suite = match.group(2)
        expected_match = match.group(3)
        result = match.group(4)
        spread = match.group(5)
        is_matching = get_matching(expected_match)
        return BenchResult(patterns, suite, is_matching, result, spread)
    else:
        None

def filter_results_by(results, patterns, is_matching):
    filter_fn = lambda x: x.is_matching == is_matching and x.patterns == patterns
    return list(filter(filter_fn, results))

def main():
    results = []
    for line in sys.stdin:
        result = extract_information_from_line(line)
        if result is not None:
            results.append(result)
    print("Suite;Patterns;IsMatching;Result;Spread")
    for i in results:
        line = "{};{};{};{};{}".format(i.suite, i.patterns, i.is_matching, i.result, i.spread)
        print(line)
    #for i in ("100", "200", "300", "400", "500"):
    #    filtered = filter_results_by(results, i, True)
    #    sort_fn = lambda x: x.suite
    #    filtered = sorted(filtered, key=sort_fn)
    #    line = ";".join([x.result for x in filtered])
    #    print(line)

if __name__ == "__main__":
    main()

"""
test test_500_patterns::bench_RegexMatcherSuite_when_message_does_not_match                ... bench:      36,324 ns/iter (+/- 7,187)
"""
