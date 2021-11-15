//! 削除可能ヒープ。

use std::collections::BinaryHeap;

/// 削除可能ヒープ。
///
/// # Idea
/// ヒープを二つ使う。
///
/// # Implementation notes
/// `peek()` を `&self` にするため、`remove()` と `pop()` の直後はとりあえず
/// `force()` している。
///
/// # Examples
/// ```
/// use nekolib::ds::RemovableHeap;
///
/// let mut heap = RemovableHeap::new();
/// heap.push(2);
/// heap.push(1);
/// heap.push(3);
/// heap.push(0);
/// assert_eq!(heap.peek(), Some(&3)); // {0, 1, 2, 3}
///
/// heap.remove(2);                    // {0, 1, 3}
/// assert_eq!(heap.pop(), Some(3));   // {0, 1}
/// assert_eq!(heap.peek(), Some(&1));
/// heap.remove(1);                    // {0}
/// assert_eq!(heap.peek(), Some(&0));
/// assert_eq!(heap.pop(), Some(0));   // {}
/// assert!(heap.is_empty());
/// ```
///
/// 削除する要素がヒープに入っていないとき、こまる。
/// ```
/// use nekolib::ds::RemovableHeap;
///
/// let mut heap = RemovableHeap::new();
/// heap.push(1);
/// heap.remove(2);
/// heap.push(2);
/// heap.push(3);                      // {1, 2, 3}?
/// assert_eq!(heap.pop(), Some(3));   // seems good?
/// assert_ne!(heap.peek(), Some(&2)); // but...
/// assert_eq!(heap.peek(), Some(&1));
/// ```
///
/// 未来の push 操作を打ち消すというわけでもない。
/// ```
/// use nekolib::ds::RemovableHeap;
///
/// let mut heap = RemovableHeap::new();
/// heap.push(1);
/// heap.push(2);
/// heap.remove(100);
/// heap.remove(3);
/// heap.push(101);
/// heap.push(3);                      // {1, 2, 101}?
/// assert_eq!(heap.pop(), Some(101)); // seems good, so far
/// assert_ne!(heap.peek(), Some(&2)); // but...
/// assert_eq!(heap.peek(), Some(&3));
/// ```
#[derive(Clone)]
pub struct RemovableHeap<T> {
    alive: BinaryHeap<T>,
    dead: BinaryHeap<T>,
    len: usize,
}

impl<T: Ord> RemovableHeap<T> {
    /// 空のヒープで初期化する。
    pub fn new() -> Self {
        Self {
            alive: BinaryHeap::new(),
            dead: BinaryHeap::new(),
            len: 0,
        }
    }

    /// 要素を追加する。
    pub fn push(&mut self, item: T) {
        self.len += 1;
        self.alive.push(item);
    }

    /// 要素を削除する。
    ///
    /// # Notes
    /// 削除処理を遅延させて行うため、`&T` ではなく `T` であることに注意。
    ///
    /// また、`item` はその時点でヒープに入っている要素のいずれかと等しい必要がある。
    /// 「未来に追加される要素を打ち消す」あるいは「単に無視する」という仕様は、
    /// どちらも操作順しだいでどちらもうまくいかなくなるはず。
    ///
    /// `BTreeSet` を使えば判定できるが、それなら最初から `BTreeSet` でやればよい。
    pub fn remove(&mut self, item: T) {
        self.len -= 1;
        self.dead.push(item);
        self.force();
    }

    /// 最大値を取り出す。
    pub fn pop(&mut self) -> Option<T> {
        self.force();
        let res = self.alive.pop();
        if res.is_some() {
            self.len -= 1;
            self.force();
        }
        res
    }

    /// 最大値を取得する。
    pub fn peek(&self) -> Option<&T> { self.alive.peek() }

    /// 空のとき `true` を返す。
    pub fn is_empty(&self) -> bool { self.len == 0 }

    /// 要素数を返す。
    pub fn len(&self) -> usize { self.len }

    fn force(&mut self) {
        while let (Some(x), Some(y)) = (self.alive.peek(), self.dead.peek()) {
            if x != y {
                break;
            }
            self.alive.pop();
            self.dead.pop();
        }
    }
}

#[test]
fn test() {
    let mut heap = RemovableHeap::new();
    assert_eq!(heap.pop(), None);
    heap.push(1); // {1}
    assert_eq!(heap.peek(), Some(&1));
    assert_eq!(heap.len(), 1);
    heap.push(2); // {1, 2}
    assert_eq!(heap.peek(), Some(&2));
    assert_eq!(heap.len(), 2);
    heap.remove(1); // {2}
    assert_eq!(heap.len(), 1);
    assert_eq!(heap.pop(), Some(2)); // {}
    assert_eq!(heap.len(), 0);
    assert_eq!(heap.pop(), None); // {}
    assert_eq!(heap.len(), 0);

    heap.push(2); // {2}
    heap.push(1); // {1, 2}
    assert_eq!(heap.len(), 2);
    heap.remove(2); // {1}
    assert_eq!(heap.len(), 1);
    assert_eq!(heap.pop(), Some(1)); // {}
    assert_eq!(heap.len(), 0);
}
