use crate::bitmatrix::BitMatrix;
use crate::forward::DIFFUSION;

fn make_diffusion(mask : &u32) -> [u8; 32]
{
    let mut result = [0u8; 32];
    for i in 0 .. 32 {
        if ((mask >> i) & 1) != 0u32 {
            result[i] = 255u8;
        }
    }
    result
}

fn diffusion_matrix() -> BitMatrix
{
    let mut result = BitMatrix::zero();

    for row in 0 .. 32u32 {
        let diffusion = make_diffusion(&DIFFUSION[row as usize]);
        result.set_row(&row, &diffusion);
    }

    result
}

#[cfg(test)]
mod tests {
    fn next_input() -> u8
    {
        let mut val : u8 = rand::random::<u8>();
        while val == 250 {
            val = rand::random::<u8>()
        }
        val
    } 

    fn convolute(input: &[u8; 32]) -> [u8; 32]
    {
        let matrix = super::diffusion_matrix();
        matrix.multiply(input)
    }

    #[test]
    fn convolute_matrix_test() {
        for _ in 0 .. 50
        {
            let mut input = [0u8; 32];
            for i in 0 .. 32 {
                input[i] = next_input();
            }

            let expected = crate::forward::convolute(&input);
            let actual = convolute(&input);

            assert_eq!(expected, actual);
        }
    }
}