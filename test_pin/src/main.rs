#![feature(generators, generator_trait)]
use pin_project::pin_project;
use std::{
    ops::Generator,
    ops::GeneratorState,
    pin::Pin,
};

#[pin_project]
struct Test<T, U> {
    #[pin]
    pinned: T,
    unpinned: U,
}

impl<T, U> Test<T, U> {
    fn method(self: Pin<&mut Self>) {
        let this = self.project();
        let _: Pin<&mut T> = this.pinned;
        let _: &mut U = this.unpinned;
    }
}

fn main() {
    let mut gen1 = || {
        for i in 0..5 {
            yield i;
        }
        return "foo";
    };
    while let GeneratorState::Yielded(n) = Pin::new(&mut gen1).resume(()) {
        println!("Got number: {}", n);
    }
    // let foo = Pin::new(&mut gen1).resume(()); 
    // error: while let has already comsume the Generator::Complete then break out of the loop.
}
