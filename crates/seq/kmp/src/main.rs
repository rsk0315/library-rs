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
}
