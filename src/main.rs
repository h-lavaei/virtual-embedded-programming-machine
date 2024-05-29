mod seven_segment;
mod devices;
mod named_pipe;

use std::fs;
use std::io::{stdin, Write};
use crate::devices::Device;

fn main() {
    let remove_tmp = fs::remove_dir_all("/tmp/VEPM/");
    if remove_tmp.is_ok(){
        remove_tmp.unwrap();
    }
    let mut devices:Vec<Device> = Vec::new();
    loop {
        println!("Enter a command ..");
        match_input(&mut devices);
    }
}
fn get_input() -> String{
    let mut input = String::new();
    stdin().read_line(&mut input).expect("can't read user input!!");
    input
}
fn match_input(devices:&mut Vec<Device>){
    let input = get_input();
    let mut arguments = input.split_ascii_whitespace();
    let action = arguments.next().unwrap_or("list");
    match action.trim() {
        "add" => devices::add(&mut arguments,devices),
        "edit" => devices::edit(&mut arguments,devices),
        "remove" => devices::remove(&mut arguments,devices),
        "list" => {devices.iter().for_each(move |device| device.print())},
        "help" => {println!("add a new device : add device_type name\nedit a device : edit name change\nlist devices : list\nhelp : help")},
        "exit" => {std::process::exit(0)}
        _ => {println!("Command not found! See help for valid commands")},
    }
}