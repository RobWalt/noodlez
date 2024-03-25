pub mod storage;

pub mod prelude {
    pub use crate::storage::graph::slotmap_graph::Graph;
    pub use crate::storage::traits::GraphStorage;
}
