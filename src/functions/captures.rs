pub fn test_closure() {
    let outer_var = 42;

    let closure_annotated = |i: i32| -> i32{ i + outer_var };
    let closure_inferred = |i| i + outer_var;

    println!("closure_annotated: {}", closure_annotated(1));
    println!("closure_inferred: {}", closure_inferred(1));
}

fn apply<F>(f: F) where
    F: FnOnce()
{
    f();
}

fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
    f(3)
}

pub fn test_closure_as_input_params()
{
    use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let dairy = || {
        println!("I said {}.", greeting);

        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzzzzzzzzzzzz");

        mem::drop(farewell);
    };

    apply(dairy);

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}
