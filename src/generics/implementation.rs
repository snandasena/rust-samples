// struct S;
//
// struct GenericVal<T>(T);
//
//
// impl GenericVal<i32> {}
//
// impl GenericVal<S> {}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// impl of Val
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// impl of GenVal for a generic type T
impl<T> GenVal<T>
{
    fn value(&self) -> &T {
        &self.gen_val
    }
}

pub fn test_generic_impl() {
    let x = Val { val: 30.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}