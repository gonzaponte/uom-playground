#[macro_use]
extern crate uom;

use std::f64::consts::PI;
use uom::fmt::DisplayStyle::Abbreviation;

use crate::digitized_signal::{adc, kadc};
use crate::calibrated_signal::{pes, kpes};


// Define a new quantity
#[macro_use]
mod digitized_signal {
    quantity! {
        quantity: DigitizedSignal; "digitized_signal";
        dimension: Q<P1, Z0>;
        units {
            @adc: prefix!(none); "adc", "adc", "adcs";
            @kadc: prefix!(kilo); "kadc", "kadc", "kadcs";
        }
    }
}

// Define a new quantity
#[macro_use]
mod calibrated_signal {
    quantity! {
        quantity: CalibratedSignal; "calibrated_signal";
        dimension: Q<Z0, P1>;
        units {
            @pes: prefix!(none); "pes", "pes", "pes";
            @kpes: prefix!(kilo); "kpes", "kpes", "kpes";
        }
    }
}

system! {
    quantities: Q {
        digitized_signal: adc, Adc;
        calibrated_signal: pes, Pes;
    }

    units: U {
        mod digitized_signal::DigitizedSignal,
        mod calibrated_signal::CalibratedSignal,
    }
}

mod f32 {
    mod extended_isq {
        pub use super::super::*;
    }

    Q!(self::extended_isq, f32);
}

mod f64 {
    mod extended_isq {
        pub use super::super::*;
    }

    Q!(self::extended_isq, f64);
}


fn main() {
    let some_adc = f32:: DigitizedSignal::new::<kadc>(42_f32);
    let some_pes = f64::CalibratedSignal::new::<kpes>(PI);

    #[cfg(feature = "compile-error")]
    let calibration  = some_pes / some_adc; // Doesn't work out of the box

    println!("I have {}, which corresponds to {}",
        some_adc.into_format_args(adc, Abbreviation),
        some_pes.into_format_args(pes, Abbreviation)
    );
}
