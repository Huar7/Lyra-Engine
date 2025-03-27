mod mathly;
use mathly::*;

mod graphly;

fn main() {
    let nil: Vec2 = Vec2::new(32.0, 32.0);
    let nil2: Vec2 = Vec2::new(33.0, 31.9);

    let nil3 = nil + nil2;
    println!("{:?}, {:?}, {:?}", nil, nil2, nil3);
    println!("kontol {:?}", nil);
}
