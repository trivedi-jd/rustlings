fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    // Fix:
    // let a: [char;100] = ['a';100];
    let a: [u32;100] = [0;100];

    // Print all the values
    // for i in a.iter() {
    //     println!("{}", i);
    // }

    println!("{}",a.len());
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
