mod functions;

use functions::methods;
use functions::closure;

fn main() {
    methods::test_methods();
    closure::test_closure();
    closure::test_closure_as_input_params();
}
