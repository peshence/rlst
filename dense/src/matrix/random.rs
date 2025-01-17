//! Methods for the creation of random matrices.

use crate::data_container::DataContainerMut;
use crate::tools::*;
use crate::traits::*;
use crate::types::*;
use rand::prelude::*;
use rand_distr::StandardNormal;

use super::GenericBaseMatrixMut;

macro_rules! rand_impl {
    ($Scalar:ty) => {
        impl<RS: SizeIdentifier, CS: SizeIdentifier, Data: DataContainerMut<Item = $Scalar>>
            GenericBaseMatrixMut<$Scalar, Data, RS, CS>
        {
            /// Fill a matrix with normally distributed random numbers.
            pub fn fill_from_rand_standard_normal<R: Rng>(&mut self, rng: &mut R) {
                let dist = StandardNormal;
                self.for_each(|val| *val = <$Scalar>::random_scalar(rng, &dist));
            }
        }
    };
}

rand_impl!(f32);
rand_impl!(f64);
rand_impl!(c32);
rand_impl!(c64);
