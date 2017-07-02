// SUMMARY
// * At any given time, you can have either but not both of:
//  ** one mutable reference
//  ** any number of immutable references
// * References must be always valid 

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // cannot change borrowed variable
    // change(&s1);
    let mut s2 = String::from("hello");
    change_mutable_reference(&mut s2);
    println!("{}", s2);

    let r1 = &mut s2;
    // Error - cannot borrow as mutable more than once
    // let r2 = &mut s;
    
    // Cannot create dangling reference
    //let reference_to_nothing = dangle();

    let string = no_dangle();
    println!("{}", string);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// cannot modify reference
//fn change(some_string: &String) {
//    some_string.push_str(", world");
//}

fn change_mutable_reference(some_string: &mut String) {
    some_string.push_str(", world");
}

// cannot return reference to owned variable
//fn dangle() -> &String {
//    let  s = String::from("hello");
//    &s
//}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
