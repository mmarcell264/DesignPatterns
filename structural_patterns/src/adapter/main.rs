use crate::adapter;

use super::adaptee::SpecificTarget;
use super::adapter::TargetAdepter;
use super::target::{OrdinaryTarget, Target};

/// Calls any object of a `Target` trait.
///
/// To understand the Adapter pattern better, imagine that this is
/// a client code, which can operate over a specific interface only
/// (`Target` trait only). It means that an incompatible interface cannot be
/// passed here without an adapter.
fn call(target: impl Target) {
    println!("'{}'", target.request())
}

pub fn adapter_main() {
    let target = OrdinaryTarget;

    print!("A compatible target can be directly called: ");
    call(target);

    let adaptee = SpecificTarget;

    println!(
        "Adaptee is incompatible with client: '{}'",
        adaptee.specific_request()
    );

    let adapter = TargetAdepter::new(adaptee);

    print!("But with adapter cleint can call its method: ");
    call(adapter);
}