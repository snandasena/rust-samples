mod captures;
mod methods;
mod hof;
mod diverging_func;

pub fn test_methods()
{
    methods::test_method();
}

pub fn test_closures()
{
    captures::test_closure();
    captures::test_closure_as_input_params();
    captures::test_capture();
    captures::test_move();
    captures::test_std_fn();
}

pub fn test_hof()
{
    hof::test_hof();
}

pub fn test_diverging()
{
    diverging_func::func();
}
