use super::{into_next, Department, Patient};

pub struct Medical<'a> {
    next: Option<Box<dyn Department<'a> + 'a >>
}

impl<'a> Medical<'a> {
    pub fn new<T: Department<'a>  + Sized + 'a>(next: T) -> Self {
        Self { next: into_next(next) }
    }
}

impl<'a> Department<'a> for Medical<'a> {
    fn handle(&mut self, patient: &mut Patient) {
        if patient.medicine_done {
            println!("Medicine is already given to a patient");
        } else {
            println!("Medical giving medicine to a patient {}", patient.name);
            patient.medicine_done = true
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn  Department<'a> + 'a>> {
        &mut self.next
    }


}