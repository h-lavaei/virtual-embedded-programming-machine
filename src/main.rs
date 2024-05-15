mod seven_segment;
mod devices;

use std::io::stdin;
use crate::devices::Device;
use crate::seven_segment::SevenSegment;

fn main() {
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
        "add" => {
            let device_type = arguments.next();
            if device_type.is_some(){
                match device_type.unwrap() {
                    "seven_segment" => {
                        let name = arguments.next();
                        if name.is_some() {
                            devices.push(Device::SevenSegment(SevenSegment::new(name.unwrap())));
                        }else {
                            println!("Wrong arguments! No name found! Enter help for more info.");
                        }
                    }
                    _ => {println!("Device type not found")}
                }
            }else {
                println!("Wrong arguments! No device type found! Enter help for more info.");
            }
        },
        "edit" => {
            let name = arguments.next();
            if name.is_some() {
                let device = devices.iter_mut().find(|device| device.name() == name.unwrap());
                if device.is_some() {
                    let device: &mut Device = device.unwrap();
                    let change = arguments.next();
                    if change.is_some(){
                        let change = change.unwrap();
                        if device.edit(change).is_err(){
                            println!("Wrong argument {}. Enter help for more info.",change);
                        }
                    }else {
                        println!("Wrong argument! no changing found! Enter help for more info.");
                    }
                } else {
                    println!("Device not found!");
                }
            }else {
                println!("Device not found!");
            }
        },
        "remove" => {
            let name = arguments.next();
            if name.is_some(){
                let device = devices.iter().find(|device| device.name() == name.unwrap()).unwrap();
                let device_pos = devices.iter().position(|x| x.name() == device.name());
                devices.remove(device_pos.unwrap());
            }
        },
        "list" => {devices.iter().for_each(move |device| device.print())},
        "help" => {println!("add a new device : add device_type name\nedit a device : edit name change\nlist devices : list\nhelp : help")},
        "exit" => {std::process::exit(0)}
        _ => {println!("Command not found! See help for valid commands")},
    }
}