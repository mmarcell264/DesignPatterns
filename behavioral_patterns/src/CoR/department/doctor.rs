use super::{into_next, Department, Patient};

pub struct Doctor<'a> {
    next: Option<Box<dyn Department<'a> + 'a >>
}

impl<'a> Doctor<'a> {
    pub fn new<T: Department<'a>  + Sized + 'a>(next: T) -> Self {
        Self { next: into_next(next) }
    }
}

impl<'a> Department<'a> for Doctor<'a> {
    fn handle(&mut self, patient: &mut Patient) {
        if patient.doctor_check_up_done {
            println!("A doctor checkup is already done");
        } else {
            println!("Doctor checking a patient {}", patient.name);
            patient.doctor_check_up_done = true
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn  Department<'a> + 'a>> {
        &mut self.next
    }


}