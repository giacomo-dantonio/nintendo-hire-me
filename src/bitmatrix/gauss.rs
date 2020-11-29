use crate::bitmatrix::BitMatrix;

pub fn sum_vectors(lhs : &[u8; 32], rhs : &[u8; 32]) -> [u8; 32] {
    let mut result = [0u8; 32];

    for i in 0 .. 32 {
        result[i] = lhs[i] ^ rhs[i];
    }

    result
}

pub fn swap_rows(mat : &mut BitMatrix, i : &u32, j : &u32) {
    let tmp = mat.get_row(i);
    mat.set_row(i, &mat.get_row(j));
    mat.set_row(j, &tmp);
}

// i -> i + j
pub fn add_row(mat : &mut BitMatrix, i : &u32, j : &u32) {
    let row = sum_vectors(&mat.get_row(i), &mat.get_row(j));
    mat.set_row(i, &row);
}

// for triangular matrices this is the determinant
pub fn diagonal_product(mat : &BitMatrix) -> u8 {
    let mut value = 255u8;
    for i in 0 .. 32u32 {
        value &= mat.get(&i, &i);
    }
    value
}

pub fn invert(mat: &BitMatrix) -> Option<BitMatrix> {
    let mut transform = BitMatrix::identity();
    let upper = make_upper_triangular(mat, &mut transform);

    if diagonal_product(&upper) != 0 {
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
            swap_rows(mat, row, &j);
            swap_rows(acc, row, &j);
        }
        Some(j)
    }
}

fn gauss_step(mat: &mut BitMatrix, acc: &mut BitMatrix, row: &u32, col: &u32)
{
    for j in (row + 1) .. 32u32 {
        let value = mat.get(&j, &col);
        if value == 255u8 {
            add_row(mat, &j, &row);
            add_row(acc, &j, &row);
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
            add_row(mat, &j, &row);
            add_row(acc, &j, &row);
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
