//! An indexable vector space has elements that can be indexed as n-dimensional vectors.

use mpi::traits::*;
use std::marker::PhantomData;

use super::index_layout::DistributedIndexLayout;
use super::indexable_vector::DistributedIndexableVector;
use crate::traits::index_layout::IndexLayout;
use crate::traits::indexable_vector::Inner;
use rlst_common::types::{IndexType, Scalar};
use rlst_operator::{Element, IndexableSpace, InnerProductSpace};

pub struct DistributedIndexableVectorSpace<'comm, T: Scalar + Equivalence, C: Communicator> {
    index_layout: &'comm DistributedIndexLayout<'comm, C>,
    _phantom: PhantomData<T>,
}

impl<'comm, T: Scalar + Equivalence, C: Communicator> DistributedIndexableVectorSpace<'comm, T, C> {
    pub fn new(index_layout: &'comm DistributedIndexLayout<'comm, C>) -> Self {
        DistributedIndexableVectorSpace {
            index_layout,
            _phantom: PhantomData,
        }
    }
}

pub struct DistributedIndexableVectorSpaceElement<
    'space,
    'comm,
    T: Scalar + Equivalence,
    C: Communicator,
> {
    space: &'space DistributedIndexableVectorSpace<'comm, T, C>,
    data: super::indexable_vector::DistributedIndexableVector<'comm, T, C>,
}

impl<'space, 'comm, T: Scalar + Equivalence, C: Communicator> Element
    for DistributedIndexableVectorSpaceElement<'space, 'comm, T, C>
where
    T::Real: Equivalence,
{
    type Space = DistributedIndexableVectorSpace<'comm, T, C>;
    type View<'b> = &'b super::indexable_vector::DistributedIndexableVector<'comm, T, C> where Self: 'b;
    type ViewMut<'b> = &'b mut super::indexable_vector::DistributedIndexableVector<'comm, T, C> where Self: 'b;

    fn space(&self) -> &Self::Space {
        self.space
    }

    fn view<'b>(&'b self) -> Self::View<'b> {
        &self.data
    }

    fn view_mut<'b>(&'b mut self) -> Self::ViewMut<'b> {
        &mut self.data
    }
}

impl<'comm, T: Scalar + Equivalence, C: Communicator> rlst_operator::LinearSpace
    for DistributedIndexableVectorSpace<'comm, T, C>
where
    T::Real: Equivalence,
{
    type F = T;
    type E<'space> = DistributedIndexableVectorSpaceElement<'space, 'comm, T, C> where Self: 'space;

    fn create_element<'space>(&'space self) -> Self::E<'space> {
        DistributedIndexableVectorSpaceElement {
            space: &self,
            data: DistributedIndexableVector::<'comm, T, C>::new(&self.index_layout),
        }
    }
}

impl<'a, T: Scalar + Equivalence, C: Communicator> IndexableSpace
    for DistributedIndexableVectorSpace<'a, T, C>
where
    T::Real: Equivalence,
{
    fn dimension(&self) -> IndexType {
        self.index_layout.number_of_global_indices()
    }
}

impl<'a, T: Scalar + Equivalence, C: Communicator> InnerProductSpace
    for DistributedIndexableVectorSpace<'a, T, C>
where
    T::Real: Equivalence,
{
    fn inner<'b>(
        &self,
        x: &rlst_operator::ElementView<'b, Self>,
        other: &rlst_operator::ElementView<'b, Self>,
    ) -> rlst_common::types::SparseLinAlgResult<Self::F>
    where
        Self: 'b,
    {
        x.inner(other)
    }
}
