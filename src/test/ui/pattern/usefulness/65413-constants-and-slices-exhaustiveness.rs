#![feature(slice_patterns)]
#![deny(unreachable_patterns)]

const C0: &'static [u8] = b"\x00";

fn main() {
    let x: &[u8] = &[0];
    match x {
        &[] => {}
        &[1..=255] => {}
        // this shouldn't be unreachable
        C0 => {} //~ unreachable pattern
        &[_, _, ..] => {}
    }
}
