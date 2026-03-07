mod cashier;
mod doctor;
mod medical;
mod reception;

use super::patient::Patient;
pub use self::{cashier::Cashier, doctor::Doctor, medical::Medical, reception::Reception};

/// A single role of objects that make up a chain.
/// A typical trait implementation must have `handle` and `next` methods,
/// while `execute` is implemented by default and contains a proper chaining
/// logic.
pub trait Department<'a> {
    fn execute(&mut self, patient: &mut Patient) {
        self.handle(patient);

        if let Some(next) = &mut self.next() {
            next.execute(patient);
        }
    }

    fn handle(&mut self, patient: &mut Patient);
    fn next(&mut self) -> &mut Option<Box<dyn  Department<'a> + 'a>>;
}

/// Helps to wrap an object into a boxed type.
pub fn into_next<'a, T: Department<'a>  + Sized + 'a>(department: T) -> Option<Box<dyn Department<'a> + 'a >> {
    Some(Box::new(department))
}