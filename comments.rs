#![crate_name = "doc"] // this is the name of the program that will be compiled by rustc

//Comment one line

/*

Comment multi line
also known as block comment

*/

/// main docs
fn main(){
    /// This is a comment go direct to html docs
    /// it supports markdown syntax! *amazing!*
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);


}