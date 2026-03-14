use creational_patterns::factory_method::dialog_rendering::dialog_main;
use creational_patterns::factory_method::maze_game::maze_main;
use creational_patterns::abstract_factory::main::{static_gui_elements_main, dynamic_gui_elements_main};
use creational_patterns::builder::main::builder_main;
use creational_patterns::prototpye::prototype_main;
use creational_patterns::singleton::{main_local_s, main_mutex_s};
use structural_patterns::adapter::main::adapter_main;
use structural_patterns::bridge::main::bridge_main;
use structural_patterns::composite::main::composite_main;
use structural_patterns::decorator::main::decorator_main;
use structural_patterns::facade::main::facade_main;
use structural_patterns::flyweight::main::flyweight_main;
use structural_patterns::proxy::main::proxy_main;
use behavioral_patterns::CoR::main::CoR_main;
use behavioral_patterns::command::main::command_main;
use behavioral_patterns::iterator::standard_iterator::st_iterator_main;
use behavioral_patterns::iterator::custom_iterator::ct_iterator_main;
use behavioral_patterns::mediator::main::mediator_main;
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
        "composite" => composite_main(),
        "decorator" => decorator_main(),
        "facade" => facade_main().expect("Error in facade"),
        "flyweight" => flyweight_main(),
        "proxy" => proxy_main(),
        "CoR" => CoR_main(),
        "command" => command_main(),
        "iterator" => {st_iterator_main(); ct_iterator_main();},
        "mediator" => mediator_main(),
        _ => println!("Hello, world!")
    }
}
