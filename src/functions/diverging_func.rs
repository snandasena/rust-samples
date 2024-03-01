//use core::panicking::panic;


pub fn func()
{
    // let x: ! = panic("this function never returns");
    // println!("You will never see this line!");

    fn sum_of_od_numbers(up_to: u32) -> u32
    {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i % 2 == 1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }

    println!("Sum of odd numbers up to 9 (excluding):{}", sum_of_od_numbers(9));
}
