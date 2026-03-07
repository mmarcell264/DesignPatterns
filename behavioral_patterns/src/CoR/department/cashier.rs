use super::{Department, Patient};

#[derive(Default)]
pub struct Cashier<'a> {
    next: Option<Box<dyn Department<'a> + 'a>>
}

impl<'a> Department<'a> for Cashier<'a> {
    fn handle(&mut self, patient: &mut Patient) {
        if patient.payment_done {
            println!("Payment done");
        } else {
            println!("Cashier getting money from a patient {}", patient.name);
            patient.payment_done = true
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn  Department<'a> + 'a>> {
        &mut self.next
    }


}