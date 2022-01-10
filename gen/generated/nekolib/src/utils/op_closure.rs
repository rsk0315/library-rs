//! クロージャの wrapper クラス。

use super::super::traits::binop;

use std::fmt::Debug;

use binop::{Associative, Identity, Magma};

/// 任意の結合的な演算を持つ。
///
/// 結合性については使用者側に保証してもらう。
///
/// # Examples
/// ```
/// use std::cell::RefCell;
///
/// use nekolib::traits::{Magma, Identity};
/// use nekolib::utils::OpClosure;
///
/// let runtime_mod = 10;
/// let op_memo = RefCell::new(vec![]);
/// let id_times = RefCell::new(0);
/// let op_cl = OpClosure::new(|x: i32, y| {
///     op_memo.borrow_mut().push((x, y));
///     (x + y).rem_euclid(runtime_mod)
/// }, || {
///     *id_times.borrow_mut() += 1;
///     0
/// });
///
/// assert_eq!(op_cl.op(1, 9), 0);
/// assert_eq!(op_cl.op(3, 8), 1);
/// assert_eq!(op_cl.op(-5, -3), 2);
/// assert_eq!(op_cl.id(), 0);
///
/// assert_eq!(*op_memo.borrow(), [(1, 9), (3, 8), (-5, -3)]);
/// assert_eq!(*id_times.borrow(), 1);
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct OpClosure<T, Op: Fn(T, T) -> T, Id: Fn() -> T>(Op, Id);

impl<T: Eq, Op: Fn(T, T) -> T, Id: Fn() -> T> OpClosure<T, Op, Id> {
    pub fn new(op: Op, id: Id) -> Self { Self(op, id) }
}

impl<T: Eq, Op: Fn(T, T) -> T, Id: Fn() -> T> Magma for OpClosure<T, Op, Id> {
    type Set = T;
    fn op(&self, lhs: T, rhs: T) -> T { (self.0)(lhs, rhs) }
}

impl<T: Eq, Op: Fn(T, T) -> T, Id: Fn() -> T> Identity
    for OpClosure<T, Op, Id>
{
    fn id(&self) -> T { (self.1)() }
}

impl<T: Eq, Op: Fn(T, T) -> T, Id: Fn() -> T> Associative
    for OpClosure<T, Op, Id>
{
}
