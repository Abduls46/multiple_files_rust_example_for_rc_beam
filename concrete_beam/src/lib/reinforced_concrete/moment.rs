
use crate::sub_lib::sublib::hello_sublib as hi_sublib;
// use crate::src::mybin::calculation::hello::hello_from_bin; // cannot call bin folder


pub fn say_hello_moment(){
    print!("hi from fn say_hello_moment -> ");
    hi_sublib();
	println!("halo from moment module");
}

#[derive(Debug)]
pub struct Moment {
    pub as_: f64,
    pub fy: f64,
    pub fc: f64,
    pub b: f64,
    pub d: f64,
}

impl Moment {
    pub fn moment(&self) -> f64 {
        let m = self.as_ * self.fy * (self.d - self.a()/ 2.) / 1000000.;	//kN.m
        m
    }
}

pub trait MomentCalculation {
    fn update_dimension_b(&mut self, b_: f64) -> f64;
    fn a(&self) -> f64;
    fn coba(&self) -> f64;
}

impl MomentCalculation for Moment {
    fn update_dimension_b(&mut self, b_: f64) -> f64 {
        self.b = b_;
        self.b
    }

    fn a(&self) -> f64 {
        let a = self.as_ * self.fy / (0.85 * self.fc * self.b);
        a
    }
    fn coba(&self) -> f64 {
        self.moment()
    }
}

