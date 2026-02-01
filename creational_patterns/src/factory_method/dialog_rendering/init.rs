use super::Dialog;
use super::HtmlDialog;
use super::WindowsDialog;

pub fn initialize() -> &'static dyn Dialog {
    // The dialog type is selected depending on the environment settings or configuration
    if cfg!(windows) {
        println!("-- Windows detected, creating Windows GUI --");
        &WindowsDialog
    } else {
        println!("-- No OS detected, creating the HTML GUI --");
        &HtmlDialog
    }
}