mod indices;

// FIXME: clarify should apply deconvolute to the tentative output.
// If some element in the output is 250, it should use an alternative
// clarity for some of the input elements.
// it should only try to change the relevant elements
// (look at the diffusion matrix for this).

pub fn clarify(input: &[u8; 32]) -> Option<[u8; 32]> {
    let mut result = [0; 32];
    for index in 0 .. 32 {
        if let [val, ..] = indices::CLARITIES[input[index] as usize][..] {
            result[index] = val
        }
        else {
            return None
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use crate::solve::deconvolute::deconvolute;
    use super::{clarify, indices::MISSING_INDICES};

    fn next_input() -> u8
    {
        let mut val : u8 = rand::random::<u8>();
        while MISSING_INDICES.contains(&val) {
            val = rand::random::<u8>()
        }
        val
    } 

    #[test]
    fn test_inverse() {
        for _ in 0 .. 50
        {
            let mut input = [0u8; 32];
            for i in 0 .. 32 {
                input[i] = next_input();
            }
    
            if let Some(inverse) = super::clarify(&input) {
                let actual = crate::forward::confuse(&inverse);
                assert_eq!(input, actual);
            }
            else {
                panic!("Couldn't compute inverse");
            }
        }
    }

    #[test]
    fn test_valid() {
        for _ in 0 .. 50 {
            let mut input = [0u8; 32];
            for i in 0 .. 32 {
                input[i] = next_input();
            }

            let clear = clarify(&input).unwrap();
            let next_input = deconvolute(&clear);

            assert!(next_input.iter().all(|x| !MISSING_INDICES.contains(x)));
        }
    }
}