use kmp::KmpSearcher;
use push_pop::PushBack;

fn main() {
    // let kmp: KmpSearcher<_> = "aabaabaaa".into();
    let kmp: KmpSearcher<_> = vec![0, 0, 1, 0, 0, 1, 0, 0, 0].into();
    eprintln!("{:?}", kmp);

    let mut kmp: KmpSearcher<_> = vec![].into();
    for &x in &[0, 0, 1, 0, 0, 1, 0, 0, 0] {
        kmp.push_back(x);
        // eprintln!("{:#?}", kmp);
    }

    let kmp = kmp;
    let text = [2, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0];
    for r in kmp.occurrences(&text) {
        eprintln!("{:?}", r);
    }

    // let kmp: KmpSearcher<i32> = vec![].into();
    let kmp = KmpSearcher::<i32>::from(vec![]);
    for r in kmp.occurrences(&text) {
        eprintln!("{:?}", r);
    }

    let kmp = KmpSearcher::<i32>::from(vec![0, 0, 1, 0, 0, 1, 0, 0, 0]);
    eprintln!("{:?}", kmp);
    for r in kmp.occurrences(&text) {
        eprintln!("{:?}", r);
    }
    let o: Vec<_> = kmp.occurrences(&text).collect();
    eprintln!("{:?}", o);

    eprintln!(
        "{:?}",
        KmpSearcher::from(vec![0, 0, 1, 0, 2, 0, 0, 1, 0, 2])
    );

    eprintln!("{:?}", KmpSearcher::from(vec![0; 5]));

    // s[1] = "b"; s[2] = "a"; s[n] = s[n-1] s[n-2]
    // b a ab aba abaab abaababa abaababaabaab
    let buf = vec![0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1];
    let mut k_dyn: KmpSearcher<i32> = vec![].into();
    for i in 0..buf.len() {
        k_dyn.push_back(buf[i]);
        let k: KmpSearcher<_> = buf[..=i].to_vec().into();
        assert_eq!(k, k_dyn);
    }

    eprintln!("{:?}", k_dyn);
}
