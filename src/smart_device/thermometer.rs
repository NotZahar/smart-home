use num_traits::FromPrimitive;

use crate::utils::random::{RandomGenerator, SimpleRandomGenerator};
use crate::utils::trait_alias::{Number, RandomNumber};

pub trait Thermometer<TemperatureT: Number> {
    const DEFAULT_TEMPERATURE: TemperatureT;

    fn new() -> Self;

    #[must_use]
    fn get_temperature(&mut self) -> TemperatureT;
}

pub struct CelsiusThermometer<TemperatureT: RandomNumber> {
    current_temperature: TemperatureT,
    random_offset_generator: SimpleRandomGenerator<TemperatureT>,
}

impl<TemperatureT: RandomNumber + FromPrimitive> Thermometer<TemperatureT>
    for CelsiusThermometer<TemperatureT>
{
    const DEFAULT_TEMPERATURE: TemperatureT = TemperatureT::ZERO;

    fn new() -> Self {
        CelsiusThermometer {
            current_temperature: Self::DEFAULT_TEMPERATURE,
            random_offset_generator: SimpleRandomGenerator::new(),
        }
    }

    fn get_temperature(&mut self) -> TemperatureT {
        let min_offset = TemperatureT::from_f64(-10.0).unwrap();
        let max_offset = TemperatureT::from_f64(10.0).unwrap();

        self.current_temperature
            + self
                .random_offset_generator
                .generate(min_offset, max_offset)
    }
}
