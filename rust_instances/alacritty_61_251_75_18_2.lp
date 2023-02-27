% author: dominik.rusovac@tu-dresden.de
% generated on 2023-02-27
#const n_tests=251.
#const n_modules=61.
test(59,0).
test(59,1).
test(59,2).
test(59,3).
test(59,4).
test(59,5).
test(59,6).
test(59,7).
test(59,8).
test(59,9).
test(59,10).
test(59,11).
test(59,12).
test(7,13).
test(7,14).
test(7,15).
test(7,16).
test(7,17).
test(7,18).
test(7,19).
test(7,20).
test(7,21).
test(7,22).
test(7,23).
test(7,24).
test(7,25).
test(7,26).
test(7,27).
test(7,28).
test(7,29).
test(7,30).
test(7,31).
test(7,32).
test(52,33).
test(52,34).
test(52,35).
test(52,36).
test(52,37).
test(18,38).
test(18,39).
test(18,40).
test(18,41).
test(18,42).
test(18,43).
test(18,44).
test(18,45).
test(18,46).
test(18,47).
test(18,48).
test(18,49).
test(18,50).
test(18,51).
test(39,52).
test(39,53).
test(39,54).
test(39,55).
test(39,56).
test(39,57).
test(39,58).
test(39,59).
test(39,60).
test(39,61).
test(39,62).
test(39,63).
test(39,64).
test(39,65).
test(39,66).
test(44,67).
test(56,68).
test(56,69).
test(50,70).
test(57,71).
test(16,72).
test(13,73).
test(29,74).
test(29,75).
test(29,76).
test(29,77).
test(29,78).
test(29,79).
test(29,80).
test(29,81).
test(29,82).
test(29,83).
test(29,84).
test(29,85).
test(29,86).
test(29,87).
test(29,88).
test(29,89).
test(29,90).
test(29,91).
test(38,92).
test(38,93).
test(38,94).
test(38,95).
test(38,96).
test(38,97).
test(38,98).
test(38,99).
test(38,100).
test(38,101).
test(38,102).
test(38,103).
test(38,104).
test(38,105).
test(38,106).
test(38,107).
test(38,108).
test(38,109).
test(38,110).
test(38,111).
test(38,112).
test(38,113).
test(38,114).
test(38,115).
test(38,116).
test(4,117).
test(4,118).
test(4,119).
test(4,120).
test(4,121).
test(4,122).
test(4,123).
test(4,124).
test(4,125).
test(4,126).
test(4,127).
test(42,128).
test(42,129).
test(42,130).
test(42,131).
test(42,132).
test(42,133).
test(42,134).
test(42,135).
test(42,136).
test(42,137).
test(42,138).
test(42,139).
test(42,140).
test(42,141).
test(42,142).
test(42,143).
test(10,144).
test(10,145).
test(10,146).
test(10,147).
test(10,148).
test(10,149).
test(10,150).
test(10,151).
test(10,152).
test(10,153).
test(10,154).
test(10,155).
test(10,156).
test(10,157).
test(10,158).
test(10,159).
test(10,160).
test(10,161).
test(10,162).
test(10,163).
test(10,164).
test(10,165).
test(10,166).
test(10,167).
test(10,168).
test(10,169).
test(10,170).
test(10,171).
test(10,172).
test(10,173).
test(10,174).
test(10,175).
test(10,176).
test(10,177).
test(10,178).
test(10,179).
test(10,180).
test(10,181).
test(10,182).
test(10,183).
test(10,184).
test(10,185).
test(10,186).
test(10,187).
test(10,188).
test(60,189).
test(19,190).
test(19,191).
test(19,192).
test(19,193).
test(19,194).
test(19,195).
test(19,196).
test(19,197).
test(19,198).
test(19,199).
test(19,200).
test(19,201).
test(19,202).
test(19,203).
test(19,204).
test(19,205).
test(19,206).
test(19,207).
test(30,208).
test(36,209).
test(40,210).
test(14,211).
test(31,212).
test(53,213).
test(32,214).
test(25,215).
test(43,216).
test(8,217).
test(26,218).
test(34,219).
test(21,220).
test(45,221).
test(28,222).
test(5,223).
test(46,224).
test(2,225).
test(47,226).
test(0,227).
test(17,228).
test(33,229).
test(58,230).
test(11,231).
test(6,232).
test(24,233).
test(51,234).
test(35,235).
test(12,236).
test(55,237).
test(37,238).
test(9,239).
test(48,240).
test(1,241).
test(15,242).
test(23,243).
test(22,244).
test(3,245).
test(27,246).
test(41,247).
test(54,248).
test(49,249).
test(20,250).

