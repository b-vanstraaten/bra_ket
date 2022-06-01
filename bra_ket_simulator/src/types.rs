use std::f64::consts::{PI as PI_f64, SQRT_2 as SQRT_2_f64};

use nalgebra::{matrix, Complex, DMatrix, DVector, SMatrix};

/// A real number
pub type R = f64;

/// A complex number
pub type C = Complex<R>;

/// A density matrix
pub type CMatrix = DMatrix<C>;

/// A state vector
pub type CVector = DVector<C>;

/// A 2x2 complex matrix
pub type Matrix2x2 = SMatrix<C, 2, 2>;

pub type Matrix4x4 = SMatrix<C, 4, 4>;

pub type ClassicalRegister = DVector<R>;

/// An angle
pub type Angle = R;

/// pi
pub static PI: Angle = PI_f64 as Angle;
pub static SQRT_2: R = SQRT_2_f64 as R;

/// The acceptable numerical precision for the test
pub static COMPARISON_PRECISION: R = 1e-6;

/// The pauli idenitiy matrix
pub static IDENTITY: Matrix2x2 = matrix![
    C::new(1., 0.), C::new(0., 0.);
    C::new(0., 0.), C::new(1., 0.);
];

/// The x pauli matrix
pub static SIGMA_X: Matrix2x2 = matrix![
    C::new(0., 0.), C::new(1., 0.);
    C::new(1., 0.), C::new(0., 0.);
];

/// the y pauli matrix
pub static SIGMA_Y: Matrix2x2 = matrix![
    C::new(0., 0.), C::new(0., -1.);
    C::new(0., 1.), C::new(0., 0.);
];

/// The z pauli matrix
pub static SIGMA_Z: Matrix2x2 = matrix![
    C::new(1., 0.), C::new(0., 0.);
    C::new(0., 0.), C::new(-1., 0.);
];

/// The hadamard matrix
pub static H: Matrix2x2 = matrix![
    C::new(1. / SQRT_2 as R, 0.), C::new(1. / SQRT_2 as R, 0.);
    C::new(1. / SQRT_2 as R, 0.), C::new(-1. / SQRT_2 as R, 0.);
];

pub static CNOT: Matrix4x4 = matrix![
    C::new(1., 0.), C::new(0., 0.),C::new(0., 0.),C::new(0., 0.);
    C::new(0., 0.), C::new(1., 0.),C::new(0., 0.),C::new(0., 0.);
    C::new(0., 0.), C::new(0., 0.),C::new(0., 0.),C::new(1., 0.);
    C::new(0., 0.), C::new(0., 0.),C::new(1., 0.),C::new(0., 0.);
];

pub static CZ: Matrix4x4 = matrix![
    C::new(1., 0.), C::new(0., 0.),C::new(0., 0.),C::new(0., 0.);
    C::new(0., 0.), C::new(1., 0.),C::new(0., 0.),C::new(0., 0.);
    C::new(0., 0.), C::new(0., 0.),C::new(1., 0.),C::new(0., 0.);
    C::new(0., 0.), C::new(0., 0.),C::new(0., 0.),C::new(-1., 0.);
];

pub static ISWAP: Matrix4x4 = matrix![
    C::new(1., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
    C::new(0., 0.), C::new(0., 0.), C::new(0., 1.), C::new(0., 0.);
    C::new(0., 0.), C::new(0., 1.), C::new(0., 0.), C::new(0., 0.);
    C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(1., 0.);
];

pub static SISWAP: Matrix4x4 = matrix![
    C::new(1., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
    C::new(0., 0.), C::new(1. / SQRT_2 as R, 0.), C::new(0., 1. / SQRT_2 as R), C::new(0., 0.);
    C::new(0., 0.), C::new(0., 1. / SQRT_2 as R), C::new(1. / SQRT_2 as R, 0.), C::new(0., 0.);
    C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(1., 0.);
];

pub static S: Matrix2x2 = matrix![
    C::new(1., 0.), C::new(0., 0.);
    C::new(0., 0.), C::new(0., 1.);
];

pub static SWAP: Matrix4x4 = matrix![
    C::new(1., 0.), C::new(0., 0.), C::new(0., 0.), C::new(0., 0.);
    C::new(0., 0.), C::new(0., 0.), C::new(1., 0.), C::new(0., 0.);
    C::new(0., 0.), C::new(1., 0.), C::new(0., 0.), C::new(0., 0.);
    C::new(0., 0.), C::new(0., 0.), C::new(0., 0.), C::new(1., 0.);
];

#[derive(Debug, Clone)]
pub struct DensityMatrixPointer<T> {
    pointer: *mut T,
    shape: (usize, usize),
}

impl<T> DensityMatrixPointer<T> {
    pub fn new(value: &mut T, shape: (usize, usize)) -> Self {
        DensityMatrixPointer {
            pointer: value as *mut T,
            shape,
        }
    }

    pub fn flatten_index(&self, indices: (usize, usize)) -> usize {
        indices.0 + self.shape.1 * indices.1
    }

    pub unsafe fn offset(&self, indices: (usize, usize)) -> *mut T {
        self.pointer.add(self.flatten_index(indices) as usize)
    }

    pub unsafe fn read(&self, indices: (usize, usize)) -> T {
        self.offset(indices).read()
    }

    pub unsafe fn write(&self, indices: (usize, usize), value: T) {
        self.offset(indices).write(value)
    }
}

unsafe impl<T> Sync for DensityMatrixPointer<T> {}
unsafe impl<T> Send for DensityMatrixPointer<T> {}

#[derive(Debug, Clone)]
pub struct StateVectorPointer<T> {
    pointer: *mut T,
    size: usize,
}

impl<T> StateVectorPointer<T> {
    pub fn new(value: &mut T, size: usize) -> Self {
        StateVectorPointer {
            pointer: value as *mut T,
            size,
        }
    }
    pub unsafe fn read(&self, indices: usize) -> T {
        self.pointer.add(indices).read()
    }

    pub unsafe fn write(&self, indices: usize, value: T) {
        self.pointer.add(indices).write(value)
    }
}

unsafe impl<T> Sync for StateVectorPointer<T> {}
unsafe impl<T> Send for StateVectorPointer<T> {}
