pub mod methods {
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }

        fn new(x: f64, y: f64) -> Point {
            Point { x, y }
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {
        fn area(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            ((x1 - x2) * (y1 - y2)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        }

        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;

            self.p1.y += y;
            self.p2.y += y;
        }
    }

    struct Pair(Box<i32>, Box<i32>);

    impl Pair {
        fn destroy(self)
        {
            let Pair(first, second) = self;

            println!("Destroying pair({}, {})", first, second);
        }
    }

    pub fn test_methods()
    {
        let rectangle = Rectangle { p1: Point::origin(), p2: Point::new(3.0, 4.0) };
        println!("Rectangle perimeter: {}", rectangle.perimeter());
        println!("Rectangle area: {}", rectangle.area());

        // rectangle.translate(1.0, 1.0); immutable rectangle object

        let mut square = Rectangle {
            p1: Point::origin(),
            p2: Point::new(1.0, 1.0),
        };

        square.translate(1.0, 1.0);

        let pair = Pair(Box::new(1), Box::new(2));
        pair.destroy();

        // pair.destroy(); cannot destroy: already destroyed.
    }
}

pub mod closure {
    pub fn test_closure() {
        let outer_var = 42;

        let closure_annotated = |i: i32| -> i32{ i + outer_var };
        let closure_inferred = |i| i + outer_var;

        println!("closure_annotated: {}", closure_annotated(1));
        println!("closure_inferred: {}", closure_inferred(1));
    }

    fn apply<F>(f: F) where
        F: FnOnce()
    {
        f();
    }

    fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
        f(3)
    }

    pub fn test_closure_as_input_params()
    {
        use std::mem;

        let greeting = "hello";
        let mut farewell = "goodbye".to_owned();

        let dairy = || {
            println!("I said {}.", greeting);

            farewell.push_str("!!!");
            println!("Then I screamed {}.", farewell);
            println!("Now I can sleep. zzzzzzzzzzzzzzz");

            mem::drop(farewell);
        };

        apply(dairy);

        let double = |x| 2 * x;
        println!("3 doubled: {}", apply_to_3(double));
    }
}