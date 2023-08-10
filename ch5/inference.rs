fn main() {
    let elem = 5u8;

    let mut vec = Vec::new();

    vec.push(elem);
    // the compiler knows that `vec` is a vector of `u8`s

    println!("{:?}", vec);
}