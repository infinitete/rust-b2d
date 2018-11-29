extern crate b2d;

use b2d::binary::BinaryString;

fn main() {
    let mut b1 = BinaryString::new("01111111111111111111111111111111");
    println!("{:?}", b1.to_decimal().unwrap());
    println!("{:?}", b1.to_decimal());
    println!("{:?}", BinaryString::new("01001").to_decimal());
}
