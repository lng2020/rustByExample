fn destory_box(c: Box<i32>) {
    println!("Destoring a box that contains {}", c);
}

fn main() {
    let x = 5u32;
    let y = x;
    println!("x is {}, and y is {}", x, y);
    let a = Box::new(5i32);
    println!("a contains: {}", a);

    let b = a;
    // println!("a contains: {}", a);
    destory_box(b);
    // println!("b contains: {}", b);
}