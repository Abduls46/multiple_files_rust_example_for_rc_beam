
use lib_example_folder::my_example_test::ex_test;

mod calculation;

use calculation::hello::hello_from_bin as hello_bin;

use mylib::reinforced_concrete::moment::say_hello_moment as hi_moment;
use mylib::reinforced_concrete::moment::Moment;
use mylib::reinforced_concrete::moment::MomentCalculation;

// extern crate mylib; // not needed since Rust edition 2018
use mylib::mytest::test;

fn main() {
    ex_test();
    hello_bin();
    test();
    hi_moment();

    let mut m : Moment = Moment{as_: 1000., fy: 420., fc: 30., b: 500., d: 700.};
    println!("concrete block a: {} mm", m.a());
    println!("b: {} mm\n", m.b);

    println!("moment: {:?} kN.m", m.moment());
    println!("moment: {:?} kN.m\n\n", m.coba());

    m.as_ = <Moment as MomentCalculation>::update_dimension_b(&mut m, 1000.);
    println!("concrete block a: {} mm", m.a());
    println!("b: {} mm\n", m.b);

    println!("moment: {:?} kN.m", m.moment());
    println!("moment: {:?} kN.m", m.coba());

}

