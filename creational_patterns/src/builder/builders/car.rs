use super::{Builder, CarType, Engine, GpsNavigator, Transmissions};
use crate::builder::cars::Car;

pub const DEFAULT_FUEL: f64 = 5f64;

#[derive(Default)]
pub struct CarBuilder {
    car_type: Option<CarType>,
    engine: Option<Engine>,
    gps_navigator: Option<GpsNavigator>,
    seats: Option<u16>,
    transmission: Option<Transmissions>,
}

impl Builder for CarBuilder {
    type OutputType = Car;

    fn set_car_type(&mut self, car_type: CarType) {
        self.car_type = Some(car_type);
    }

    fn set_engine(&mut self, engine: Engine) {
        self.engine = Some(engine)
    }

    fn set_gps_navigator(&mut self, gps_navigator: GpsNavigator) {
        self.gps_navigator = Some(gps_navigator)
    }

    fn set_seats(&mut self, seats: u16) {
        self.seats = Some(seats);
    }

    fn set_transmission(&mut self, transmission: Transmissions) {
        self.transmission = Some(transmission);
    }

    fn build(self) -> Self::OutputType {
        Car::new(
            self.car_type.expect("Please, set a car type"),
            self.seats.expect("Please, set a number of seats"),
            self.engine.expect("Plese, set an engine configuration"),
            self.transmission.expect("Please, set up transmission"),
            self.gps_navigator,
            DEFAULT_FUEL
        )
    }
}