use crate::forward::CONFUSION;
use itertools::Itertools;

static LEFT_CLARITY : [Option<u8>; 256] = [
    Some(106), Some(26), Some(24), Some(89), Some(56), Some(86), Some(227),
    Some(195), Some(228), Some(95), Some(188), Some(144), Some(47), Some(223),
    Some(70), None, Some(88), None, Some(245), Some(42), Some(247), Some(191),
    Some(128), Some(10), Some(204), Some(221), Some(23), Some(246), Some(83),
    Some(131), Some(129), Some(4), None, Some(130), Some(240), Some(145),
    Some(48), Some(2), Some(112), Some(158), Some(7), Some(253), Some(232),
    Some(9), Some(118), Some(132), Some(222), Some(176), Some(81), Some(251),
    Some(200), Some(6), Some(140), Some(115), Some(99), Some(210), Some(110),
    Some(68), Some(197), Some(216), Some(148), Some(218), None, Some(230),
    Some(161), Some(162), Some(37), Some(117), None, Some(248), Some(238),
    Some(100), Some(18), Some(77), Some(43), Some(33), Some(125), Some(67),
    Some(146), Some(183), Some(189), Some(92), Some(82), Some(39), Some(149),
    Some(14), Some(104), Some(184), Some(193), Some(75), None, Some(215),
    Some(59), Some(15), Some(123), Some(198), Some(252), Some(121), Some(22),
    Some(40), Some(91), Some(239), Some(41), Some(214), Some(196), Some(80),
    Some(34), None, Some(62), Some(155), Some(51), Some(244), Some(233),
    Some(31), Some(32), Some(255), Some(119), None, Some(49), Some(127),
    Some(21), Some(157), Some(174), Some(192), Some(8), Some(236), Some(226),
    Some(103), None, Some(60), Some(165), Some(30), Some(142), Some(45),
    Some(65), Some(250), Some(185), Some(153), Some(72), Some(205), Some(71),
    Some(137), Some(96), Some(16), Some(186), Some(116), Some(61), Some(166),
    Some(3), Some(134), Some(76), Some(93), Some(180), Some(213), Some(234),
    Some(87), Some(187), Some(122), None, Some(201), Some(69), Some(53),
    Some(58), Some(172), Some(152), Some(50), Some(168), Some(150), Some(90),
    Some(133), Some(28), Some(101), Some(0), Some(97), Some(173), None,
    Some(13), None, Some(85), Some(5), Some(120), Some(219), Some(175),
    Some(208), Some(135), Some(242), Some(66), Some(217), Some(11), Some(178),
    Some(229), Some(126), Some(38), Some(44), Some(57), Some(136), Some(159),
    Some(164), Some(55), Some(249), Some(235), Some(27), Some(105), None,
    Some(203), Some(170), Some(102), Some(46), Some(73), Some(1), Some(17),
    Some(20), Some(19), None, Some(143), Some(52), Some(111), Some(231),
    Some(63), Some(139), Some(114), Some(167), Some(211), Some(163), Some(84),
    Some(141), Some(206), Some(107), None, Some(179), Some(78), Some(190),
    Some(35), Some(36), Some(79), Some(202), Some(54), Some(108), Some(199),
    Some(169), Some(29), Some(109), Some(25), Some(182), Some(241), Some(212),
    Some(12), Some(98), Some(138), Some(160), None, Some(156), Some(225),
    Some(177), Some(74), Some(147)];

