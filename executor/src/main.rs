use creational_patterns::factory_method::dialog_rendering::dialog_main;
use creational_patterns::factory_method::maze_game::maze_main;
use creational_patterns::abstract_factory::main::{static_gui_elements_main, dynamic_gui_elements_main};
use creational_patterns::builder::main::builder_main;
use creational_patterns::prototpye::prototype_main;
use creational_patterns::singleton::{main_local_s, main_mutex_s};
use structural_patterns::adapter::main::adapter_main;
use structural_patterns::bridge::main::bridge_main;
use std::io::{self, Write};

fn main() {
    let mut buffer = String::new();
    print!("Name the pattern example: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    match buffer.as_str().trim() {
        "f_dialog_rendering" => dialog_main(),
        "f_maze_game" => maze_main(),
        "a_f" => {static_gui_elements_main(); dynamic_gui_elements_main();},
        "builder" => builder_main(),
        "prototype" => prototype_main(),
        "singleton" => {main_local_s(); main_mutex_s();},
        "adapter" => adapter_main(),
        "bridge" => bridge_main(),
        _ => println!("Hello, world!")
    }
}
