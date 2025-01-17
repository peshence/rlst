use crate::traits::index_layout::IndexLayout;
use rlst_common::types::{IndexType, RlstError, RlstResult};

pub struct DefaultSerialIndexLayout {
    size: IndexType,
}

impl DefaultSerialIndexLayout {
    pub fn new(size: IndexType) -> Self {
        Self { size }
    }
}

impl IndexLayout for DefaultSerialIndexLayout {
    fn number_of_local_indices(&self) -> IndexType {
        self.number_of_global_indices()
    }

    fn local_range(&self) -> (IndexType, IndexType) {
        (0, self.size)
    }

    fn number_of_global_indices(&self) -> IndexType {
        self.size
    }

    fn index_range(&self, rank: IndexType) -> RlstResult<(IndexType, IndexType)> {
        if rank == 0 {
            Ok((0, self.size))
        } else {
            Err(RlstError::MpiRankError(rank as i32))
        }
    }

    fn local2global(&self, index: IndexType) -> Option<IndexType> {
        if index < self.number_of_local_indices() {
            Some(index)
        } else {
            None
        }
    }

    fn global2local(&self, rank: IndexType, index: IndexType) -> Option<IndexType> {
        if rank == 0 && index < self.number_of_global_indices() {
            Some(index)
        } else {
            None
        }
    }

    fn rank_from_index(&self, index: IndexType) -> Option<IndexType> {
        if index < self.number_of_global_indices() {
            Some(0)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_local_index_layout() {
        let index_layout = DefaultSerialIndexLayout::new(14);

        // Test that the range is correct on rank 0
        assert_eq!(index_layout.index_range(0).unwrap(), (0, 14));

        // Test that the number of global indices is correct.
        assert_eq!(index_layout.number_of_global_indices(), 14);

        // Test that map works

        assert_eq!(index_layout.local2global(2).unwrap(), 2);
    }
}