static RIGHT_CLARITY : [Option<u8>; 256] = [
    Some(53), Some(151), None, Some(87), Some(255), Some(244), Some(152),
    Some(204), Some(180), Some(234), Some(169), Some(227), Some(26), Some(41),
    None, Some(198), Some(210), Some(38), Some(31), Some(110), Some(102), None,
    Some(243), Some(96), Some(45), None, Some(184), Some(126), Some(253),
    Some(183), Some(142), Some(208), Some(190), Some(88), Some(44),
    Some(148), Some(132), Some(231), Some(40), Some(15), Some(33), Some(172),
    Some(54), Some(177), Some(0), Some(215), Some(103), Some(80), Some(62),
    Some(76), Some(131), Some(64), Some(42), Some(146), Some(230), Some(122),
    Some(238), Some(217), Some(70), Some(49), Some(128), Some(221), Some(71),
    Some(14), Some(25), Some(175), Some(94), Some(18), Some(140), Some(193),
    Some(30), Some(105), Some(189), Some(165), Some(160), Some(173), Some(7),
    Some(228), Some(16), Some(89), Some(13), Some(47), Some(27), Some(246),
    Some(118), Some(106), Some(158), Some(150), Some(157), Some(11), Some(32),
    Some(236), Some(201), Some(145), Some(52), Some(72), Some(226), Some(57),
    Some(170), Some(197), None, Some(216), Some(43), Some(83), None, Some(147),
    Some(164), Some(124), Some(211), Some(67), Some(20), Some(46), Some(166),
    Some(149), Some(2), None, Some(219), Some(65), Some(182), Some(205),
    Some(84), Some(225), Some(114), Some(23), Some(24), Some(139), Some(188),
    None, Some(101), Some(112), Some(135), Some(9), Some(220), Some(79),
    Some(28), None, Some(6), Some(4), Some(162), None, Some(161), Some(133),
    Some(168), Some(56), None, Some(120), Some(81), Some(194), Some(115),
    Some(39), Some(59), Some(1), Some(252), Some(22), Some(155), Some(48),
    None, Some(73), Some(239), Some(77), Some(104), Some(69), Some(250),
    Some(99), Some(156), Some(85), Some(5), Some(121), Some(34), Some(75),
    Some(78), Some(86), Some(200), Some(91), Some(17), Some(82), Some(199),
    Some(127), Some(218), Some(60), Some(136), Some(117), Some(113), Some(178),
    Some(185), Some(90), Some(58), Some(35), Some(232), Some(223), Some(245),
    Some(119), Some(240), Some(29), Some(248), Some(196), Some(107), Some(167),
    Some(249), Some(159), Some(68), Some(125), Some(12), Some(36), Some(187),
    Some(3), Some(179), Some(235), Some(21), Some(61), Some(242), Some(213),
    Some(143), Some(19), Some(163), Some(97), Some(100), Some(130), Some(50),
    Some(138), Some(63), Some(214), Some(195), Some(203), Some(233), None,
    Some(237), Some(186), Some(153), Some(109), Some(191), Some(206), Some(8),
    Some(66), Some(144), Some(10), Some(51), None, Some(98), Some(241),
    Some(37), Some(123), Some(134), Some(93), Some(222), Some(55), None,
    Some(247), Some(192), Some(137), None, Some(141), Some(95), Some(74),
    Some(92), Some(108)];

pub fn enlighten(input : &[u8; 32]) -> Option<[u8; 32]>
{
    let mut result = [0u8; 32];
    for i in 0 .. 16 {
        if let Some(right_index) = RIGHT_CLARITY[input[i] as usize] {
            result[2 * i as usize] = LEFT_CLARITY[0].unwrap();
            result[2 * i + 1 as usize] = right_index;
        }
        else if let Some(left_index) = LEFT_CLARITY[input[i] as usize] {
            result[2 * i as usize] = left_index;
            result[2 * i + 1 as usize] = RIGHT_CLARITY[0].unwrap();
        }
        else {
            // FIXME: try to find a combination of indices which gives
            // input[i]
            return None;
        }
    }
    Some(result)
}

fn get_pairs(val : u8) -> Vec<(u8, u8)>
{
    let mut result = vec![];

    for lindex in 0 .. 256 {
        let lval = CONFUSION[lindex];
        for rindex in 0 .. 256 {
            let rval = CONFUSION[rindex + 256];
            if lval^rval == val {
                result.push((lindex as u8, rindex as u8))
            }
        }
    }

    result
}

pub fn enlighten_iter(input : &[u8; 32]) -> impl Iterator<Item = [u8; 32]> {
    input[..16].iter()
        .map(|val| get_pairs(*val))
        .multi_cartesian_product()
        .map(|pairs| {
            let mut result = [0u8; 32];
            for i in 0 .. 16 {
                let (lindex, rindex) = pairs[i];
                result[2 * i as usize] = lindex;
                result[2 * i + 1 as usize] = rindex;    
            }
            result
        })
}


#[cfg(test)]
mod tests {
    fn next_input() -> u8
    {
        let mut val : u8 = rand::random::<u8>();
        while super::LEFT_CLARITY[val as usize].is_none() && super::RIGHT_CLARITY[val as usize].is_none() {
            val = rand::random::<u8>()
        }
        val
    } 

    #[test]
    fn test_enlighten() {
        for _ in 0 .. 50
        {
            let mut input = [0u8; 32];
            for i in 0 .. 32 {
                input[i] = next_input();
            }
    
            if let Some(inverse) = super::enlighten(&input) {
                let actual = crate::forward::confuse_alot(&inverse);
                assert_eq!(input[..16], actual[..16]);
            }
            else {
                panic!("Couldn't compute inverse");
            }
        }
    }

    #[test]
    fn test_enlighten_iter() {
        for _ in 0 .. 50
        {
            let mut input = [0u8; 32];
            for i in 0 .. 32 {
                input[i] = next_input();
            }
    
            for inverse in super::enlighten_iter(&input).take(50) {
                let actual = crate::forward::confuse_alot(&inverse);
                assert_eq!(input[..16], actual[..16]);
            }
        }
    }
}