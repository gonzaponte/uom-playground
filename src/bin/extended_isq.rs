#[macro_use]
extern crate uom;

use std::f64::consts::PI;

use uom::fmt::DisplayStyle::Abbreviation;

use crate::digit::{adc, kadc};
use crate::count::{pes, kpes};
use crate::sensor_calibration::pes_per_adc;

use uom::si::f64::{Length, Time, Ratio};
use crate::f64::{Digit, Count};

use uom::si::length::millimeter;
use uom::si::time  ::nanosecond;


//  Digits are meant to be discrete, but the underlying type must be the same
// for all units...
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

    // TODO: create  macro to avoid code duplication
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

#[macro_use]
mod sensor_calibration {
    quantity! {
        quantity: SensorCalibration; "sensor_calibration";
        dimension: CustomSQ<N1, P1>;
        units {
            @pes_per_adc : prefix!(none); "pes/adc", "pes/adc", "pes/adcs";
            @kpes_per_adc: prefix!(kilo); "kpes/adc", "kpes/adc", "kpes/adcs";
        }
    }

    use uom::si::ratio::Ratio;

    impl<A, U, V> From<Ratio<A, V>> for SensorCalibration<U, V> where
    A: uom::si::Units<V> + ?Sized,
    U: super::Units<V> + ?Sized,
    V: uom::num_traits::Num + uom::Conversion<V>,
    {
        fn from(val: Ratio<A, V>) -> SensorCalibration<U, V> {
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
        mod sensor_calibration::SensorCalibration,
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

        some_adc *= some_pes;
    }

    // no physical interpretation, but still allowed
    let _  = some_adc * some_pes;

    // has physical meaning
    let  sensor_calibration  = some_pes / some_adc;
    let isensor_calibration  = sensor_calibration.recip();


    println!("I have {} and {}. The calibration factor is {} or {}",
         some_adc          .into_format_args(adc        , Abbreviation),
         some_pes          .into_format_args(pes        , Abbreviation),
         sensor_calibration.into_format_args(pes_per_adc, Abbreviation),
         "### adc/pes"
         // isensor_calibration.into_format_args(pes_per_adc, Abbreviation), // No units defined!
    );

    println!("From ratios I got {} and {}",
        adc_from_ratio.into_format_args(adc, Abbreviation),
        pes_from_ratio.into_format_args(pes, Abbreviation)
    );

}
