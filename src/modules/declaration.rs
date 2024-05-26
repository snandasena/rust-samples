use deeply::nested::function as func;

fn function() {
    println!("called `function()`");
}

mod deeply
{
    pub mod nested {
        pub fn function()
        {
            println!("called `deeply::nested::function()`");
        }
    }
}


pub fn test_declaration()
{
    func();

    println!("entering new scope");
    {
        use crate::modules::declaration::deeply::nested::function;
        function();

        println!("leaving the scope");
    }

    function();
}