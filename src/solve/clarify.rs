use lazy_static::lazy_static;

lazy_static! {
    static ref CLARITIES: [Vec<u8>; 256] = [
        vec![106], vec![26], vec![24, 94], vec![89], vec![56], vec![86], vec![227],
        vec![195], vec![228], vec![95], vec![188], vec![144], vec![47], vec![223],
        vec![70], vec![], vec![88], vec![], vec![245], vec![42], vec![247],
        vec![191],vec![128], vec![10], vec![204], vec![221], vec![23], vec![246],
        vec![83, 113], vec![131], vec![129], vec![4], vec![], vec![130], vec![240],
        vec![145], vec![48], vec![2], vec![112], vec![158], vec![7], vec![253],
        vec![232], vec![9], vec![118], vec![132, 220], vec![222], vec![176],
        vec![81], vec![251], vec![200], vec![6, 154], vec![140], vec![115],
        vec![99], vec![210], vec![110], vec![68], vec![197], vec![216], vec![148],
        vec![218], vec![], vec![230], vec![161], vec![162], vec![37], vec![117],
        vec![], vec![248], vec![238], vec![100], vec![18], vec![77, 209], vec![43],
        vec![33], vec![125], vec![67], vec![146], vec![183], vec![189], vec![92],
        vec![82], vec![39], vec![149], vec![14], vec![104], vec![184, 207],
        vec![193], vec![75], vec![], vec![215], vec![59], vec![15], vec![123],
        vec![198], vec![252], vec![121], vec![22], vec![40], vec![91], vec![239],
        vec![41, 181], vec![214], vec![196], vec![80], vec![34], vec![], vec![62],
        vec![155], vec![51], vec![244], vec![233], vec![31], vec![32], vec![255],
        vec![119], vec![], vec![49], vec![127], vec![21, 151], vec![157], vec![174],
        vec![192], vec![8], vec![236], vec![226], vec![103], vec![], vec![60],
        vec![165], vec![30], vec![142], vec![45], vec![65], vec![250], vec![185],
        vec![153], vec![72], vec![205], vec![71], vec![137, 171], vec![96],
        vec![16], vec![186], vec![116], vec![61], vec![166, 224], vec![3],
        vec![134], vec![76], vec![93], vec![180], vec![213], vec![234], vec![87],
        vec![187], vec![122], vec![], vec![201], vec![69], vec![53], vec![58, 243],
        vec![172], vec![152], vec![50], vec![168], vec![150], vec![90], vec![133],
        vec![28], vec![101], vec![0], vec![97], vec![173], vec![], vec![13], vec![],
        vec![85], vec![5], vec![120], vec![219], vec![175], vec![208], vec![135],
        vec![242], vec![66], vec![217], vec![11, 194], vec![178], vec![229],
        vec![126], vec![38], vec![44], vec![57], vec![136], vec![159],
        vec![164], vec![55, 64], vec![249], vec![235], vec![27], vec![105], vec![],
        vec![203], vec![170], vec![102], vec![46], vec![73], vec![1], vec![17],
        vec![20], vec![19], vec![], vec![143], vec![52], vec![111, 237], vec![231],
        vec![63], vec![139], vec![114], vec![167], vec![211], vec![163], vec![84],
        vec![141], vec![206], vec![107], vec![], vec![179], vec![78], vec![190],
        vec![35], vec![36, 124], vec![79], vec![202], vec![54], vec![108],
        vec![199], vec![169], vec![29], vec![109], vec![25], vec![182], vec![241],
        vec![212], vec![12], vec![98, 254], vec![138], vec![160], vec![], vec![156],
        vec![225], vec![177], vec![74], vec![147]
    ];
}

static MISSING_INDICES: [u8; 16] = [
    15, 17, 32, 62, 68, 90, 107, 117, 128, 158, 175, 177, 203, 213, 228, 250
];

static CLARITY : [Option<u8>; 256]  = [
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

// FIXME: clarify should apply deconvolute to the tentative output.
// If some element in the output is 250, it should use an alternative
// clarity for some of the input elements.
// it should only try to change the relevant elements
// (look at the diffusion matrix for this).

pub fn clarify(input: &[u8; 32]) -> Option<[u8; 32]> {
    let mut result = [0; 32];
    for index in 0 .. 32 {
        if let Some(val) = CLARITY[input[index] as usize] {
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
    use super::{clarify, MISSING_INDICES};

    fn next_input() -> u8
    {
        let mut val : u8 = rand::random::<u8>();
        while super::CLARITY[val as usize].is_none() {
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
            let input: [u8; 32] = rand::random();

            if input.iter().any(|x| MISSING_INDICES.contains(x)) {
                continue;
            }

            let clear = clarify(&input).unwrap();
            let next_input = deconvolute(&clear);

            assert!(next_input.iter().all(|x| !MISSING_INDICES.contains(x)));
        }
    }
}