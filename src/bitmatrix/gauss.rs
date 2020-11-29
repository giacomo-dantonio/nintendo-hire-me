use crate::bitmatrix::BitMatrix;

pub fn invert(mat: &BitMatrix) -> Option<BitMatrix> {
    let mut transform = BitMatrix::identity();
    let upper = make_upper_triangular(mat, &mut transform);

    if upper.diagonal_product() != 0 {
        make_lower_triangular(&upper, &mut transform);
        Some(transform)
    }
    else {
        None
    }
}

pub fn make_upper_triangular(mat : &BitMatrix, acc: &mut BitMatrix) -> BitMatrix {
    let mut result = mat.clone();

    let mut col = 0u32;
    for row in 0 .. 32u32
    {
        // make sure the pivot is not zero
        let mut pivot = non_zero_pivot(&mut result, acc, &row, &col);
        while pivot.is_none() && col < 32 {
            col += 1;
            pivot = non_zero_pivot(&mut result, acc, &row, &col);
        }

        if col == 32 {
            break;
        }
        gauss_step(&mut result, acc, &row, &col);
        col += 1;
    }

    result
}

// here we assume mat is upper triangular non singular
fn make_lower_triangular(mat : &BitMatrix, acc: &mut BitMatrix) -> BitMatrix {
    let mut result = mat.clone();

    for i in 0 .. 32u32 {
        let row = 31 - i;
        let col = row;

        reverse_gauss_step(&mut result, acc, &row, &col);
    } 

    result
}

fn non_zero_pivot(mat: &mut BitMatrix, acc: &mut BitMatrix, row: &u32, col: &u32) -> Option<u32> {
    let mut j = *row;
    while j < 32u32 && mat.get(&j, col) == 0 {
        j += 1;
    }

    if j == 32 {
        None
    }
    else {
        if j != *row {
            mat.swap_rows(row, &j);
            acc.swap_rows(row, &j);
        }
        Some(j)
    }
}

fn gauss_step(mat: &mut BitMatrix, acc: &mut BitMatrix, row: &u32, col: &u32)
{
    for j in (row + 1) .. 32u32 {
        let value = mat.get(&j, &col);
        if value == 255u8 {
            mat.add_row(&j, &row);
            acc.add_row(&j, &row);
        }
        else if value != 0u8 {
            // this should not actually happen
            panic!("unexpected value");
        }
    }
}

fn reverse_gauss_step(mat: &mut BitMatrix, acc: &mut BitMatrix, row: &u32, col: &u32)
{
    for j in 0 .. *row {
        let value = mat.get(&j, &col);
        if value == 255u8 {
            mat.add_row(&j, &row);
            acc.add_row(&j, &row);
        }
        else if value != 0u8 {
            // this should not actually happen
            panic!("unexpected value");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bitmatrix::BitMatrix;
    use crate::forward::DIFFUSION;

    #[test]
    fn test_upper_triangular() {
        let mat = BitMatrix::from_diffusion(&DIFFUSION);
        let mut acc = super::BitMatrix::identity();

        let triangular = super::make_upper_triangular(&mat, &mut acc);
        println!("{:#?}", triangular);
    }

    #[test]
    fn invert_test() {
        let mat = BitMatrix::from_diffusion(&DIFFUSION);
        let maybe_inv = super::invert(&mat);

        if let Some(inv) = maybe_inv {
            let check = mat.multiply_m(&inv);
            assert_eq!(check, super::BitMatrix::identity());
        }
        else {
            panic!("diffusion matrix could not be inverted");
        }
    }
}
