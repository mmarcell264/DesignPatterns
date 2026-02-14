use super::device::{Device, Radio, Tv};
use super::remotes::{AdvancedRemote, BasicRemote, HasMutableDevice, Remote};

pub fn bridge_main() {
    test_device(Tv::default());
    test_device(Radio::default());
}

fn test_device(device: impl Device + Clone) {
    println!("Test with basic remote.");
    let mut basic_remote = BasicRemote::new(device.clone());
    basic_remote.power();
    basic_remote.device().print_status();

    println!("Test with advanced remote.");
    let mut advanced_remote = AdvancedRemote::new(device);
    advanced_remote.power();
    advanced_remote.mute();
    advanced_remote.device().print_status();
}