use std::f64::consts::{PI as PI_f64, SQRT_2 as SQRT_2_f64};


use nalgebra::{matrix, DMatrix, DVector, SMatrix};
use nalgebra::Complex as ComplexBase;
use crate::macros::*;

/// An integer number
pub type Int = usize;

/// A Real number
pub type Real = f32;

/// A complex number
pub type Complex = ComplexBase<Real>;

/// A density matrix
pub type CMatrix = DMatrix<Complex>;

/// A complex vector
pub type CVector = DVector<Complex>;

/// A real vector
pub type RVector = DVector<Real>;

/// A 2x2 complex matrix
pub type Matrix2x2 = SMatrix<Complex, 2, 2>;

pub type Matrix4x4 = SMatrix<Complex, 4, 4>;

pub type ClassicalRegister = Vec<Option<bool>>;

/// An angle in radians
pub type Angle = Real;

/// pi
pub static PI: Angle = PI_f64 as Angle;

/// The square root of two
pub static SQRT_2: Real = SQRT_2_f64 as Real;

/// The acceptable numerical precision for the tests
pub static COMPARISON_PRECISION: Real = 1e-6;

/// The pauli idenitiy matrix
pub static IDENTITY: Matrix2x2 = matrix![
    c!(1., 0.), c!(0., 0.);
    c!(0., 0.), c!(1., 0.);
];

/// The x pauli matrix
pub static SIGMA_X: Matrix2x2 = matrix![
    c!(0., 0.), c!(1., 0.);
    c!(1., 0.), c!(0., 0.);
];

/// the y pauli matrix
pub static SIGMA_Y: Matrix2x2 = matrix![
    c!(0., 0.), c!(0., -1.);
    c!(0., 1.), c!(0., 0.);
];

/// The z pauli matrix
pub static SIGMA_Z: Matrix2x2 = matrix![
    c!(1., 0.), c!(0., 0.);
    c!(0., 0.), c!(-1., 0.);
];

pub static S: Matrix2x2 = matrix![
    c!(1., 0.), c!(0., 0.);
    c!(0., 0.), c!(0., 1.);
];

/// The hadamard matrix
pub static H: Matrix2x2 = matrix![
    c!(1. / SQRT_2 as Real, 0.), c!(1. / SQRT_2 as Real, 0.);
    c!(1. / SQRT_2 as Real, 0.), c!(-1. / SQRT_2 as Real, 0.);
];

pub static CNOT: Matrix4x4 = matrix![
    c!(1., 0.), c!(0., 0.),c!(0., 0.),c!(0., 0.);
    c!(0., 0.), c!(1., 0.),c!(0., 0.),c!(0., 0.);
    c!(0., 0.), c!(0., 0.),c!(0., 0.),c!(1., 0.);
    c!(0., 0.), c!(0., 0.),c!(1., 0.),c!(0., 0.);
];

pub static CZ: Matrix4x4 = matrix![
    c!(1., 0.), c!(0., 0.),c!(0., 0.),c!(0., 0.);
    c!(0., 0.), c!(1., 0.),c!(0., 0.),c!(0., 0.);
    c!(0., 0.), c!(0., 0.),c!(1., 0.),c!(0., 0.);
    c!(0., 0.), c!(0., 0.),c!(0., 0.),c!(-1., 0.);
];

pub static ISWAP: Matrix4x4 = matrix![
    c!(1., 0.), c!(0., 0.), c!(0., 0.), c!(0., 0.);
    c!(0., 0.), c!(0., 0.), c!(0., 1.), c!(0., 0.);
    c!(0., 0.), c!(0., 1.), c!(0., 0.), c!(0., 0.);
    c!(0., 0.), c!(0., 0.), c!(0., 0.), c!(1., 0.);
];

pub static SISWAP: Matrix4x4 = matrix![
    c!(1., 0.), c!(0., 0.), c!(0., 0.), c!(0., 0.);
    c!(0., 0.), c!(1. / SQRT_2, 0.), c!(0., 1. / SQRT_2), c!(0., 0.);
    c!(0., 0.), c!(0., 1. / SQRT_2), c!(1. / SQRT_2, 0.), c!(0., 0.);
    c!(0., 0.), c!(0., 0.), c!(0., 0.), c!(1., 0.);
];


pub static SWAP: Matrix4x4 = matrix![
    c!(1., 0.), c!(0., 0.), c!(0., 0.), c!(0., 0.);
    c!(0., 0.), c!(0., 0.), c!(1., 0.), c!(0., 0.);
    c!(0., 0.), c!(1., 0.), c!(0., 0.), c!(0., 0.);
    c!(0., 0.), c!(0., 0.), c!(0., 0.), c!(1., 0.);
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
        assert!(indices.0 < self.shape.0, "index 0 out of bounds {} > {}", indices.0, self.shape.0);
        assert!(indices.1 < self.shape.1, "index 1 out of bounds {} > {}", indices.1, self.shape.1);
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
    size: usize
}

impl<T> StateVectorPointer<T> {
    pub fn new(value: &mut T, size: usize) -> Self {
        StateVectorPointer {
            pointer: value as *mut T,
            size: size
        }
    }
    pub unsafe fn read(&self, index: usize) -> T {
        assert!(index < self.size, "index out of bounds {} > {}", index, self.size);
        self.pointer.add(index).read()
    }

    pub unsafe fn write(&self, index: usize, value: T) {
        assert!(index < self.size, "index out of bounds {} > {}", index, self.size);
        self.pointer.add(index).write(value)
    }
}

unsafe impl<T> Sync for StateVectorPointer<T> {}
unsafe impl<T> Send for StateVectorPointer<T> {}
