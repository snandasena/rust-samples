fn function()
{
    println!("called `function()`")
}

mod cool {
    pub fn function()
    {
        println!("called `cool::function()`")
    }
}

mod my {
    fn function()
    {
        println!("called `my::function()`")
    }


    mod cool {
        pub fn function()
        {
            println!("called `my::cool::function()`")
        }
    }

    pub fn indirect_call()
    {
        println!("Called `my::indirect_call()`");
        self::function();
        function();

        self::cool::function();
        super::function();

        {
            use crate::modules::self_n_super::function as root_function;
            root_function();
        }
    }
}


pub fn test_self_n_super()
{
    my::indirect_call();
}