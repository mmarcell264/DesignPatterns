use super::Train;
use crate::mediator::train_station::Mediator;

pub struct PassangerTrain {
    name: String,
}

impl PassangerTrain {
    pub fn new(name: &'static str) -> Self {
        Self { name: name.into() }
    }
}


impl Train for PassangerTrain {
    fn name(&self) -> &String {
        &self.name
    }

    fn arrive(&mut self, mediator: &mut dyn Mediator) {
        if !mediator.notify_about_arrival(&self.name) {
            println!("Passanger train {}: Arrival blocked, waiting", self.name);
            return;
        }

        println!("Passanger train {}: Arrived", self.name);
    }

    fn depart(&mut self, mediator: &mut dyn Mediator) {
        println!("Passanger train {}: Leaving", self.name);
        mediator.notify_about_departure(&self.name);
    }
}