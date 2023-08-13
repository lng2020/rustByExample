fn main() {
    let outer_var = 42;

    // fn function(i: i32) -> i32 { i + outer_var} 

    let closure_annotated = |i: i32| -> i32 { i + outer_var};
    let closure_inferred = |i| i + outer_var;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));

    // println!("caanot reuse closure_inferred with another type: {}", closure_inferred(42i64));

    let one = || 1;
    println!("closure returning one: {}", one());
}