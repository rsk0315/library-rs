//! グラフに関するものたち。
//!
//! ここに何かを書く。
pub mod dijkstra;
pub mod dinic_;
pub mod scc;

#[doc(inline)]
pub use dijkstra::dijkstra;
#[doc(inline)]
pub use dinic_::dinic;
#[doc(inline)]
pub use scc::scc;
