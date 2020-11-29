use crate::bitmatrix::BitMatrix;

// opposite of DIFFUSION
static ACCUMULATION : [u32; 32] = [
    0xf26cb481,
    0x16a5dc92,
    0x3c5ba924,
    0x79b65248,
    0x2fc64b18,
    0x615acd29,
    0xc3b59a42,
    0x976b2584,
    0x6cf281b4,
    0xa51692dc,
    0x5b3c24a9,
    0xb6794852,
    0xc62f184b,
    0x5a6129cd,
    0xb5c3429a,
    0x6b978425,
    0xb481f26c,
    0xdc9216a5,
    0xa9243c5b,
    0x524879b6,
    0x4b182fc6,
    0xcd29615a,
    0x9a42c3b5,
    0x2584976b,
    0x81b46cf2,
    0x92dca516,
    0x24a95b3c,
    0x4852b679,
    0x184bc62f,
    0x29cd5a61,
    0x429ab5c3,
    0x84256b97,
];

fn deconvolute(input : &[u8; 32]) -> [u8; 32] {
    let matrix = BitMatrix::from_diffusion(&ACCUMULATION);
    matrix.multiply(input)
}

#[cfg(test)]
mod tests {
    use crate::bitmatrix::BitMatrix;
    use crate::forward::DIFFUSION;

    fn convolute(input: &[u8; 32]) -> [u8; 32]
    {
        let matrix = BitMatrix::from_diffusion(&DIFFUSION);
        matrix.multiply(input)
    }

    #[test]
    fn convolute_matrix_test() {
        for _ in 0 .. 50
        {
            let mut input = [0u8; 32];
            for i in 0 .. 32 {
                input[i] = rand::random::<u8>();
            }

            let expected = crate::forward::convolute(&input);
            let actual = convolute(&input);

            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn deconvolute_test() {
        for _ in 0 .. 50
        {
            let mut input = [0u8; 32];
            for i in 0 .. 32 {
                input[i] = rand::random::<u8>();
            }

            let actual = crate::forward::convolute(&super::deconvolute(&input));
            assert_eq!(input, actual);
        }
    }
}