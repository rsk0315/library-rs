//! グラフに関するものたち。
//!
//! ここに何かを書く。
pub mod adjlist;
pub mod dijkstra_;
pub mod dinic_;
pub mod functional_graph;
pub mod hld;
pub mod scc_;
pub mod tree_cata;

#[doc(inline)]
pub use adjlist::from_root;
#[doc(inline)]
pub use dijkstra_::dijkstra;
#[doc(inline)]
pub use dinic_::dinic;
#[doc(inline)]
pub use functional_graph::FunctionalGraph;
#[doc(inline)]
pub use hld::{Direction, HlEdge, Hld};
#[doc(inline)]
pub use scc_::scc;
#[doc(inline)]
pub use tree_cata::TreeCata;
