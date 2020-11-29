mod gauss;

#[derive(Clone, Debug, PartialEq)]
pub struct BitMatrix {
    data: [u8; 1024]
}

impl BitMatrix {
    pub fn zero() -> BitMatrix
    {
        BitMatrix {
            data: [0u8; 1024]
        }
    }

    pub fn identity() -> BitMatrix
    {
        let mut result = BitMatrix::zero();

        for i in 0 .. 32 {
            result.set(&i, &i, 255u8);
        }

        result
    }

    fn parse_diffusion_el(mask : &u32) -> [u8; 32]
    {
        let mut result = [0u8; 32];
        for i in 0 .. 32 {
            if ((mask >> i) & 1) != 0u32 {
                result[i] = 255u8;
            }
        }
        result
    }
    
    pub fn from_diffusion(diffusion : &[u32; 32]) -> BitMatrix
    {
        let mut result = BitMatrix::zero();
    
        for row in 0 .. 32u32 {
            let diffusion = BitMatrix::parse_diffusion_el(&diffusion[row as usize]);
            result.set_row(&row, &diffusion);
        }
    
        result
    }

    fn make_diffusion_el(row : &[u8; 32]) -> Option<u32> {
        let mut result = 0u32;
        for i in 0 .. 32 {
            if row[i] == 255u8 {
                result ^= 1 << i;
            }
            else if row[i] != 0 {
                return None;
            }
        }
        Some(result)
    }

    pub fn to_diffusion(&self) -> Option<[u32; 32]> {
        let mut result = [0u32; 32];

        for i in 0 .. 32 {
            if let Some(mask) = BitMatrix::make_diffusion_el(&self.get_row(&i)) {
                result[i as usize] = mask;
            }
            else {
                return None;
            }
        }

        Some(result)
    }

    pub fn inverse(&self) -> Option<BitMatrix> {
        gauss::invert(self)
    }

    pub fn set(&mut self, row : &u32, column : &u32, value: u8) {
        self.data[(row * 32 + column) as usize] = value;
    }

    pub fn set_row(&mut self, row : &u32, value : &[u8; 32]) {
        for column in 0 .. 32 {
            self.set(row, &column, value[column as usize]);
        }
    }

    pub fn get(&self, row : &u32, column : &u32) -> u8 {
        self.data[(row * 32 + column) as usize]
    }

    pub fn get_row(&self, row: &u32) -> [u8; 32] {
        let mut result = [0u8; 32];
        for col in 0 .. 32u32 {
            result[col as usize] = self.get(row, &col);
        }
        result
    }

    pub fn multiply(&self, x : &[u8; 32]) -> [u8; 32] {
        let mut result = [0u8; 32];

        for row in 0 .. 32 {
            let mut value = 0u8;
            for col in 0 .. 32 {
                value ^= self.get(&row, &col) & x[col as usize];
            }
            result[row as usize] = value;
        }
        result
    }

    pub fn multiply_m(&self, other : &BitMatrix) -> BitMatrix {
        let mut result = BitMatrix::zero();
        
        for row in 0 .. 32u32 {
            for col in 0 .. 32u32 {
                let mut value = 0u8;
                for k in 0 .. 32u32 {
                    value ^= self.get(&row, &k) & other.get(&k, &col);
                }
                result.set(&row, &col, value);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::forward::DIFFUSION;

    #[test]
    fn inverse_test() {
        let matrix = super::BitMatrix::from_diffusion(&DIFFUSION);
        let maybe_inverse = matrix.inverse();

        if let Some(inverse) = maybe_inverse {
            assert_eq!(super::BitMatrix::identity(), matrix.multiply_m(&inverse));
            assert_eq!(super::BitMatrix::identity(), inverse.multiply_m(&matrix));
        }
        else {
            panic!("Inverse could not be computed");
        }
    }

    #[test]
    fn diffusion_test() {
        let matrix = super::BitMatrix::from_diffusion(&DIFFUSION);
        let maybe_diff = matrix.to_diffusion();

        if let Some(diff) = maybe_diff {
            assert_eq!(DIFFUSION, diff);
        }
        else {
            panic!("Diffusion could not be computed");
        }
    }
}
