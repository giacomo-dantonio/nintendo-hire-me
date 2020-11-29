pub mod clarify;
pub mod deconvolute;
pub mod enlighten;

pub fn solve(input : &[u8; 32]) -> Option<[u8; 32]> {
    let mut result : Option<[u8; 32]> = None;

    for candidate in enlighten::enlighten_iter(input) {
        result = solve_cycle(&candidate);
        if result.is_some() {
            break;
        }
    }

    result
}

pub fn solve_cycle(input : &[u8; 32]) -> Option<[u8; 32]> {
    let mut result : Option<[u8; 32]> = Some(*input);

    for _ in 0 .. 256 {
        let candidate = result.unwrap();
        result = None;

        for clarity in clarify::clarify_iter(&candidate) {
            let candidate = deconvolute::deconvolute(&clarity);
            if clarify::validate(&candidate) {
                result = Some(candidate);
                break;
            }
        }

        if result.is_none() {
            return None
        }
    }

    result
}