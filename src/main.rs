// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

//const STARTING_MISSILES: i32 = 8;
//const READY_AMOUNT: i32 = 2;

fn main() {
    //part A
    // let mut missiles= STARTING_MISSILES;
    // let ready:i32=READY_AMOUNT;
    // println!("Firing {} of my {} missiles...", ready, missiles);
    // missiles= missiles - ready;
    // println!("missiles left {} ...", missiles);

    //part B
    let width = 4;
    let height = 7;
    let depth = 10;
    
        let area = area_of(width, height);

    println!("Area is {}", area);
      println!("Volume is {}", volume(width, height, depth));
}

fn area_of(x: i32, y: i32) -> i32 {

    return x*y;
    // Challenge: It isn't idiomatic (the normal way a Rust programmer would do things) to use
    //            `return` on the last line of a function. Change the last line to be a
    //            "tail expression" that returns a value without using `return`.
    //            Hint: `cargo clippy` will warn you about this exact thing.
}
 fn volume(width:i32, height:i32, depth:i32)-> i32{
       width*height*depth
 }