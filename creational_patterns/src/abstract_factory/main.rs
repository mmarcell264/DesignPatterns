use crate::abstract_factory::gui::GuiFactoryDynamic;

use super::render::{s_render, dyn_render};
use super::macos_gui::factory::MacFactory;
use super::windows_gui::factory::WindowsFactory;

pub fn static_gui_elements_main() {
    let windows = true;

    if windows {
        s_render(WindowsFactory)
    } else {
        s_render(MacFactory)
    }
}

pub fn dynamic_gui_elements_main() {
    let windows = false;

    // Allocate a factory object in runtime depending on unpredictable input.
    let factory: &dyn GuiFactoryDynamic = if windows {
        &WindowsFactory
    } else {
        &MacFactory
    };

    // Factory invocation can be inlined right here.
    let button = factory.create_button();
    button.press();

    dyn_render(factory);
}
