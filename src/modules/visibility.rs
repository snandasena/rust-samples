mod vis {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    pub struct CloseBox<T> {
        contents: T,
    }

    impl<T> CloseBox<T> {
        pub fn new(contents: T) -> CloseBox<T> {
            CloseBox { contents }
        }

        pub fn get(&self) -> &T {
            &self.contents
        }
    }
}


pub fn test_visibility() {
    let open_box = vis::OpenBox { contents: "public info" };
    println!("The open box contents: {}", open_box.contents);

    let close_box = vis::CloseBox::new("Classified info");
    println!("The close box contents: {}", close_box.get());
}

