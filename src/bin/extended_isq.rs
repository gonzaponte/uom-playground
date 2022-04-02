#[macro_use]
extern crate uom;

use std::f64::consts::PI;

use uom::fmt::DisplayStyle::Abbreviation;

use crate::digit::{adc, kadc};
use crate::count::{pes, kpes};

use uom::si::f64::{Length, Time, Ratio};
use crate::f64::{Digit, Count};

use uom::si::length::millimeter;
use uom::si::time  ::nanosecond;

// #[allow(non_camel_case_types)]
// struct adc(uom::si::f64::Ratio);
// struct pes(uom::si::f64::Ratio);


#[macro_use]
mod digit {
    quantity! {
        quantity: Digit; "digit";
        dimension: CustomSQ<P1, Z0>;
        units {
            @adc : prefix!(none); "adc", "adc", "adcs";
            @kadc: prefix!(kilo); "kadc", "kadc", "kadcs";
        }
    }

    use uom::si::ratio::Ratio;

    impl<A, U, V> From<Ratio<A, V>> for Digit<U, V> where
    A: uom::si::Units<V> + ?Sized,
    U: super::Units<V> + ?Sized,
    V: uom::num_traits::Num + uom::Conversion<V>,
    {
        fn from(val: Ratio<A, V>) -> Self {
            Self {
                dimension: uom::lib::marker::PhantomData,
                units: uom::lib::marker::PhantomData,
                value: val.value
            }
        }
    }

}

#[macro_use]
mod count {
    quantity! {
        quantity: Count; "count";
        dimension: CustomSQ<Z0, P1>;
        units {
            @pes : prefix!(none); "pes", "pes", "pes";
            @kpes: prefix!(kilo); "kpes", "kpes", "kpes";
        }
    }

    use uom::si::ratio::Ratio;

    impl<A, U, V> From<Ratio<A, V>> for Count<U, V> where
    A: uom::si::Units<V> + ?Sized,
    U: super::Units<V> + ?Sized,
    V: uom::num_traits::Num + uom::Conversion<V>,
    {
        fn from(val: Ratio<A, V>) -> Count<U, V> {
            Self {
                dimension: uom::lib::marker::PhantomData,
                units: uom::lib::marker::PhantomData,
                value: val.value
            }
        }
    }
}


system! {
    quantities: CustomSQ {
        digit: adc, ADC;
        count: pes, PES;
    }

    units: CustomSU {
        mod digit::Digit,
        mod count::Count,
    }
}

mod f32 {
    mod detector {
        pub use super::super::*;
    }

    CustomSQ!(self::detector, f32);
}


mod f64 {
    mod detector {
        pub use super::super::*;
    }

    CustomSQ!(self::detector, f64);
}


fn main() {
    let some_length  = Length::new::<millimeter>(9.8_f64);
    let other_length = Length::new::<millimeter>(5.9_f64);
    let some_time    = Time  ::new::<nanosecond>(1.1_f64);
    let other_time   = Time  ::new::<nanosecond>(5.5_f64);

    let some_length_ratio : Ratio = some_length / other_length;
    let some_time_ratio   : Ratio = some_time   / other_time;

    let some_adc = Digit::new::<kadc>(42_f64);
    let some_pes = Count::new::<kpes>(PI);

    let adc_from_ratio : Digit = Digit::from(some_time_ratio);
    let pes_from_ratio : Count = Count::from(some_length_ratio);


    // These don't
    #[cfg(feature = "compile-error")]
    {
        let _  = some_lengh + some_time;
        let _  = some_lengh + some_adc;
        let _  = some_lengh + some_pes;

        let _  = some_adc + some_pes;
    }

    // This works
    let _ = some_length / some_time;

    // This doesn't work out of the box
    #[cfg(feature = "compile-error")]
    let calibration  = some_pes / some_adc;


    println!("I have {} and {}",
        some_adc.into_format_args(adc, Abbreviation),
        some_pes.into_format_args(pes, Abbreviation)
    );

    println!("From ratios I got {} and {}",
        adc_from_ratio.into_format_args(adc, Abbreviation),
        pes_from_ratio.into_format_args(pes, Abbreviation)
    );

}
