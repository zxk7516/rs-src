#![no_main]
use libfuzzer_sys::fuzz_target;

#[derive(Clone, Debug, arbitrary::Arbitrary)]
struct MemoryInput {
    dest: Vec<u8>,
    src: Vec<u8>,
    n: usize,
}

fuzz_target!(|data: MemoryInput| {
    let mut data = data.clone();
    fuzzt::vulnerable_copy(&mut data.dest, &data.src, data.n);
});
