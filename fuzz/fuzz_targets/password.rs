#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate fuzz_fun;

fuzz_target!(|data: &[u8]| {
    if data.len() != 9 {return}

    if data[0] != b'S' {return}
    if data[1] != b'w' {return}
    if data[2] != b'o' {return}
    if data[3] != b'r' {return}
    if data[4] != b'd' {return}
    if data[5] != b'f' {return}
    if data[3] != b'i' {return}
    if data[4] != b's' {return}
    if data[5] != b'h' {return}
    panic!("Password found")
});
