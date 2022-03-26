#[macro_use]
extern crate uom;

use std::f64::consts::PI;

use uom::fmt::DisplayStyle::Abbreviation;

pub use uom::si::{self, *};

// use uom::si::{ length
//              , mass
//              , time
//              , electric_current
//              , thermodynamic_temperature
//              , amount_of_substance
//              , luminous_intensity
//              };

// use uom::si::length::meter;
// use uom::si::mass::kilogram;
// use uom::si::time::second;
// use uom::si::electric_current::ampere;
// use uom::si::thermodynamic_temperature::kelvin;
// use uom::si::amount_of_substance::mole;
// use uom::si::luminous_intensity::candela;

use crate::digitized_signal::{adc, kadc};
use crate::calibrated_signal::{pes, kpes};


// Define a new quantity
#[macro_use]
pub mod digitized_signal {
    quantity! {
        quantity: DigitizedSignal; "digitized_signal";
        dimension: EISQ<Z0, Z0, Z0, Z0, Z0, Z0, Z0, P1, Z0>;
        units {
            @adc: prefix!(none); "adc", "adc", "adcs";
            @kadc: prefix!(kilo); "kadc", "kadc", "kadcs";
        }
    }
}

// Define a new quantity
#[macro_use]
pub mod calibrated_signal {
    quantity! {
        quantity: CalibratedSignal; "calibrated_signal";
        dimension: EISQ<Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, P1>;
        units {
            @pes: prefix!(none); "pes", "pes", "pes";
            @kpes: prefix!(kilo); "kpes", "kpes", "kpes";
        }
    }
}

system! {
    quantities: EISQ {
        length: meter, L;
        mass: kilogram, M;
        time: second, T;
        electric_current: ampere, I;
        thermodynamic_temperature: kelvin, Th;
        amount_of_substance: mole, N;
        luminous_intensity: candela, J;
        digitized_signal: adc, Adc;
        calibrated_signal: pes, Pes;
    }

    units: ESIU {
        // mod absement::Absement, // for some reason, it can't find this particular module
        mod acceleration::Acceleration,
        mod amount_of_substance::AmountOfSubstance,
        mod angle::Angle,
        mod angular_acceleration::AngularAcceleration,
        mod angular_jerk::AngularJerk,
        mod angular_velocity::AngularVelocity,
        mod area::Area,
        mod available_energy::AvailableEnergy,
        mod capacitance::Capacitance,
        mod catalytic_activity::CatalyticActivity,
        mod catalytic_activity_concentration::CatalyticActivityConcentration,
        mod curvature::Curvature,
        mod electric_charge::ElectricCharge,
        mod electric_current::ElectricCurrent,
        mod electric_potential::ElectricPotential,
        mod electrical_conductance::ElectricalConductance,
        mod electrical_resistance::ElectricalResistance,
        mod energy::Energy,
        mod force::Force,
        mod frequency::Frequency,
        mod heat_capacity::HeatCapacity,
        mod heat_flux_density::HeatFluxDensity,
        mod heat_transfer::HeatTransfer,
        mod inductance::Inductance,
        mod information::Information,
        mod information_rate::InformationRate,
        mod jerk::Jerk,
        mod length::Length,
        mod luminance::Luminance,
        mod luminous_intensity::LuminousIntensity,
        mod magnetic_flux::MagneticFlux,
        mod magnetic_flux_density::MagneticFluxDensity,
        mod mass::Mass,
        mod mass_concentration::MassConcentration,
        mod mass_density::MassDensity,
        mod mass_rate::MassRate,
        mod molar_concentration::MolarConcentration,
        mod molar_energy::MolarEnergy,
        mod molar_heat_capacity::MolarHeatCapacity,
        mod molar_mass::MolarMass,
        mod momentum::Momentum,
        mod power::Power,
        mod pressure::Pressure,
        mod ratio::Ratio,
        mod specific_heat_capacity::SpecificHeatCapacity,
        mod solid_angle::SolidAngle,
        mod radiant_exposure::RadiantExposure,
        mod temperature_interval::TemperatureInterval,
        mod thermal_conductivity::ThermalConductivity,
        mod thermodynamic_temperature::ThermodynamicTemperature,
        mod time::Time,
        mod torque::Torque,
        mod velocity::Velocity,
        mod volume::Volume,
        mod volume_rate::VolumeRate,

        mod digitized_signal::DigitizedSignal,
        mod calibrated_signal::CalibratedSignal,
    }
}

mod f32 {
    use crate::ESIU;

    mod extended_isq {
        pub use super::*;
        pub use super::super::si::*;
        pub use super::super:: digitized_signal;
        pub use super::super::calibrated_signal;
    }

    EISQ!(self::extended_isq, f32);
}

mod f64 {
    use crate::ESIU;

    mod extended_isq {
        pub use super::*;
        pub use super::super::si::*;
        pub use super::super:: digitized_signal;
        pub use super::super::calibrated_signal;
    }

    EISQ!(self::extended_isq, f64);
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
