mod functions;
mod implementation;
mod traits;
mod bounds;

pub fn test_function()
{
    functions::test_generic_functions();
}

pub fn test_impl()
{
    implementation::test_generic_impl();
}

pub fn test_traits()
{
    traits::test_generic_traits();
}

pub fn test_bounds()
{
    bounds::test_generic_bounds();
}