use nalgebra::{Complex, DMatrix, SMatrix};

pub type Qubit = usize;
pub type R = f64;
pub type C = Complex<R>;
pub type DensityMatrix = DMatrix<C>;
pub type Matrix2x2 = SMatrix<C, 2, 2>;
pub type Angle = R;

pub static PI: Angle = std::f64::consts::PI as Angle;
pub static COMPARISON_PRECISION: R = 1e-6;

pub static IDENTITY: Matrix2x2 = Matrix2x2::new(
    C::new(1., 0.),
    C::new(0., 0.),
    C::new(0., 0.),
    C::new(1., 0.),
);

pub static SIGMA_X: Matrix2x2 = Matrix2x2::new(
    C::new(0., 0.),
    C::new(1., 0.),
    C::new(1., 0.),
    C::new(0., 0.),
);

pub static SIGMA_Y: Matrix2x2 = Matrix2x2::new(
    C::new(0., 0.),
    C::new(0., -1.),
    C::new(0., 1.),
    C::new(0., 0.),
);

pub static SIGMA_Z: Matrix2x2 = Matrix2x2::new(
    C::new(1., 0.),
    C::new(0., 0.),
    C::new(0., 0.),
    C::new(-1., 0.),
);
