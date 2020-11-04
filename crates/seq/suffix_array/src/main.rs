use suffix_array::SuffixArray;

fn main() {
    let sa: Vec<_> = SuffixArray::from("abracadabra").into();
    assert_eq!(sa, [11, 10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]);

    let sa: Vec<_> = SuffixArray::from("mississippi").into();
    assert_eq!(sa, [11, 10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]);
}
