#![feature(generators, generator_trait)]
use std::{
    fs::File,
    io::{
        self,
        Read,
    },
    ops::{
        ControlFlow,
        Generator,
        GeneratorState,
    },
    pin::Pin,
};

use futures::future::{
    self,
    Future,
};

async fn example(min_len: usize) -> String {
    let content = async_read_file("foo.txt").await.unwrap();
    if content.len() < min_len {
        content + &async_read_file("bar.txt").await.unwrap()
    } else {
        content
    }
}

// 1 Start
// 2 Waiting for future `foo.txt`
// 3 Waiting for future `bar.txt`
// 4 End

fn async_read_file(name: &str) -> impl Future<Output = io::Result<String>> {
    let mut file = File::open(name).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf);
    future::ready(Ok(buf))
}

fn main() {
    let mut generator = || {
        yield 1;
        return "foo";
    };
    loop {
        match Pin::new(&mut generator).resume(()) {
            GeneratorState::Yielded(1) => {}
            GeneratorState::Complete("foo") => {
                break;
            }
            _ => {
                eprintln!("resume ofter completion!");
            }
        }
    }
}

// fn main() {
//     enum Po {
//         A([u8; 255]),
//         B([u8; 3]),
//     }
//     enum Po2 {
//         A([u8; 256]),
//         B([u8; 3]),
//     }
//     dbg!(std::mem::size_of::<Po>());
//     dbg!(std::mem::size_of::<Po2>());
//
//     let c = "abc你好吗";
//     dbg!(&c[1..]);
//     dbg!(&c[3..]);
//     dbg!(&c[6..]);
//     dbg!(&c[7..]);
// }

// // https://leetcode-cn.com/problems/fibonacci-number/
// pub fn fib(n: i32) -> i32 {
//     if n <= 1 {
//         n
//     } else {
//         let mut s: (i32, i32) = (0, 1);
//         for _ in 1..n {
//             s = (s.1, s.0 + s.1)
//         }
//         s.1
//     }
// }
//
// // https://leetcode-cn.com/problems/n-th-tribonacci-number/
// pub fn tribonacci(n: i32) -> i32 {
//     if n <= 1 {
//         n
//     } else {
//         let mut s: (i32, i32, i32) = (0, 1, 1);
//         for _ in 2..n {
//             s = (s.1, s.2, s.0 + s.1 + s.2)
//         }
//         s.2
//     }
// }
//
// // https://leetcode-cn.com/problems/climbing-stairs/
// pub fn climb_stairs(n: i32) -> i32 {
//     if n <= 2 {
//         return n;
//     }
//     let mut s = (1, 2);
//     for _ in 2..n {
//         s = (s.1, s.0 + s.1)
//     }
//     s.1
// }
//
// // https://leetcode-cn.com/problems/min-cost-climbing-stairs/
// pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
//     let len = cost.len();
//     let mut result = vec![0; len];
//     result[0] = cost[0];
//     result[1] = cost[1];
//     for i in 2..len {
//         result[i] = cost[i] + i32::min(result[i - 1], result[i - 2])
//     }
//     i32::min(result[len - 1], result[len - 2])
// }
//
// // https://leetcode-cn.com/problems/house-robber/
// pub fn rob(nums: Vec<i32>) -> i32 {
//     let len = nums.len();
//     let mut results = vec![0; len];
//     results[0] = nums[0];
//     results[1] = i32::max(nums[1], nums[0]);
//     for i in 2..len {
//         results[i] = i32::max(results[i - 2] + nums[i], results[i - 1])
//     }
//     i32::max(results[len - 1], results[len - 2])
// }

// fn _threads() {
//     let map = ConcurrentHashMap::<u64, u64, _>::new();
//     let mut handlers = Vec::with_capacity(200);

//     for i in 1..=200 {
//         let mut m = map.clone();
//         handlers.push(thread::spawn(move || {
//             let mut res = 1;
//             for ii in 1..=i {
//                 res *= ii
//             }
//             m.insert(i, res);
//         }))
//     }
//     for h in handlers.into_iter() {
//         h.join().unwrap();
//     }
//     for (k, v) in map.entries() {
//         println!("map[{}] = {}", k, v)
//     }
// }
