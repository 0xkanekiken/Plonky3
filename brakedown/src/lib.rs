#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use p3_code::SystematicCode;
use p3_field::field::Field;
use p3_field::matrix::dense::{DenseMatrixView, DenseMatrixViewMut};
use p3_field::matrix::mul::mul_csr_dense;
use p3_field::matrix::sparse::CsrMatrix;
use p3_field::matrix::Matrix;

pub struct BrakedownCode<F: Field, IC: SystematicCode<F>> {
    pub a: CsrMatrix<F>,
    pub b: CsrMatrix<F>,
    pub inner_code: Box<IC>,
}

impl<F: Field, IC: SystematicCode<F>> BrakedownCode<F, IC> {
    fn y_len(&self) -> usize {
        self.a.height()
    }

    fn z_len(&self) -> usize {
        self.y_len() + self.z_parity_len()
    }

    fn z_parity_len(&self) -> usize {
        self.inner_code.parity_len()
    }

    fn v_len(&self) -> usize {
        self.b.height()
    }
}

impl<F: Field, IC: SystematicCode<F>> SystematicCode<F> for BrakedownCode<F, IC> {
    fn systematic_len(&self) -> usize {
        self.a.width()
    }

    fn parity_len(&self) -> usize {
        self.y_len() + self.z_parity_len() + self.v_len()
    }

    fn write_parity(&self, x: DenseMatrixView<F>, parity: &mut DenseMatrixViewMut<F>) {
        let (mut z, mut v) = parity.split_rows(self.z_len());
        let (mut y, mut z_parity) = z.split_rows(self.y_len());

        mul_csr_dense(&self.a, x, &mut y);
        self.inner_code.write_parity(y.as_view(), &mut z_parity);
        mul_csr_dense(&self.b, z.as_view(), &mut v);
    }
}