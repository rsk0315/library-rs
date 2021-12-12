//! グラフに関するものたち。
//!
//! ここに何かを書く。
pub mod dijkstra_;
pub mod dinic_;
pub mod functional_graph;
pub mod scc_;
pub mod tree;

#[doc(inline)]
pub use dijkstra_::dijkstra;
#[doc(inline)]
pub use dinic_::dinic;
#[doc(inline)]
pub use functional_graph::FunctionalGraph;
#[doc(inline)]
pub use scc_::scc;
#[doc(inline)]
pub use tree::Tree;
