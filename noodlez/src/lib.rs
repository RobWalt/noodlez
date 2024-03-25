// #![warn(
//     clippy::all,
//     clippy::restriction,
//     clippy::pedantic,
//     clippy::nursery,
//     clippy::cargo
// )]

pub mod storage;

pub mod prelude {
    pub use crate::storage::graph::slotmap_graph::Graph;
    pub use crate::storage::traits::GraphStorage;
}
