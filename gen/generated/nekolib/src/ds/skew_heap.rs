pub struct SkewHeap<T> {
    root: Link<T>,
    len: usize,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Self { Self { elem, left: None, right: None } }
}

impl<T: Ord> SkewHeap<T> {
    pub fn new() -> Self { SkewHeap { root: None, len: 0 } }

    pub fn len(&self) -> usize { self.len }
    pub fn is_empty(&self) -> bool { self.len == 0 }
    pub fn clear(&mut self) { while self.pop().is_some() {} }

    pub fn peek(&self) -> Option<&T> {
        self.root.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<PeekMut<'_, T>> {
        if self.is_empty() {
            None
        } else {
            Some(PeekMut { heap: self, tainted: false })
        }
    }

    pub fn push(&mut self, elem: T) {
        self.meld(Self { root: Some(Box::new(Node::new(elem))), len: 1 });
    }

    pub fn pop(&mut self) -> Option<T> {
        self.root.take().map(|node| {
            self.len -= 1;
            self.root = Self::meld_internal(node.left, node.right);
            node.elem
        })
    }

    pub fn meld(&mut self, other: Self) {
        self.len += other.len;
        self.root = Self::meld_internal(self.root.take(), other.root);
    }

    fn meld_internal(left: Link<T>, right: Link<T>) -> Link<T> {
        match (left, right) {
            (left, None) => left,
            (None, right) => right,
            (Some(mut left), Some(right)) if left.elem >= right.elem => {
                std::mem::swap(&mut left.left, &mut left.right);
                left.left = Self::meld_internal(left.left.take(), Some(right));
                Some(left)
            }
            (left, right) => Self::meld_internal(right, left),
        }
    }

    fn fix_root(&mut self) {
        let root = &self.root.as_ref().unwrap().elem;
        let left = &self.root.as_ref().unwrap().left;
        let right = &self.root.as_ref().unwrap().right;
        if left.as_ref().map(|left| &left.elem > root).unwrap_or(false)
            || right.as_ref().map(|right| &right.elem > root).unwrap_or(false)
        {
            let tmp = self.pop().unwrap();
            self.push(tmp);
        }
    }
}

impl<T> SkewHeap<T> {
    #[cfg(test)]
    fn push_with_dir(&mut self, elem: T, dir: &[bool]) {
        let mut from = &mut self.root;
        for &is_right in dir.iter().chain(std::iter::once(&false)) {
            if let Some(v) = from {
                from = if is_right { &mut v.right } else { &mut v.left };
            } else if from.is_none() {
                *from = Some(Box::new(Node::new(elem)));
                return;
            }
        }
    }

    #[cfg(test)]
    fn in_order(&self) -> impl Iterator<Item = &T> {
        fn dfs<'a, T>(v: &'a Link<T>, vec: &mut Vec<&'a T>) {
            let v = match v {
                Some(v) => v,
                None => return,
            };
            dfs(&v.left, vec);
            vec.push(&v.elem);
            dfs(&v.right, vec);
        }

        let mut res = vec![];
        dfs(&self.root, &mut res);
        res.into_iter()
    }

    fn bfs_order(&self) -> impl Iterator<Item = (&T, Option<&T>, Option<&T>)> {
        use std::collections::VecDeque;

        let mut res = vec![];
        let mut q = VecDeque::new();

        q.extend(self.root.as_ref());
        while let Some(v) = q.pop_front() {
            let left = v.left.as_ref();
            let right = v.right.as_ref();
            res.push((&v.elem, left.map(|o| &o.elem), right.map(|o| &o.elem)));
            q.extend(left.into_iter().chain(right));
        }

        res.into_iter()
    }
}

impl<T: Ord> Extend<T> for SkewHeap<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item);
        }
    }
}

use std::iter::FromIterator;

impl<T: Ord> FromIterator<T> for SkewHeap<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut heap = Self::new();
        heap.extend(iter);
        heap
    }
}

pub struct IntoIter<T>(SkewHeap<T>);

impl<T: Ord> IntoIterator for SkewHeap<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter { IntoIter(self) }
}

impl<T: Ord> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> { self.0.pop() }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len, Some(self.0.len))
    }
}

impl<T: Ord> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize { self.0.len }
}

pub struct PeekMut<'a, T: 'a + Ord> {
    heap: &'a mut SkewHeap<T>,
    tainted: bool,
}

use std::ops::{Deref, DerefMut};

impl<T: Ord> Drop for PeekMut<'_, T> {
    fn drop(&mut self) {
        if self.tainted {
            self.heap.fix_root();
        }
    }
}

impl<T: Ord> Deref for PeekMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &T { &self.heap.root.as_ref().unwrap().elem }
}

impl<T: Ord> DerefMut for PeekMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.tainted = true;
        &mut self.heap.root.as_mut().unwrap().elem
    }
}

impl<'a, T: Ord> PeekMut<'a, T> {
    pub fn pop(mut self) -> T {
        let value = self.heap.pop().unwrap();
        self.tainted = false;
        value
    }
}

impl<T: Ord> Default for SkewHeap<T> {
    fn default() -> Self { Self::new() }
}

use std::fmt;

impl<T: fmt::Debug> fmt::Debug for SkewHeap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.bfs_order().map(|(x, ..)| x)).finish()
    }
}

#[cfg(test)]
mod test {
    use super::SkewHeap;

