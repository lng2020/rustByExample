#[derive(Debug)]
#[allow(dead_code)]
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;

    let first_available_fruit = no_fruit.or(orange).or(apple);
    println!("first_available_fruit: {:?}", first_available_fruit);

    // the variable name `apple` has been moved regardless
    // println!("Variable apple was moved, so this line won't complete: {:?}", apple);
}