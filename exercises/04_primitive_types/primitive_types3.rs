fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    let a: [i32; 100] = [42;100];
    let b = a[a.len()-1];

    if a.len() >= 100 {
        println!("Wow, that's a big array! {b}");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
