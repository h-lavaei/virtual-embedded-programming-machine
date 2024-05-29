use std::str::SplitAsciiWhitespace;
use crate::named_pipe;
use crate::seven_segment::SevenSegment;

#[derive(Debug)]
pub enum Device {
    SevenSegment(SevenSegment),
}
impl Device {
    pub fn print(&self){
        match self {
            Device::SevenSegment(seven_segment) => { seven_segment.print() },
        }
    }
    pub fn name(&self) -> &str{
        match self {
            Device::SevenSegment(seven_segment) => {seven_segment.name.as_str()}
        }
    }
    pub fn edit(&mut self,change:&str) -> Result<(),&str>{
        match self {
            Device::SevenSegment(seven_segment) => {
                let mut chars = change.chars();
                if change.chars().count() == 8 {
                    seven_segment.leds = [chars.nth(0).unwrap() == '1',chars.nth(0).unwrap() == '1',chars.nth(0).unwrap() == '1',chars.nth(0).unwrap() == '1',chars.nth(0).unwrap() == '1',chars.nth(0).unwrap() == '1',chars.nth(0).unwrap() == '1',chars.nth(0).unwrap() == '1',];
                    Ok(())
                } else if change.chars().count() == 1 {
                    seven_segment.make_char(change.chars().nth(0).unwrap());
                    Ok(())
                } else {
                    Err("")
                }
            }
        }
    }
    pub fn get_data(&self) -> Box<[u8]> {
        match self {
            Device::SevenSegment(seven_segment) => {seven_segment.get_data()}
            _ => {Box::new([0])}
        }
    }
}


pub fn add(arguments:&mut SplitAsciiWhitespace,devices:&mut Vec<Device>){
    let device_type = arguments.next();
    if device_type.is_some(){
        let device_type = device_type.unwrap();
        let name = arguments.next();
        if name.is_some(){
            match device_type {
                "seven_segment" => {
                    let name = name.unwrap();
                    let seven_segment = Device::SevenSegment(SevenSegment::new(name));
                    let named_pipe = named_pipe::add(&seven_segment);
                    devices.push(seven_segment);
                    if named_pipe.is_err(){
                        println!("can't create a named pipe");
                    }else {
                        named_pipe.unwrap();
                    }
                }
                _ => {println!("Device type not found")}
            }
        }else {
            println!("Wrong arguments! No name found! Enter help for more info.");
        }
    }else {
        println!("Wrong arguments! No device type found! Enter help for more info.");
    }
}
pub fn edit(arguments:&mut SplitAsciiWhitespace,devices:&mut Vec<Device>){
    let name = arguments.next();
    if name.is_some() {
        let name = name.unwrap();
        let device = devices.iter_mut().find(|device| device.name() == name);
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

}
pub fn remove(arguments:&mut SplitAsciiWhitespace,devices:&mut Vec<Device>){
    let name = arguments.next();
    if name.is_some(){
        let device = devices.iter().find(|device| device.name() == name.unwrap()).unwrap();
        let device_pos = devices.iter().position(|x| x.name() == device.name()).unwrap();
        let named_pipe = named_pipe::remove(devices.iter().nth(device_pos).unwrap());
        if named_pipe.is_ok(){
            named_pipe.unwrap();
        }
        devices.remove(device_pos);
    }
}