use super::{into_next, Department, Patient};

#[derive(Default)]
pub struct Reception<'a> {
    next: Option<Box<dyn Department<'a> + 'a >>
}

impl<'a> Reception<'a> {
    pub fn new<T: Department<'a>  + Sized + 'a>(next: T) -> Self {
        Self { next: into_next(next) }
    }
}

impl<'a> Department<'a> for Reception<'a> {
    fn handle(&mut self, patient: &mut Patient) {
        if patient.registration_done {
            println!("Patient registration is already done");
        } else {
            println!("Reception registering a patient {}", patient.name);
            patient.registration_done = true
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn  Department<'a> + 'a>> {
        &mut self.next
    }


}