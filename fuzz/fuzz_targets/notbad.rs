#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate fuzz_fun;

fn notbad(input: &[u8]) {
    let bad = vec![b'S', b'u', b'p', b'e', b'r', b'B', b'a', b'd', b'!'];

    let matches = bad.iter().zip(input)
        .fold(0, |s, (&a, &b)| if a == b { s + 1 } else { s });
    if matches == bad.len() { panic!("BAD! input"); }

    return
}

fuzz_target!(|data: &[u8]| {
    notbad(data);
});
