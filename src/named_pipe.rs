use std::fs;
use std::fs::create_dir_all;
use std::process::Child;
use crate::devices::Device;

pub fn add(device: &Device) -> std::io::Result<Child> {
    let mut dir = String::from("/tmp/VEPM/dev/");
    create_dir_all(&dir).unwrap();
    dir.push_str(device.name());
    let mut command = std::process::Command::new("mknod");
    command.args([dir.as_str(),"p"]).spawn()
}
pub fn remove(device: &Device) -> std::io::Result<()> {
    let mut dir = String::from("/tmp/VEPM/dev/");
    dir.push_str(device.name());
    fs::remove_file(dir)
}