use std::ops::Bound::*;

use interval_set::IntervalSet;

fn main() {
    let mut s = IntervalSet::<i32>::new();
    eprintln!("{:?}", s);
    s.insert(1..=1);
    eprintln!("{:?}", s);
    s.insert(2..4);
    eprintln!("{:?}", s);
    s.insert(5..7);
    eprintln!("{:?}", s);
    s.insert(4..6);
    eprintln!("{:?}", s);

    s.insert(10..20);
    eprintln!("{:?}", s);
    s.insert((Excluded(20), Included(30)));
    eprintln!("{:?}", s);
    s.insert(20..20);
    eprintln!("{:?}", s);
    s.insert(20..=20);
    eprintln!("{:?}", s);

    s.remove((Unbounded, Included(100)));
    eprintln!("{:?}", s);

    s.insert(10..20);
    s.insert(12..=22);
    eprintln!("{:?}", s);
    s.insert(10..15);
    eprintln!("{:?}", s);
    s.insert(10..25);
    eprintln!("{:?}", s);

    s.remove(11..12);
    eprintln!("{:?}", s);
    s.remove(13..=13);
    eprintln!("{:?}", s);

    for x in &[9, 10, 11, 12, 13, 14, 24, 25, 26] {
        eprintln!("mex of {}: {:?}", x, s.mex(x));
    }

    s.insert(..=9);
    eprintln!("{:?}", s);
    for x in &[8, 9, 10] {
        eprintln!("mex of {}: {:?}", x, s.mex(x));
    }

    s.insert((Excluded(30), Unbounded));
    eprintln!("{:?}", s);
    for x in &[25, 26, 29, 30, 31] {
        eprintln!("mex of {}: {:?}", x, s.mex(x));
    }

    s.clear();
    s.insert(1..5);
    s.insert(7..=10);
    s.insert(15..);
    eprintln!("{:?}", s);

    eprintln!("{:?}", s.mex(&15));

    {
        let mut s = IntervalSet::<u32>::new();
        s.insert(0..3);
        eprintln!("{:?}", s);
        s.insert(0..2);
        eprintln!("{:?}", s);
        s.insert(1..3);
        eprintln!("{:?}", s);
        s.remove(2..3);
        eprintln!("{:?}", s);
    }

    {
        let mut s = IntervalSet::new();
        s.insert(0..=3);
        eprintln!("{:?}", s);
        s.remove(2..3);
        eprintln!("{:?}", s);
        s.insert(2..3);
        eprintln!("{:?}", s);
        s.remove(2..=3);
        eprintln!("{:?}", s);
        s.insert(2..=3);
        eprintln!("{:?}", s);
        s.remove(0..=0);
        eprintln!("{:?}", s);
        s.remove((Excluded(0), Included(1)));
        eprintln!("{:?}", s);
    }

    {
        let mut s = IntervalSet::new();
        s.insert(0..3);
        s.remove(2..3);
        s.insert(4..6);
        s.remove(5..6);
        eprintln!("{:?}", s);
        eprintln!("{:?}", s.covering(&(4..=4)));
    }
}
