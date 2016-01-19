// Kevin Wilde
// NETID: kjw731
// EECS 395

#[doc="
Counts the frequencies of words read from the standard input, and prints a sorted frequency table.

Assumptions:
    Words do not include any characters except alphabetic characters and apostrophes and periods (to allow acronyms).
    The program is not case sensitive. `Hello` and `hello` count as the same word.
    Numbers are not words.
    If there is a number in the middle of a group of letters, like `ab3cd`, this will result in the program
    counting `ab` and `cd`.
    Apostrophes are part of words as long as they are not at the beginnig or end. 
    This allows the program to count contractions, but to trim apostrophes that surround words.
    Examples:
        won't vs. wont vs. won -- DIFFERENT
        'hello' vs. 'hello vs. hello -- SAME
    Periods are trimmed from the beginning and end of words. Abbreviations like `etc.` will be counted as just `etc`.
    Acronyms separated by periods will have the last period removed (ex. `E.E.C.S.` would show up in the output as `e.e.c.s`).
    Hyphenated words are split into separate words. Thus, `He is good-looking` would count `good` and `looking` separately.
    
    Output:
        Words are printed in lowercase in descending order of frequency.
        Words with the same frequency are printed in no particular order.
"]

use std::io::{BufRead, BufReader, Read, stdin};

fn main() {
    let mut map = CountTable::new();
    let word_counts = read_and_count(&mut map);
    print_counts(&word_counts);
}

type CountTable = std::collections::HashMap<String, usize>;

fn read_and_count(map: &mut CountTable) -> Vec<(&String, &usize)> {
    let v: Vec<String> = read_input(stdin());    
    for w in v {
        increment_word(map, w);
    }
    sort_by_value(map)
}

fn read_input<R: Read>(reader: R) -> Vec<String> {
    let mut v = std::vec::Vec::new();
    let mut lines = BufReader::new(reader).lines();
    while let Some(Ok(line)) = lines.next() {
        let lower_line = line.to_lowercase();
        let tmp: Vec<&str> = lower_line.split(|c: char| !c.is_alphabetic() && c != '\'' && c != '.').collect();
        for elem in tmp {            
            let mut word = &elem[..];
            let mut size = word.len();
            while size > 0 && (word.chars().nth(0).unwrap() == '\'' || word.chars().nth(0).unwrap() == '.') {
                word = &word[1..];
                size -= 1;
            }
            while size > 0 && (word.chars().nth(size-1).unwrap() == '\'' || word.chars().nth(size-1).unwrap() == '.') {
                word = &word[..(size-1)];
                size -= 1;
            }
            if word.len() > 0 {
                v.push(word.to_owned());
            }
        }
    }
    v
}

#[cfg(test)]
mod read_measurements_tests {
    use super::{read_input};
    use std::io::{Read, Result};

    #[test]
    fn reads_three_words_on_separate_lines() {
        assert_read(&["hi", "hello", "hey"], "'...''hi...\nhello\nhey\n");
    }

    #[test]
    fn reads_three_words_on_same_line() {
        assert_read(&["hi", "hello", "hey"], "hi hello hey\n");
    }

    #[test]
    fn trims_beginning_and_trailing_apostrophes_and_periods() {
        assert_read(&["hi", "hello", "hey"], "'...''hi...\n''...'..'hello...'\n'..'..'.hey\n");
    }

    #[test]
    fn handles_acronyms() {
        assert_read(&["i", "am", "a", "student", "in", "the", "e.e.c.s", "department"], "I am a student \nin the E.E.C.S. department!!\n");
    }

    #[test]
    fn splits_on_invalid_chars() {
        assert_read(&["hi", "my", "name", "is", "kevin", "i", "don't", "like", "the", "one"], "hi8 my name&is Kevin. I don't like the # 3.14159. One=1.")
    }

    #[test]
    fn splits_on_invalid_chars_multi_line() {
        assert_read(&["hi", "my", "name", "is", "kevin", "i", "don't", "like", "the", "one"], "hi8 my\nname&is Kevin!!!!\n I don't\nlike the # 3. One=1.\n%$#^$^")
    }

    fn assert_read(expected: &[&str], input: &str) {
        let mock_read = StringReader::new(input.to_owned());
        let v = read_input(mock_read);
        assert_eq!(expected.len(), v.len());
        for i in 0..(v.len()) {
            assert_eq!(expected[i], v[i]);
        }
    }
    
    struct StringReader {
        contents: Vec<u8>,
        position: usize,
    }

    impl StringReader {
        fn new(s: String) -> Self {
            StringReader {
                contents: s.into_bytes(),
                position: 0,
            }
        }
    }

    impl Read for StringReader {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let mut count = 0;
            while self.position < self.contents.len() && count < buf.len() {
                buf[count] = self.contents[self.position];
                count += 1;
                self.position += 1;
            }
            Ok(count)
        }
    }
}

fn increment_word(map: &mut CountTable, word: String) {
    *map.entry(word).or_insert(0) += 1;
}

fn sort_by_value(map: &CountTable) -> Vec<(&String, &usize)> {
    let mut v = map.iter().collect::<Vec<_>>();
    v.sort_by(|a, b| b.1.cmp(a.1));
    v
}

fn print_counts(v: &Vec<(&String, &usize)>) {
    for elem in v {
        println!("{}: {}", *(elem.0), *(elem.1));
    }
}

#[cfg(test)]
mod increment_word_tests {
    use super::increment_word;
    use super::CountTable;
 
    #[test]
    fn insert_if_empty() {
        let mut h = CountTable::new();
        increment_word(&mut h, "one".to_owned());

        assert_eq!(Some(&1), h.get("one"));
        assert_eq!(1, h.len());
    }

    #[test]
    fn increments_if_present() {
        let mut under_test = fixture();
        let mut expected = fixture();

        increment_word(&mut under_test, "three".to_owned());
        expected.insert("three".to_owned(), 4);
        assert_eq!(expected, under_test);
    }

    #[test]
    fn insert_if_absent() {
        let mut under_test = fixture();
        let mut expected = fixture();

        increment_word(&mut under_test, "one".to_owned());
        expected.insert("one".to_owned(), 1);
        assert_eq!(expected, under_test);
    }

    fn fixture() -> CountTable {
        let mut h = CountTable::new();
        h.insert("two".to_owned(), 2);
        h.insert("three".to_owned(), 3);
        
        assert_eq!(None, h.get("one"));
        assert_eq!(Some(&2), h.get("two"));
        assert_eq!(Some(&3), h.get("three"));
        assert_eq!(2, h.len());
        h
    }
}

#[cfg(test)]
mod sort_by_value_tests {
    use super::sort_by_value;
    use super::CountTable;

    #[test]
    fn sort_nonempty_hashmap() {
        let h = fixture();
        let v = sort_by_value(&h);
        let v_size = v.len();
        assert_eq!(v_size, h.len());
        for i in 0..(v_size - 1) {
            assert!(v[i].1 >= v[i+1].1);
        }
    }

    fn fixture() -> CountTable {
        let mut h = CountTable::new();
        h.insert("two".to_owned(), 2);
        h.insert("three".to_owned(), 3);
        
        assert_eq!(None, h.get("one"));
        assert_eq!(Some(&2), h.get("two"));
        assert_eq!(Some(&3), h.get("three"));
        assert_eq!(2, h.len());
        h
    }
}
