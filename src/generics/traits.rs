struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal
{
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn talk(&self)
    {
        println!("{}  says {}", self.name(), self.noise());
    }
}

impl Sheep
{
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self)
    {
        if self.is_naked() {
            println!("{} is already naked...", self.name())
        } else {
            println!("{} gets a haircut!", self.name());
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Self {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked()
        {
            "baaaah!"
        } else {
            "baaaaah!"
        }
    }

    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}


struct Empty;

struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
}

pub fn test_generic_traits()
{
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();

    /////
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);
}


