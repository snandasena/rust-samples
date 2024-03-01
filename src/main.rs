mod functions;
mod modules;

fn test_functions()
{
    functions::test_methods();
    functions::test_closures();
    functions::test_hof();
    functions::test_diverging();
}

fn test_modules()
{
    modules::test_visibility();
}

fn main() {
    test_functions();
    test_modules();
}
