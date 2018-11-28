extern crate b2d;

use b2d::binary::is_binary_string;

fn main() {
    println!("{}", is_binary_string("11111122222233333"));
    println!("{}", is_binary_string("111111"));
}
