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
}