use core::panicking::panic;


pub fn func()
{
    let x: ! = panic("this function never returns");
    println!("You will never see this line!");
}