    #[test]
    fn push_with_dir() {
        let mut tr = SkewHeap::new();
        tr.push_with_dir(1, &[]);
        tr.push_with_dir(4, &[false]);
        tr.push_with_dir(23, &[true]);
        tr.push_with_dir(63, &[false, false]);
        tr.push_with_dir(24, &[false, true]);
        tr.push_with_dir(44, &[true, false]);
        tr.push_with_dir(28, &[true, true]);

        assert_eq!(tr.in_order().copied().collect::<Vec<_>>(), [
            63, 4, 24, 1, 44, 23, 28
        ]);
    }

    #[test]
    fn meld() {
        let mut left = SkewHeap::new();
        left.push_with_dir(-1, &[]);
        left.push_with_dir(-4, &[false]);
        left.push_with_dir(-23, &[true]);
        left.push_with_dir(-63, &[false, false]);
        left.push_with_dir(-24, &[false, true]);
        left.push_with_dir(-44, &[true, false]);
        left.push_with_dir(-28, &[true, true]);

        assert_eq!(left.in_order().copied().collect::<Vec<_>>(), [
            -63, -4, -24, -1, -44, -23, -28
        ]);

        let mut right = SkewHeap::new();
        right.push_with_dir(-13, &[]);
        right.push_with_dir(-17, &[false]);
        right.push_with_dir(-99, &[true]);
        right.push_with_dir(-57, &[false, false]);
        right.push_with_dir(-49, &[false, true]);
        right.push_with_dir(-105, &[true, false]);
        right.push_with_dir(-201, &[true, true]);

        assert_eq!(right.in_order().copied().collect::<Vec<_>>(), [
            -57, -17, -49, -13, -105, -99, -201
        ]);

        left.meld(right);
        let melded: Vec<_> = left
            .bfs_order()
            .map(|(&v, l, r)| (v, l.copied(), r.copied()))
            .collect();
        assert_eq!(melded, [
            (-1, Some(-13), Some(-4)),
            (-13, Some(-23), Some(-17)),
            (-4, Some(-63), Some(-24)),
            (-23, Some(-28), Some(-44)),
            (-17, Some(-57), Some(-49)),
            (-63, None, None),
            (-24, None, None),
            (-28, Some(-99), None),
            (-44, None, None),
            (-57, None, None),
            (-49, None, None),
            (-99, Some(-105), Some(-201)),
            (-105, None, None),
            (-201, None, None),
        ]);
    }

    #[test]
    fn heap_sort() {
        let mut a = vec![2, 9, 7, 3, 1, 6, 3, 5, 4, 8];
        let mut heap = SkewHeap::new();
        heap.extend(a.iter().copied());
        a.sort_unstable();
        a.reverse();
        let mut b = vec![];
        while let Some(x) = heap.pop() {
            b.push(x);
        }
        assert_eq!(a, b);

        assert_eq!(heap.len(), 0);
        assert_eq!(heap.pop(), None);
        assert_eq!(heap.pop(), None);
        assert_eq!(heap.len(), 0);
    }

    #[test]
    fn into_iter() {
        let heap: SkewHeap<_> =
            [2, 5, 1, 3, 4, 6, 6, 1].iter().copied().collect();
        assert_eq!(heap.len(), 8);

        let mut iter = heap.into_iter();
        assert_eq!(iter.size_hint(), (8, Some(8)));
        assert_eq!(iter.len(), 8);
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.size_hint(), (7, Some(7)));
        assert_eq!(iter.len(), 7);
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.len(), 5);
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.len(), 4);
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.len(), 3);
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.len(), 2);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.len(), 0);
    }

    #[test]
    fn peek() {
        use std::cmp::Reverse;

        let mut heap: SkewHeap<_> =
            [2, 5, 3, 1, 6, 2].iter().map(|&x| Reverse(x)).collect();

        assert_eq!(heap.peek(), Some(&Reverse(1)));
        heap.peek_mut().unwrap().0 -= 1; // [0, 2, 2, 3, 5, 6]
        assert_eq!(heap.peek(), Some(&Reverse(0)));
        heap.peek_mut().unwrap().0 += 4; // [2, 2, 3, 4, 5, 6]
        assert_eq!(heap.peek(), Some(&Reverse(2)));
        assert_eq!(heap.peek_mut().unwrap().pop().0, 2); // [2, 3, 4, 5, 6]
        assert_eq!(heap.peek(), Some(&Reverse(2)));
        assert_eq!(heap.peek_mut().unwrap().0, 2);
        assert_eq!(heap.peek_mut().unwrap().pop().0, 2); // [3, 4, 5, 6]
        assert_eq!(heap.peek(), Some(&Reverse(3)));
        assert_eq!(heap.pop(), Some(Reverse(3))); // [4, 5, 6]
        assert_eq!(heap.peek(), Some(&Reverse(4)));

        heap.peek_mut().unwrap().0 += 10; // [5, 6, 14]
        assert_eq!(heap.peek(), Some(&Reverse(5)));

        heap.push(Reverse(8)); // [5, 6, 8, 14]
        assert_eq!(heap.peek(), Some(&Reverse(5)));
        assert_eq!(heap.pop(), Some(Reverse(5))); // [6, 8, 14]
        assert_eq!(heap.pop(), Some(Reverse(6))); // [8, 14]
        assert_eq!(heap.pop(), Some(Reverse(8))); // [14]
        assert_eq!(heap.peek_mut().unwrap().pop().0, 14);
        assert!(heap.peek_mut().is_none());
        assert!(heap.peek().is_none());
    }
}
