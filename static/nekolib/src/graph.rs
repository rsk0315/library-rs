//! グラフに関するものたち。
//!
//! ここに何かを書く。
pub mod dijkstra_;
pub mod dinic_;
pub mod scc_;

#[doc(inline)]
pub use dijkstra_::dijkstra;
#[doc(inline)]
pub use dinic_::dinic;
#[doc(inline)]
pub use scc_::scc;
