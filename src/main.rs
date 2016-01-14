#[doc="
Counts the frequencies of words read from the standard input, and print a sorted frequency table.

Assumptions:

"]
fn main() {
//    let word_counts = read_and_count(...);
//    let sorted_word_counts = sort_by_value(word_counts);
//    print_counts(sorted_word_counts);
}

type CountTable = std::collections::HashMap<String, usize>;

fn increment_word(map: &mut CountTable, word: String) {
    *map.entry(word).or_insert(0) += 1;
}

fn sort_by_value(map: &CountTable) -> Vec<(&String, &usize)> {
    let mut v = map.iter().collect::<Vec<_>>();
    v.sort_by(|a, b| b.1.cmp(a.1));
    v
}

fn print_counts(v: Vec<(String, usize)>) {
    for elem in v {
        println!("{}: {}", elem.0, elem.1);
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
        let size = v.len();
        for i in 0..(size - 1) {
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
