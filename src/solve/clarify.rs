static CLARITY : [Option<usize>; 256] = [
    Some(106),
    Some(26),
    Some(24),
    Some(89),
    Some(56),
    Some(86),
    Some(227),
    Some(195),
    Some(228),
    Some(95),
    Some(188),
    Some(144),
    Some(47),
    Some(223),
    Some(70),
    Some(454),
    Some(88),
    Some(294),
    Some(245),
    Some(42),
    Some(247),
    Some(191),
    Some(128),
    Some(10),
    Some(204),
    Some(221),
    Some(23),
    Some(246),
    Some(83),
    Some(131),
    Some(129),
    Some(4),
    Some(446),
    Some(130),
    Some(240),
    Some(145),
    Some(48),
    Some(2),
    Some(112),
    Some(158),
    Some(7),
    Some(253),
    Some(232),
    Some(9),
    Some(118),
    Some(132),
    Some(222),
    Some(176),
    Some(81),
    Some(251),
    Some(200),
    Some(6),
    Some(140),
    Some(115),
    Some(99),
    Some(210),
    Some(110),
    Some(68),
    Some(197),
    Some(216),
    Some(148),
    Some(218),
    Some(327),
    Some(230),
    Some(161),
    Some(162),
    Some(37),
    Some(117),
    Some(396),
    Some(248),
    Some(238),
    Some(100),
    Some(18),
    Some(77),
    Some(43),
    Some(33),
    Some(125),
    Some(67),
    Some(146),
    Some(183),
    Some(189),
    Some(92),
    Some(82),
    Some(39),
    Some(149),
    Some(14),
    Some(104),
    Some(184),
    Some(193),
    Some(75),
    Some(288),
    Some(215),
    Some(59),
    Some(15),
    Some(123),
    Some(198),
    Some(252),
    Some(121),
    Some(22),
    Some(40),
    Some(91),
    Some(239),
    Some(41),
    Some(214),
    Some(196),
    Some(80),
    Some(34),
    Some(380),
    Some(62),
    Some(155),
    Some(51),
    Some(244),
    Some(233),
    Some(31),
    Some(32),
    Some(255),
    Some(119),
    Some(321),
    Some(49),
    Some(127),
    Some(21),
    Some(157),
    Some(174),
    Some(192),
    Some(8),
    Some(236),
    Some(226),
    Some(103),
    Some(357),
    Some(60),
    Some(165),
    Some(30),
    Some(142),
    Some(45),
    Some(65),
    Some(250),
    Some(185),
    Some(153),
    Some(72),
    Some(205),
    Some(71),
    Some(137),
    Some(96),
    Some(16),
    Some(186),
    Some(116),
    Some(61),
    Some(166),
    Some(3),
    Some(134),
    Some(76),
    Some(93),
    Some(180),
    Some(213),
    Some(234),
    Some(87),
    Some(187),
    Some(122),
    Some(495),
    Some(201),
    Some(69),
    Some(53),
    Some(58),
    Some(172),
    Some(152),
    Some(50),
    Some(168),
    Some(150),
    Some(90),
    Some(133),
    Some(28),
    Some(101),
    Some(0),
    Some(97),
    Some(173),
    Some(338),
    Some(13),
    Some(383),
    Some(85),
    Some(5),
    Some(120),
    Some(219),
    Some(175),
    Some(208),
    Some(135),
    Some(242),
    Some(66),
    Some(217),
    Some(11),
    Some(178),
    Some(229),
    Some(126),
    Some(38),
    Some(44),
    Some(57),
    Some(136),
    Some(159),
    Some(164),
    Some(55),
    Some(249),
    Some(235),
    Some(27),
    Some(105),
    Some(292),
    Some(203),
    Some(170),
    Some(102),
    Some(46),
    Some(73),
    Some(1),
    Some(17),
    Some(20),
    Some(19),
    Some(275),
    Some(143),
    Some(52),
    Some(111),
    Some(231),
    Some(63),
    Some(139),
    Some(114),
    Some(167),
    Some(211),
    Some(163),
    Some(84),
    Some(141),
    Some(206),
    Some(107),
    Some(409),
    Some(179),
    Some(78),
    Some(190),
    Some(35),
    Some(36),
    Some(79),
    Some(202),
    Some(54),
    Some(108),
    Some(199),
    Some(169),
    Some(29),
    Some(109),
    Some(25),
    Some(182),
    Some(241),
    Some(212),
    Some(12),
    Some(98),
    Some(138),
    Some(160),
    None,
    Some(156),
    Some(225),
    Some(177),
    Some(74),
    Some(147)    
];

pub fn clarify(input: &[u8; 32]) -> Option<[usize; 32]> {
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
    fn next_input() -> u8
    {
        let mut val : u8 = rand::random::<u8>();
        while val == 250 {
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
}