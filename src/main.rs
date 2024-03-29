use std::str;
use bitmatrix::BitMatrix;
use forward::DIFFUSION;

mod forward;
mod solve;
mod bitmatrix;

fn as_bytes(target_str : &str) -> [u8; 32] {
    let target = target_str.as_bytes();
    let mut target_32 = [0u8; 32];
    let len = std::cmp::min(target.len(), 32);
    for i in 0 .. len {
        target_32[i] = target[i];
    }
    target_32
}

fn main() {
    let mut input : [u8; 32] = [
        //change only this :
        0x66,0xd5,0x4e,0x28,0x5f,0xff,0x6b,0x53,0xac,0x3b,0x34,0x14,0xb5,0x3c,0xb2,0xc6,
        0xa4,0x85,0x1e,0x0d,0x86,0xc7,0x4f,0xba,0x75,0x5e,0xcb,0xc3,0x6e,0x48,0x79,0x8f
        //
    ];

    let output : [u8;32] = forward::forward(&mut input);
    let str_output = str::from_utf8(&output).unwrap_or("not a valid ut8-string");

    println!("{:02x?}", output);
    println!("{}", str_output);

    let matrix = BitMatrix::from_diffusion(&DIFFUSION);
    let inverse = matrix.inverse().unwrap();
    let accumulation = inverse.to_diffusion().unwrap();
    println!("{:#x?}", accumulation);

    let target = as_bytes("Hire me!!!!!!!!");
    let solution = solve::solve(&target);
    println!("{:#x?}", solution);
}
