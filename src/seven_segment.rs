use colored::Colorize;

#[derive(Debug)]
pub struct SevenSegment {
    pub name:String,
    pub leds:[bool;8],
}
impl SevenSegment {
    pub fn new(name:&str) -> SevenSegment{
        SevenSegment{
            name: name.to_string(),
            leds: [false,false,false,false,false,false,false,false],
        }
    }
    pub fn make_char(&mut self,char:char){
        let led7 = self.leds[7];
        match char {
            // work to do
            'a' => {self.leds = [true,true,true,true,true,false,true,false]},
            'b' => {self.leds = [false,false,true,true,true,true,true,false]},
            'c' => {self.leds = [false,false,false,true,true,false,true,false]},
            'd' => {self.leds = [false,true,true,true,true,false,true,false]},
            'e' => {self.leds = [true,true,false,true,true,true,true,false]},
            'g' => {self.leds = [true,true,true,true,false,true,true,false]},
            'h' => {self.leds = [false,false,true,false,true,true,true,false]},
            'i' => {self.leds = [false,false,true,false,false,false,false,led7]},
            'j' => {self.leds = [false,true,true,true,false,false,false,false]},
            'l' => {self.leds = [false,false,false,false,true,true,false,false]},
            'n' => {self.leds = [false,false,true,false,true,false,true,false]},
            'o' => {self.leds = [false,false,true,true,true,false,true,false]},
            'q' => {self.leds = [true,true,true,false,false,true,true,false]},
            'r' => {self.leds = [false,false,false,false,true,false,true,false]},
            't' => {self.leds = [false,false,false,true,true,true,true,false]},
            'u' => {self.leds = [false,false,true,true,true,false,false,false]},
            'y' => {self.leds = [false,true,true,true,false,true,true,false]},
            'A' => {self.leds = [true,true,true,false,true,true,true,false]},
            'C' => {self.leds = [true,false,false,true,true,true,false,false]},
            'E' => {self.leds = [true,false,false,true,true,true,true,false]},
            'F' => {self.leds = [true,false,false,false,true,true,true,false]},
            'G' => {self.leds = [true,false,true,true,true,true,false,false]},
            'H' => {self.leds = [false,true,true,false,true,true,true,false]},
            'I' => {self.leds = [false,true,true,false,false,false,false,false]},
            'J' => {self.leds = [false,true,true,true,true,false,false,false]},
            'L' => {self.leds = [false,false,false,true,true,true,false,false]},
            'O' => {self.leds = [true,true,true,true,true,true,false,false]},
            'P' => {self.leds = [true,true,false,false,true,true,true,false]},
            'S' => {self.leds = [true,false,true,true,false,true,true,false]},
            'T' => {self.leds = [true,false,false,false,true,true,false,false]},
            'U' => {self.leds = [false,true,true,true,true,true,false,false]},
            'Z' => {self.leds = [true,true,false,true,true,false,true,false]},
            ' ' => {self.leds = [false,false,false,false,false,false,false,false]},
            '_' => {self.leds = [false,false,false,true,false,false,false,false]},
            '-' => {self.leds = [false,false,false,false,false,false,true,false]},
            '‾' => {self.leds = [true,false,false,false,false,false,false,false]},
            '=' => {self.leds = [false,false,false,true,false,false,true,false]},
            '≡' => {self.leds = [true,false,false,true,false,false,true,false]},
            '°' => {self.leds = [true,true,false,false,false,true,true,false]},
            '"' => {self.leds = [false,true,false,false,false,true,false,false]},
            '\'' => {self.leds = [false,false,false,false,false,true,false,false]},
            '(' => {self.leds = [true,false,false,true,true,true,false,false]},
            '[' => {self.leds = [true,false,false,true,true,true,false,false]},
            ')' => {self.leds = [true,true,true,true,false,false,false,false]},
            ']' => {self.leds = [true,true,true,true,false,false,false,false]},
            '<' => {self.leds = [true,false,false,false,false,true,true,false]},
            '>' => {self.leds = [true,true,false,false,false,false,true,false]},
            '?' => {self.leds = [true,true,false,false,true,false,true,false]},
            '0' => {self.leds = [true,true,true,true,true,true,false,led7]},
            '1' => {self.leds = [false,true,true,false,false,false,false,led7]},
            '2' => {self.leds = [true,true,false,true,true,false,true,led7]},
            '3' => {self.leds = [true,true,true,true,false,false,true,led7]},
            '4' => {self.leds = [false,true,true,false,false,true,true,led7]},
            '5' => {self.leds = [true,false,true,true,false,true,true,led7]},
            '6' => {self.leds = [true,false,true,true,true,true,true,led7]},
            '7' => {self.leds = [true,true,true,false,false,false,false,led7]},
            '8' => {self.leds = [true,true,true,true,true,true,true,led7]},
            '9' => {self.leds = [true,true,true,true,false,true,true,led7]},
            _ => {self.leds = [false,false,false,false,false,false,false,false]; println!("Unknown letter for a seven segment. turning all LED's off")},
        }
    }
    pub fn print(&self) {
        println!("Seven Segment {} :\n{}\n{}   {}\n{}\n{}   {}\n{} {}\n",
               self.name,
               if self.leds[0] { "-----".red() } else { "-----".white() },
               if self.leds[5] { "|".red() } else { "|".white() },
               if self.leds[1] { "|".red() } else { "|".white() },
               if self.leds[6] { "-----".red() } else { "-----".white() },
               if self.leds[4] { "|".red() } else { "|".white() },
               if self.leds[2] { "|".red() } else { "|".white() },
               if self.leds[3] { "-----".red() } else { "-----".white() },
               if self.leds[7] { ".".red() } else { ".".white() },
        );
    }
}