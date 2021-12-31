use std::{
    cell::Cell,
    ops::Deref,
    pin::Pin,
};

struct MyP<P> {
    pointer: P,
}

impl<P: Deref> Deref for MyP<P> {
    type Target = P::Target;
    fn deref(&self) -> &Self::Target {
        &self.pointer
    }
}

struct I {
    pub i: i32,
}

impl I {
    pub fn h(&self) {
        println!("i = {} \n ", self.i)
    }
}

fn main() {
    let myp = MyP {
        pointer: Box::new(I { i: 1 }),
    };
    myp.h();
    let myp = MyP {
        pointer: Cell::new(I { i: 1 }),
    };
    myp.h();
}
