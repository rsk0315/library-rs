//! データ構造たち。
//!
//! ここに何かを書く。
pub mod disjoint_sparse_table;
pub mod foldable_deque;
pub mod foldable_queue;
pub mod interval_set;
pub mod rs_dict;
pub mod union_find;
pub mod vec_act_segtree;
pub mod vec_segtree;
pub mod wavelet_matrix;

#[doc(inline)]
pub use disjoint_sparse_table::DisjointSparseTable;
#[doc(inline)]
pub use foldable_deque::FoldableDeque;
#[doc(inline)]
pub use foldable_queue::FoldableQueue;
#[doc(inline)]
pub use interval_set::IntervalSet;
#[doc(inline)]
pub use rs_dict::RsDict;
#[doc(inline)]
pub use union_find::UnionFind;
#[doc(inline)]
pub use vec_act_segtree::VecActSegtree;
#[doc(inline)]
pub use vec_segtree::VecSegtree;
#[doc(inline)]
pub use wavelet_matrix::WaveletMatrix;
