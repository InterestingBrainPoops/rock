use nalgebra::{Matrix6, Matrix6x3};
#[allow(unused_imports)]
use num_traits::real::Real;

const CONTROLLER_FREQ: f64 = 200.0;
const DT: f64 = 1.0 / CONTROLLER_FREQ;
#[rustfmt::skip]
const STATE_TRANSITION: Matrix6<f64> = Matrix6::<f64>::new(
    1.0, 0.0, 0.0, DT,  0.0, 0.0,
    0.0, 1.0, 0.0, 0.0, DT,  0.0,
    0.0, 0.0, 1.0, 0.0, 0.0, DT,
    0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
);

const A_TO_VDT: f64 = 0.5 * DT * DT;

#[rustfmt::skip]
const CONTROL_MATRIX: Matrix6x3<f64> = Matrix6x3::new(
    A_TO_VDT, 0.0,      0.0,
    0.0,      A_TO_VDT, 0.0,
    0.0,      0.0,      A_TO_VDT,
    DT,       0.0,      0.0,
    0.0,      DT,       0.0,
    0.0,      0.0,      DT,
);
pub struct KinematicFilter {}
