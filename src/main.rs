extern crate core;

mod functions;

fn test_functions()
{
    functions::test_methods();
    functions::test_closures();
    functions::test_hof();
    functions::test_diverging();
}

fn main() {
    test_functions();
}
