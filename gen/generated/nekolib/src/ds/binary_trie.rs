use std::fmt::{self, Debug};
use std::ops::AddAssign;

#[derive(Debug)]
pub struct BinaryTrie<I> {
    head: Link<I>,
}

type Link<I> = Option<Box<Node<I>>>;

struct Node<I> {
    sum0: usize,
    sum1: I,
    next: [Link<I>; 2],
}

impl<I: Debug> Debug for Node<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entry(&"sum0", &self.sum0)
            .entry(&"sum1", &self.sum1)
            .entry(&"next", &[
                self.next[0].as_ref().map(|_| ..),
                self.next[1].as_ref().map(|_| ..),
            ])
            .finish()
    }
}

impl<I: BinaryInt> BinaryTrie<I> {
    pub fn new() -> Self { Self { head: None } }

    pub fn insert(&mut self, elem: I) {
        let mut cursor = &mut self.head;
        for bit in elem.bits() {
            let tmp = cursor.get_or_insert_with(|| Self::new_node());
            tmp.sum0 += 1;
            tmp.sum1 += elem;
            cursor = &mut tmp.next[bit as usize];
        }
        let tmp = cursor.get_or_insert_with(|| Self::new_node());
        tmp.sum0 += 1;
        tmp.sum1 += elem;
    }

    pub fn iter(&self) -> Iter<'_, I> { Iter::new(&self) }

    pub fn iter_dup(
        &self,
    ) -> impl '_ + Iterator<Item = I> + DoubleEndedIterator {
        self.iter().flat_map(|(x, i)| (0..i).map(move |_| x))
    }

    fn new_node() -> Box<Node<I>> {
        Box::new(Node { sum0: 0, sum1: I::zero(), next: [None, None] })
    }
}

#[derive(Debug)]
pub struct Iter<'a, I> {
    // trie: &'a BinaryTrie<I>,
    left_path: Vec<(&'a Box<Node<I>>, usize)>,
    right_path: Vec<(&'a Box<Node<I>>, usize)>,
    left_int: I,
    right_int: I,
}

impl<'a, I: BinaryInt> Iter<'a, I> {
    fn new(trie: &'a BinaryTrie<I>) -> Iter<I> {
        let (left_path, left_int) = Self::descend(trie, 0);
        let (right_path, right_int) = Self::descend(trie, 1);
        Self { left_path, left_int, right_path, right_int }
    }

    fn descend(
        trie: &'a BinaryTrie<I>,
        fst: usize,
    ) -> (Vec<(&Box<Node<I>>, usize)>, I) {
        let mut int = I::zero();
        let cursor = trie.head.as_ref();
        let mut path = vec![];
        Self::descend_inner(cursor, 0, &mut path, &mut int, fst);
        (path, int)
    }

    fn descend_inner(
        mut cursor: Option<&'a Box<Node<I>>>,
        mut dir: usize,
        path: &mut Vec<(&'a Box<Node<I>>, usize)>,
        int: &mut I,
        fst: usize,
    ) {
        while let Some(next) = cursor {
            path.push((next, dir));
            if let Some(fst_path) = &next.next[fst] {
                int.push(fst != 0);
                cursor = Some(&fst_path);
                dir = fst;
            } else if let Some(snd_path) = &next.next[fst ^ 1] {
                int.push((fst ^ 1) != 0);
                cursor = Some(&snd_path);
                dir = fst ^ 1;
            } else {
                break;
            }
        }
    }

    fn next_dir(&mut self, dir: usize) -> Option<(I, usize)> {
        // (値, 個数) を返したい？

        let Self { left_int, left_path, right_int, right_path } = self;
        if left_path.is_empty() {
            return None;
        }
        let res = if dir == 0 { *left_int } else { *right_int };
        let count = {
            let path_last = if dir == 0 {
                left_path.last().unwrap()
            } else {
                right_path.last().unwrap()
            };
            path_last.0.sum0
        };

        if left_int == right_int {
            left_path.clear();
            right_path.clear();
        }

        let path = if dir == 0 { left_path } else { right_path };
        let int = if dir == 0 { left_int } else { right_int };

        let mut last_dir = dir ^ 1;
        while let Some((node, cur_dir)) = path.pop() {
            if let Some(next) = &node.next[dir ^ 1] {
                if last_dir == dir {
                    path.push((node, cur_dir));
                    int.push(dir == 0);
                    Self::descend_inner(Some(next), dir ^ 1, path, int, dir);
                    break;
                }
            }

            int.pop();
            last_dir = cur_dir;
        }
        Some((res, count))
    }
}

impl<I: BinaryInt> Iterator for Iter<'_, I> {
    type Item = (I, usize);
    fn next(&mut self) -> Option<Self::Item> { self.next_dir(0) }
}

impl<I: BinaryInt> DoubleEndedIterator for Iter<'_, I> {
    fn next_back(&mut self) -> Option<Self::Item> { self.next_dir(1) }
}

