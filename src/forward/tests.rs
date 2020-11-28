use std::str;

fn forward(c: &mut [u8; 32], d: &mut[u8; 32], s: &[u8; 512], p: &[u32;32])
{
    for _ in 0 .. 256usize
    {
        for j in 0 .. 32usize
        {
            d[j]=s[c[j] as usize];
            c[j]=0;
        }

        for j in 0 .. 32usize
        {
            for k in 0 .. 32usize
            {
                c[j]^=d[k] * (((p[j]>>k)&1) as u8);
            }
        }
    }

    for i in 0 .. 16usize
    {
        d[i]=s[c[i*2] as usize]^s[c[i*2+1] as usize + 256];
    }
}

#[test]
fn refactoring_didnt_break() {
    let mut input : [u8; 32] = [
        0x66,0xd5,0x4e,0x28,0x5f,0xff,0x6b,0x53,0xac,0x3b,0x34,0x14,0xb5,0x3c,0xb2,0xc6,
        0xa4,0x85,0x1e,0x0d,0x86,0xc7,0x4f,0xba,0x75,0x5e,0xcb,0xc3,0x6e,0x48,0x79,0x8f
    ];

    let mut expected = [0u8; 32];
    forward(&mut input, &mut expected, &super::CONFUSION, &super::DIFFUSION);
    let expected_str = str::from_utf8(&expected[..16]).unwrap_or("not a valid ut8-string");

    input = [
        0x66,0xd5,0x4e,0x28,0x5f,0xff,0x6b,0x53,0xac,0x3b,0x34,0x14,0xb5,0x3c,0xb2,0xc6,
        0xa4,0x85,0x1e,0x0d,0x86,0xc7,0x4f,0xba,0x75,0x5e,0xcb,0xc3,0x6e,0x48,0x79,0x8f
    ];
    let actual : [u8;32] = super::forward(&mut input);
    let actual_str = str::from_utf8(&actual[..16]).unwrap_or("not a valid ut8-string");

    assert_eq!(expected[..16], actual[..16]);
    assert_eq!(expected_str, actual_str);
}
