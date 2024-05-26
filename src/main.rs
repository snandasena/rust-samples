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
    modules::test_declaration();
    modules::test_self_n_super();
}

fn main() {
    test_functions();
    test_modules();
}
