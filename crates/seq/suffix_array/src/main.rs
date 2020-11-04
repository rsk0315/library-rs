use suffix_array::SuffixArray;

fn main() {
    let pat = "abracadabra";
    let sa: Vec<_> = SuffixArray::from(pat).into();
    assert_eq!(sa, [11, 10, 7, 0, 3, 5, 8, 1, 4, 6, 9, 2]);

    let pat = "mississippi";
    let sa = SuffixArray::from(pat);

    for &i in sa.search("is") {
        eprintln!("{:?}", &pat[i..]);
    }

    for &i in sa.search("i") {
        eprintln!("{:?}", &pat[i..]);
    }

    let sa: Vec<_> = sa.into();
    assert_eq!(sa, [11, 10, 7, 4, 1, 0, 9, 8, 6, 3, 5, 2]);
}
