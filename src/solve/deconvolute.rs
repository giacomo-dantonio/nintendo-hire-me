
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
}