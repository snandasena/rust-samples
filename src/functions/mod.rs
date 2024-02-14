mod captures;
mod methods;

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
}


