#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate fuzz_fun;

fuzz_target!(|d: &[u8]| {
    let mut data = d.to_vec();
    fuzz_fun::quicksort(&mut data);

    if data.len() != 9 {return}

    let mut expected = vec![b'S', b'w', b'o', b'r', b'd', b'f', b'i', b's', b'h'];
    fuzz_fun::quicksort(&mut expected);

    if data[0] != expected[0] {return}
    if data[1] != expected[1] {return}
    if data[2] != expected[2] {return}
    if data[3] != expected[3] {return}
    if data[4] != expected[4] {return}
    if data[5] != expected[5] {return}
    if data[6] != expected[6] {return}
    if data[7] != expected[7] {return}
    if data[8] != expected[8] {return}

    panic!("Password found")
});
