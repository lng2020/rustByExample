fn main() {
    let iter_names = vec!["Bob", "Frank", "Ferris"];
    
    for name in iter_names.iter() {
        match name {
           &"Ferris" => println!("There is a rustacean among us!"),
           _ => println!("Hello {}", name),
        }
    }

    println!("iter_names: {:?}", iter_names);

    let into_iter_names = vec!["Bob", "Frank", "Ferris"];

    for name in into_iter_names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // println!("into_iter_names: {:?}", into_iter_names);

    let mut iter_mut_names = vec!["Bob", "Frank", "Ferris"];

    for name in iter_mut_names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("iter_mut_names: {:?}", iter_mut_names);
}