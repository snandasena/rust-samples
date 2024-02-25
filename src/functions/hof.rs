fn is_odd(i: u32) -> bool
{
    i % 2 != 0
}

pub fn test_hof()
{
    let upper = 1000;

    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;

        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }

    println!("imperative style: {}", acc);

    let sum_of_squared_odd_numbers: u32 = (0..).map(|n| n * n)
        .take_while(|&n_squared| n_squared <= upper)
        .filter(|&n_squared| is_odd(n_squared))
        .sum();

    println!("functional style: {}", sum_of_squared_odd_numbers)
}