use suffix_array::SuffixArray;

fn main() {
    let text = "abracadabra".to_string();
    let sa: Vec<_> = SuffixArray::from(text).into();
    assert_eq!(sa, [11, 10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]);

    let text = "mississippi".to_string();
    let sa = SuffixArray::from(text.clone());

    for i in sa.search(&"is".chars().collect::<Vec<_>>()) {
        eprintln!("{:?}", &text[i..]);
    }

    for i in sa.search(&"i".chars().collect::<Vec<_>>()) {
        eprintln!("{:?}", &text[i..]);
    }

    let sa: Vec<_> = sa.into();
    assert_eq!(sa, [11, 10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]);

    let text = "aabaaa".to_string();
    let sa = SuffixArray::from(text.clone());
    for pat in &["a", "aa", "b", "ba", "bb", "xyz"] {
        eprintln!(
            "{:?}",
            sa.search(&pat.chars().collect::<Vec<_>>())
                .map(|i| &text[i..])
                .collect::<Vec<_>>()
        );
    }

    let sa: Vec<_> = SuffixArray::from("abababab".to_string()).into();
    assert_eq!(sa, [8, 6, 4, 2, 0, 7, 5, 3, 1]);

    let text = "abracadabra".to_string();
    let sa = SuffixArray::from(text.chars().collect::<Vec<_>>());
    eprintln!("{:?}", sa);
}
