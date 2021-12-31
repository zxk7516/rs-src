#![allow(non_snake_case)]
use std::{
    cell::RefCell,
    rc::Rc,
};

use redis::RedisResult;
pub mod atom;
pub mod lock;
use lock::RedLock;

fn main() {
    // let client = redis::Client::open("redis://127.0.0.1/").expect("redis connect failed!");
    // let mut conn = client
    //     .get_connection()
    //     .expect("get redis connection failed");
    //
    // let atomScript = redis::Script::new(include_str!("atom.lua"));
    //
    // let res: RedisResult<i32> = atomScript.key("ticket").arg(20).arg(50).invoke(&mut conn);
    //
    // let lock = RedLock::new("key1", 5 * 60 * 1000, &mut conn).unwrap();
    // let _ = lock.unlock(&mut conn);

    let s = RefCell::new(S {
        s: "hello".to_string(),
    });
    s.borrow_mut().change_name("Hi".to_string());
    s.borrow().say();
}

struct S {
    s: String,
}
impl S {
    pub fn change_name(
        &mut self,
        s: String,
    ) {
        self.s = s;
    }
    pub fn say(&self) {
        println!("{}", self.s);
    }
}