1 { run_module(M) : M=0..n_modules-1 }.                         
1 { run_test(N) : test(M,N), run_module(M) }.
                    
:- run_test(227), run_test(104).
:- run_test(113), run_test(109).
:- run_test(107), run_test(148).
:- run_test(48), run_test(45).
:- run_test(250), run_test(122).
:- run_test(182), run_test(177).
:- run_test(65), run_test(209).
:- run_test(22), run_test(192).
:- run_test(17), run_test(120).
:- run_test(144), run_test(141).
:- run_test(166), run_test(232).
:- run_test(87), run_test(43).
:- run_test(49), run_test(62).
:- run_test(129), run_test(89).
:- run_test(98), run_test(184).
:- run_test(96), run_test(44).
:- run_test(213), run_test(186).
:- run_test(240), run_test(12).
:- run_test(178), run_test(72).
:- run_test(28), run_test(180).
:- run_test(174), run_test(91).
:- run_test(125), run_test(142).
:- run_test(219), run_test(7).
:- run_test(85), run_test(131).
:- run_test(199), run_test(0).
:- run_test(103), run_test(205).
:- run_test(143), run_test(33).
:- run_test(233), run_test(30).
:- run_test(176), run_test(225).
:- run_test(149), run_test(249).
:- run_test(185), run_test(169).
:- run_test(247), run_test(163).
:- run_test(2), run_test(123).
:- run_test(246), run_test(171).
:- run_test(5), run_test(82).
:- run_test(77), run_test(221).
:- run_test(78), run_test(234).
:- run_test(116), not run_test(17).
:- not run_test(116), run_test(17).
:- run_test(197), not run_test(1).
:- not run_test(197), run_test(1).
:- run_test(76), not run_test(159).
:- not run_test(76), run_test(159).
:- run_test(52), not run_test(206).
:- not run_test(52), run_test(206).
:- run_test(66), not run_test(68).
:- not run_test(66), run_test(68).
:- run_test(189), not run_test(141).
:- not run_test(189), run_test(141).
:- run_test(74), not run_test(47).
:- not run_test(74), run_test(47).
:- run_test(205), not run_test(77).
:- not run_test(205), run_test(77).
:- run_test(137), not run_test(157).
:- not run_test(137), run_test(157).
:- run_test(95), not run_test(70).
:- not run_test(95), run_test(70).
:- run_test(118), not run_test(217).
:- not run_test(118), run_test(217).
:- run_test(42), not run_test(200).
:- not run_test(42), run_test(200).
:- run_test(138), not run_test(202).
:- not run_test(138), run_test(202).
:- run_test(117), not run_test(88).
:- not run_test(117), run_test(88).
:- run_test(165), not run_test(85).
:- not run_test(165), run_test(85).
:- run_test(139), not run_test(79).
:- not run_test(139), run_test(79).
:- run_test(226), not run_test(247).
:- not run_test(226), run_test(247).
:- run_test(130), not run_test(172).
:- not run_test(130), run_test(172).
:- run_test(35), not run_test(41).
:- not run_test(35), run_test(41).
:- run_test(69), not run_test(148).
:- not run_test(69), run_test(148).
:- run_test(228), not run_test(49).
:- not run_test(228), run_test(49).
:- run_test(142), not run_test(10).
:- not run_test(142), run_test(10).
:- run_test(39), not run_test(132).
:- not run_test(39), run_test(132).
:- run_test(151), not run_test(127).
:- not run_test(151), run_test(127).
:- run_test(167), not run_test(64).
:- not run_test(167), run_test(64).
:- run_test(218), not run_test(101).
:- not run_test(218), run_test(101).
:- run_test(219), not run_test(40).
:- not run_test(219), run_test(40).
:- run_test(152), not run_test(233).
:- not run_test(152), run_test(233).
:- run_test(18), not run_test(153).
:- not run_test(18), run_test(153).
:- run_test(195), not run_test(6).
:- not run_test(195), run_test(6).
:- run_test(50), not run_test(143).
:- not run_test(50), run_test(143).
:- run_test(133), not run_test(215).
:- not run_test(133), run_test(215).
:- run_test(131), not run_test(22).
:- not run_test(131), run_test(22).
:- run_test(140), not run_test(59).
:- not run_test(140), run_test(59).
:- run_test(198), not run_test(91).
:- not run_test(198), run_test(91).
:- run_test(223), not run_test(184).
:- not run_test(223), run_test(184).
:- run_test(169), not run_test(145).
:- not run_test(169), run_test(145).
:- run_module(6), run_module(20).
:- run_module(7), run_module(30).
:- run_module(33), run_module(2).
:- run_module(59), run_module(44).
:- run_module(11), run_module(57).
:- run_module(34), run_module(29).
:- run_module(24), run_module(32).
:- run_module(17), run_module(48).
:- run_module(18), run_module(46).
:- run_module(44), not run_module(51).
:- run_module(32), not run_module(53).
:- run_module(12), not run_module(27).
:- run_module(52), not run_module(59).
:- run_module(3), not run_module(5).
:- run_module(54), not run_module(6).
:- run_module(35), not run_module(24).
:- run_module(23), not run_module(13).
:- run_module(56), not run_module(37).
#show run_module/1.
#show run_test/1.
:- run_test(142), run_test(226).
:- run_test(62), run_test(213).
:- run_test(219), run_test(198).
:- run_test(104), run_test(169).
:- run_test(105), run_test(161).
:- run_test(34), run_test(12).
:- run_test(111), run_test(247).
:- run_test(35), run_test(93).
:- run_test(44), run_test(149).
:- run_test(49), run_test(241).
:- run_test(135), run_test(48).
:- run_test(196), run_test(181).
:- run_test(41), run_test(11).
:- run_test(32), run_test(175).
:- run_test(244), run_test(27).
:- run_test(29), run_test(138).
:- run_test(39), run_test(222).
:- run_test(1), run_test(124).
:- run_test(33), run_test(89).
:- run_test(199), run_test(195).
:- run_test(55), run_test(88).
:- run_test(232), run_test(47).
:- run_test(127), run_test(248).
:- run_test(228), run_test(102).
:- run_test(200), run_test(67).
:- run_test(155), run_test(153).
:- run_test(148), run_test(140).
:- run_test(4), run_test(78).
:- run_test(72), run_test(172).
:- run_test(2), run_test(40).
:- run_test(69), run_test(227).
:- run_test(115), run_test(141).
:- run_test(205), run_test(28).
:- run_test(235), run_test(243).
:- run_test(160), run_test(70).
:- run_test(182), run_test(110).
:- run_test(168), run_test(45).
:- run_test(191), not run_test(102).
:- not run_test(191), run_test(102).
:- run_test(232), not run_test(219).
:- not run_test(232), run_test(219).
:- run_test(167), not run_test(52).
:- not run_test(167), run_test(52).
:- run_test(90), not run_test(248).
:- not run_test(90), run_test(248).
:- run_test(224), not run_test(109).
:- not run_test(224), run_test(109).
:- run_test(173), not run_test(170).
:- not run_test(173), run_test(170).
:- run_test(5), not run_test(65).
:- not run_test(5), run_test(65).
:- run_test(118), not run_test(60).
:- not run_test(118), run_test(60).
:- run_test(209), not run_test(26).
:- not run_test(209), run_test(26).
:- run_test(179), not run_test(148).
:- not run_test(179), run_test(148).
:- run_test(195), not run_test(211).
:- not run_test(195), run_test(211).
:- run_test(72), not run_test(145).
:- not run_test(72), run_test(145).
:- run_test(183), not run_test(243).
:- not run_test(183), run_test(243).
:- run_test(64), not run_test(105).
:- not run_test(64), run_test(105).
:- run_test(14), not run_test(66).
:- not run_test(14), run_test(66).
:- run_test(95), not run_test(53).
:- not run_test(95), run_test(53).
:- run_test(206), not run_test(250).
:- not run_test(206), run_test(250).
:- run_test(137), not run_test(185).
:- not run_test(137), run_test(185).
:- run_test(230), not run_test(169).
:- not run_test(230), run_test(169).
:- run_test(25), not run_test(15).
:- not run_test(25), run_test(15).
:- run_test(205), not run_test(149).
:- not run_test(205), run_test(149).
:- run_test(222), not run_test(49).
:- not run_test(222), run_test(49).
:- run_test(198), not run_test(138).
:- not run_test(198), run_test(138).
:- run_test(97), not run_test(176).
:- not run_test(97), run_test(176).
:- run_test(166), not run_test(249).
:- not run_test(166), run_test(249).
:- run_test(197), not run_test(164).
:- not run_test(197), run_test(164).
:- run_test(143), not run_test(153).
:- not run_test(143), run_test(153).
:- run_test(47), not run_test(133).
:- not run_test(47), run_test(133).
:- run_test(42), not run_test(57).
:- not run_test(42), run_test(57).
:- run_test(37), not run_test(20).
:- not run_test(37), run_test(20).
:- run_test(172), not run_test(91).
:- not run_test(172), run_test(91).
:- run_test(215), not run_test(174).
:- not run_test(215), run_test(174).
:- run_test(135), not run_test(13).
:- not run_test(135), run_test(13).
:- run_test(187), not run_test(16).
:- not run_test(187), run_test(16).
:- run_test(79), not run_test(130).
:- not run_test(79), run_test(130).
:- run_test(134), not run_test(123).
:- not run_test(134), run_test(123).
:- run_test(188), not run_test(144).
:- not run_test(188), run_test(144).
:- run_module(4), run_module(51).
:- run_module(18), run_module(59).
:- run_module(17), run_module(21).
:- run_module(20), run_module(60).
:- run_module(12), run_module(0).
:- run_module(32), run_module(26).
:- run_module(39), run_module(8).
:- run_module(46), run_module(2).
:- run_module(22), run_module(45).
:- run_module(25), not run_module(4).
:- run_module(34), not run_module(24).
:- run_module(22), not run_module(23).
:- run_module(20), not run_module(57).
:- run_module(32), not run_module(48).
:- run_module(45), not run_module(21).
:- run_module(35), not run_module(13).
:- run_module(27), not run_module(43).
:- run_module(49), not run_module(14).
#show run_module/1.
#show run_test/1.
:- run_test(212), run_test(108).
:- run_test(107), run_test(199).
:- run_test(133), run_test(213).
:- run_test(148), run_test(102).
:- run_test(122), run_test(200).
:- run_test(192), run_test(176).
:- run_test(106), run_test(61).
:- run_test(39), run_test(118).
:- run_test(134), run_test(115).
:- run_test(188), run_test(32).
:- run_test(87), run_test(167).
:- run_test(131), run_test(46).
:- run_test(50), run_test(154).
:- run_test(207), run_test(97).
:- run_test(24), run_test(94).
:- run_test(161), run_test(235).
:- run_test(42), run_test(169).
:- run_test(99), run_test(166).
:- run_test(52), run_test(150).
:- run_test(193), run_test(3).
:- run_test(178), run_test(6).
:- run_test(144), run_test(29).
:- run_test(74), run_test(248).
:- run_test(130), run_test(38).
:- run_test(41), run_test(43).
:- run_test(246), run_test(64).
:- run_test(218), run_test(206).
:- run_test(228), run_test(204).
:- run_test(82), run_test(4).
:- run_test(158), run_test(128).
:- run_test(0), run_test(187).
:- run_test(7), run_test(238).
:- run_test(175), run_test(59).
:- run_test(13), run_test(209).
:- run_test(48), run_test(21).
:- run_test(2), run_test(14).
:- run_test(44), run_test(151).
:- run_test(94), not run_test(192).
:- not run_test(94), run_test(192).
:- run_test(75), not run_test(2).
:- not run_test(75), run_test(2).
:- run_test(199), not run_test(83).
:- not run_test(199), run_test(83).
:- run_test(179), not run_test(133).
:- not run_test(179), run_test(133).
:- run_test(109), not run_test(167).
:- not run_test(109), run_test(167).
:- run_test(238), not run_test(160).
:- not run_test(238), run_test(160).
:- run_test(217), not run_test(171).
:- not run_test(217), run_test(171).
:- run_test(158), not run_test(208).
:- not run_test(158), run_test(208).
:- run_test(22), not run_test(32).
:- not run_test(22), run_test(32).
:- run_test(237), not run_test(87).
:- not run_test(237), run_test(87).
:- run_test(191), not run_test(229).
:- not run_test(191), run_test(229).
:- run_test(164), not run_test(91).
:- not run_test(164), run_test(91).
:- run_test(188), not run_test(107).
:- not run_test(188), run_test(107).
:- run_test(100), not run_test(200).
:- not run_test(100), run_test(200).
:- run_test(183), not run_test(184).
:- not run_test(183), run_test(184).
:- run_test(101), not run_test(136).
:- not run_test(101), run_test(136).
:- run_test(11), not run_test(219).
:- not run_test(11), run_test(219).
:- run_test(21), not run_test(57).
:- not run_test(21), run_test(57).
:- run_test(230), not run_test(23).
:- not run_test(230), run_test(23).
:- run_test(228), not run_test(225).
:- not run_test(228), run_test(225).
:- run_test(143), not run_test(6).
:- not run_test(143), run_test(6).
:- run_test(44), not run_test(74).
:- not run_test(44), run_test(74).
:- run_test(67), not run_test(82).
:- not run_test(67), run_test(82).
:- run_test(30), not run_test(210).
:- not run_test(30), run_test(210).
:- run_test(80), not run_test(95).
:- not run_test(80), run_test(95).
:- run_test(140), not run_test(104).
:- not run_test(140), run_test(104).
:- run_test(218), not run_test(250).
:- not run_test(218), run_test(250).
:- run_test(205), not run_test(56).
:- not run_test(205), run_test(56).
:- run_test(106), not run_test(111).
:- not run_test(106), run_test(111).
:- run_test(155), not run_test(70).
:- not run_test(155), run_test(70).
:- run_test(19), not run_test(68).
:- not run_test(19), run_test(68).
:- run_test(128), not run_test(149).
:- not run_test(128), run_test(149).
:- run_test(76), not run_test(249).
:- not run_test(76), run_test(249).
:- run_test(180), not run_test(172).
:- not run_test(180), run_test(172).
:- run_test(244), not run_test(37).
:- not run_test(244), run_test(37).
:- run_test(223), not run_test(166).
:- not run_test(223), run_test(166).
:- run_test(220), not run_test(157).
:- not run_test(220), run_test(157).
:- run_module(35), run_module(38).
:- run_module(42), run_module(28).
:- run_module(10), run_module(40).
:- run_module(49), run_module(26).
:- run_module(25), run_module(20).
:- run_module(32), run_module(31).
:- run_module(55), run_module(46).
:- run_module(58), run_module(7).
:- run_module(36), run_module(16).
:- run_module(58), not run_module(56).
:- run_module(25), not run_module(51).
:- run_module(24), not run_module(53).
:- run_module(22), not run_module(46).
:- run_module(47), not run_module(1).
:- run_module(6), not run_module(40).
:- run_module(17), not run_module(3).
:- run_module(37), not run_module(43).
:- run_module(39), not run_module(0).
#show run_module/1.
#show run_test/1.