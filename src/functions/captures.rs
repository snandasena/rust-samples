use std::mem;

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


pub fn test_capture()
{
    use std::mem;

    let color = String::from("Green");
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow.
    print();

    let _reborrow = &color;
    print();

    let _color_moved = color;
    // print(); // color is moved

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow.
    inc();

    // An attempt to reborrow will lead to an error.
    // let _reborrow = &count;
    inc();

    let _count_reborrow = &mut count;

    // A non-copy type.
    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();
    // consume();
}

pub fn test_move()
{
    // non-copy semantics.
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // vec is already moved. if move is removed, following code line will be worked.
    // println!("There are {} elements in vec", haystack.len());
}