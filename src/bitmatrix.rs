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

    pub fn set(&mut self, row : &u32, column : &u32, value: u8) {
        self.data[(row * 32 + column) as usize] = value;
    }

    pub fn set_row(&mut self, row : &u32, value : &[u8; 32])
    {
        for column in 0 .. 32 {
            self.set(row, &column, value[column as usize]);
        }
    }

    pub fn get(&self, row : &u32, column : &u32) -> u8 {
        self.data[(row * 32 + column) as usize]
    }

    pub fn multiply(&self, x : &[u8; 32]) -> [u8; 32]
    {
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
}