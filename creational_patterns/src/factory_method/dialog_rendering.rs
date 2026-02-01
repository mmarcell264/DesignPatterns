mod gui;
mod html_gui;
mod windows_gui;
mod init;
mod main;

use crate::factory_method::dialog_rendering::gui::{Button, Dialog};
use crate::factory_method::dialog_rendering::html_gui::HtmlDialog;
use crate::factory_method::dialog_rendering::windows_gui::WindowsDialog;
use crate::factory_method::dialog_rendering::init::initialize;
pub use crate::factory_method::dialog_rendering::main::dialog_main;