pub trait BinaryInt: Copy + AddAssign<Self> + Eq + Debug {
    fn zero() -> Self;
    fn bits(self) -> Bits<Self>;
    fn test(self, shift: u32) -> bool;
    fn push(&mut self, bit: bool);
    fn pop(&mut self);
}

pub struct Bits<I> {
    val: I,
    shift: u32,
}

impl<I: BinaryInt> Iterator for Bits<I> {
    type Item = bool;
    fn next(&mut self) -> Option<bool> {
        if self.shift == 0 {
            return None;
        }
        self.shift -= 1;
        Some(self.val.test(self.shift))
    }
}

macro_rules! impl_uint {
    ( $($ty:ty)* ) => { $(
        impl BinaryInt for $ty {
            fn zero() -> Self { 0 }
            fn bits(self) -> Bits<Self> {
                let bits = (0 as $ty).count_zeros();
                Bits { val: self, shift: bits }
            }
            fn test(self, shift: u32) -> bool {
                self >> shift & 1 != 0
            }
            fn push(&mut self, bit: bool) {
                *self <<= 1;
                *self |= bit as $ty;
            }
            fn pop(&mut self) {
                *self >>= 1;
            }
        }
    )* }
}

impl_uint! { u8 u16 u32 u64 u128 usize }

#[test]
fn test() {
    let mut bt = BinaryTrie::<u8>::new();
    // eprintln!("{:?}", bt);
    bt.insert(10);
    // eprintln!("{:?}", bt);
    bt.insert(3);
    // eprintln!("{:?}", bt);
    bt.insert(1);
    // eprintln!("{:?}", bt);
    bt.insert(3);
    // eprintln!("{:#?}", bt.iter());
    bt.insert(0);
    bt.insert(14);
    bt.insert(100);

    for x in bt.iter().take(10) {
        eprintln!("{x:?}");
    }

    for x in bt.iter().rev().take(10) {
        eprintln!("{x:?}");
    }

    for x in bt.iter_dup().take(10) {
        eprintln!("{x:?}");
    }

    for x in bt.iter_dup().rev().take(10) {
        eprintln!("{x:?}");
    }
}

// ```
// bt.insert(0);
// bt.insert(0);
// bt.remove(0);
// bt.insert(1);
// bt.insert(3);
// bt.insert(3);
// bt.insert(10);
// bt.insert(14);
// bt.insert(100);
// bt.iter_dup().collect();    // {0, 1, 3, 3, 10, 14, 100}
// bt.count(.., 3);            // 2
// bt.count_3way(.., 3);       // {lt: 2, eq: 2, gt: 3}
// bt.count(.., 3..=11);       // 3
// bt.count_3way(.., 3..=11);  // {lt: 2, eq: 3, gt: 2}
// bt.sum(.., 3..=11);         // 16
// bt.sum_3way(.., 3..=11);    // {lt: 1, eq: 16, gt: 114}
// bt.sum(.., ..);             // 131
// bt.quantile(.., 4);         // 10
// bt.quantile_sum(.., 4);     // 7
// ```
