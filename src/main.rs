mod functions;
mod modules;
mod my_crate;
mod generics;


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

#[cfg(target_os = "linux")]
fn are_you_on_linux()
{
    println!("You are on linux!");
}

fn test_generics()
{
    generics::test_function();
    generics::test_impl();
    generics::test_traits();
}


fn main() {
    are_you_on_linux();

    test_functions();
    test_modules();

    test_generics();
}
