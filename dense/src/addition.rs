//! Addition of two matrices.
//!
//! This module defines a type [AdditionMat] that represents the addition of two
//! matrices. Two matrices can be added together if they have the same dimension and
//! same index layout, meaning a 1d indexing traverses both matrices in the same order.

use crate::matrix::*;
use crate::matrix_ref::MatrixRef;
use crate::traits::*;
use crate::types::*;
use crate::DefaultLayout;

use std::marker::PhantomData;

/// A type that represents the sum of two matrices.
pub type AdditionMat<Item, MatImpl1, MatImpl2, RS, CS> =
    Matrix<Item, Addition<Item, MatImpl1, MatImpl2, RS, CS>, RS, CS>;

pub struct Addition<Item, MatImpl1, MatImpl2, RS, CS>(
    Matrix<Item, MatImpl1, RS, CS>,
    Matrix<Item, MatImpl2, RS, CS>,
    DefaultLayout,
    PhantomData<RS>,
    PhantomData<CS>,
)
where
    Item: Scalar,
    RS: SizeIdentifier,
    CS: SizeIdentifier,
    MatImpl1: MatrixTrait<Item, RS, CS>,
    MatImpl2: MatrixTrait<Item, RS, CS>;

impl<
        Item: Scalar,
        MatImpl1: MatrixTrait<Item, RS, CS>,
        MatImpl2: MatrixTrait<Item, RS, CS>,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > Addition<Item, MatImpl1, MatImpl2, RS, CS>
{
    pub fn new(mat1: Matrix<Item, MatImpl1, RS, CS>, mat2: Matrix<Item, MatImpl2, RS, CS>) -> Self {
        assert_eq!(
            mat1.layout().dim(),
            mat2.layout().dim(),
            "Dimensions not identical in a + b with a.dim() = {:#?}, b.dim() = {:#?}",
            mat1.layout().dim(),
            mat2.layout().dim()
        );
        let dim = mat1.layout().dim();
        Self(
            mat1,
            mat2,
            DefaultLayout::from_dimension(dim, (1, dim.0)),
            PhantomData,
            PhantomData,
        )
    }
}

impl<
        Item: Scalar,
        MatImpl1: MatrixTrait<Item, RS, CS>,
        MatImpl2: MatrixTrait<Item, RS, CS>,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > Layout for Addition<Item, MatImpl1, MatImpl2, RS, CS>
{
    type Impl = DefaultLayout;

    fn layout(&self) -> &Self::Impl {
        &self.2
    }
}

impl<
        Item: Scalar,
        MatImpl1: MatrixTrait<Item, RS, CS>,
        MatImpl2: MatrixTrait<Item, RS, CS>,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > SizeType for Addition<Item, MatImpl1, MatImpl2, RS, CS>
{
    type C = CS;
    type R = RS;
}

impl<
        Item: Scalar,
        MatImpl1: MatrixTrait<Item, RS, CS>,
        MatImpl2: MatrixTrait<Item, RS, CS>,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > UnsafeRandomAccessByValue for Addition<Item, MatImpl1, MatImpl2, RS, CS>
{
    type Item = Item;

    #[inline]
    unsafe fn get_value_unchecked(
        &self,
        row: crate::types::IndexType,
        col: crate::types::IndexType,
    ) -> Self::Item {
        self.0.get_value_unchecked(row, col) + self.1.get_value_unchecked(row, col)
    }

    #[inline]
    unsafe fn get1d_value_unchecked(&self, index: crate::types::IndexType) -> Self::Item {
        self.0.get1d_value_unchecked(index) + self.1.get1d_value_unchecked(index)
    }
}

impl<
        Item: Scalar,
        MatImpl1: MatrixTrait<Item, RS, CS>,
        MatImpl2: MatrixTrait<Item, RS, CS>,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > std::ops::Add<Matrix<Item, MatImpl2, RS, CS>> for Matrix<Item, MatImpl1, RS, CS>
{
    type Output = AdditionMat<Item, MatImpl1, MatImpl2, RS, CS>;

    fn add(self, rhs: Matrix<Item, MatImpl2, RS, CS>) -> Self::Output {
        Matrix::new(Addition::new(self, rhs))
    }
}

impl<
        'a,
        Item: Scalar,
        MatImpl1: MatrixTrait<Item, RS, CS>,
        MatImpl2: MatrixTrait<Item, RS, CS>,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > std::ops::Add<&'a Matrix<Item, MatImpl2, RS, CS>> for Matrix<Item, MatImpl1, RS, CS>
{
    type Output = AdditionMat<Item, MatImpl1, MatrixRef<'a, Item, MatImpl2, RS, CS>, RS, CS>;

    fn add(self, rhs: &'a Matrix<Item, MatImpl2, RS, CS>) -> Self::Output {
        Matrix::new(Addition::new(self, Matrix::from_ref(rhs)))
    }
}

impl<
        'a,
        Item: Scalar,
        MatImpl1: MatrixTrait<Item, RS, CS>,
        MatImpl2: MatrixTrait<Item, RS, CS>,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > std::ops::Add<Matrix<Item, MatImpl2, RS, CS>> for &'a Matrix<Item, MatImpl1, RS, CS>
{
    type Output = AdditionMat<Item, MatrixRef<'a, Item, MatImpl1, RS, CS>, MatImpl2, RS, CS>;

    fn add(self, rhs: Matrix<Item, MatImpl2, RS, CS>) -> Self::Output {
        Matrix::new(Addition::new(Matrix::from_ref(self), rhs))
    }
}

impl<
        'a,
        Item: Scalar,
        MatImpl1: MatrixTrait<Item, RS, CS>,
        MatImpl2: MatrixTrait<Item, RS, CS>,
        RS: SizeIdentifier,
        CS: SizeIdentifier,
    > std::ops::Add<&'a Matrix<Item, MatImpl2, RS, CS>> for &'a Matrix<Item, MatImpl1, RS, CS>
{
    type Output = AdditionMat<
        Item,
        MatrixRef<'a, Item, MatImpl1, RS, CS>,
        MatrixRef<'a, Item, MatImpl2, RS, CS>,
        RS,
        CS,
    >;

    fn add(self, rhs: &'a Matrix<Item, MatImpl2, RS, CS>) -> Self::Output {
        Matrix::new(Addition::new(Matrix::from_ref(self), Matrix::from_ref(rhs)))
    }
}

#[cfg(test)]

mod test {

    use super::*;

    #[test]
    fn scalar_mult() {
        let mut mat1 = MatrixD::<f64>::zeros_from_dim(2, 3);
        let mut mat2 = MatrixD::<f64>::zeros_from_dim(2, 3);

        mat1[[1, 2]] = 5.0;
        mat2[[1, 2]] = 6.0;

        let res = 2.0 * mat1 + mat2;
        let res = res.eval();

        assert_eq!(res[[1, 2]], 16.0);
    }
}
