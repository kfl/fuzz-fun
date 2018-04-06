#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate fuzz_fun;

fn sorted<T: PartialOrd>(arr: & [T]) -> bool {
    if arr.len() <= 1 { return true; }
    for i in 0 .. arr.len() - 1 {
        if arr[i] > arr[i + 1] {
            return false;
        }
    }
    return true;
}


fuzz_target!(|data: & [u8]| {
    let mut d = data.to_vec();
    fuzz_fun::quicksort(&mut d);
    assert!(sorted(&d));
});
