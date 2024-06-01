#![crate_type = "lib"]
#![crate_name = "rary"]

#[allow(dead_code)]
pub fn public_func()
{
    println!("called rary's `public_func()`");
}

fn private_func()
{
    println!("called rary's `private_func()`");
}

