% author: dominik.rusovac@tu-dresden.de
% generated on 2023-02-27
#const n_tests=251.
#const n_modules=61.
test(8,0).
test(8,1).
test(8,2).
test(8,3).
test(8,4).
test(8,5).
test(8,6).
test(8,7).
test(8,8).
test(8,9).
test(8,10).
test(8,11).
test(8,12).
test(15,13).
test(15,14).
test(15,15).
test(15,16).
test(15,17).
test(15,18).
test(15,19).
test(15,20).
test(15,21).
test(15,22).
test(15,23).
test(15,24).
test(15,25).
test(15,26).
test(15,27).
test(15,28).
test(15,29).
test(15,30).
test(15,31).
test(15,32).
test(17,33).
test(17,34).
test(17,35).
test(17,36).
test(17,37).
test(4,38).
test(4,39).
test(4,40).
test(4,41).
test(4,42).
test(4,43).
test(4,44).
test(4,45).
test(4,46).
test(4,47).
test(4,48).
test(4,49).
test(4,50).
test(4,51).
test(50,52).
test(50,53).
test(50,54).
test(50,55).
test(50,56).
test(50,57).
test(50,58).
test(50,59).
test(50,60).
test(50,61).
test(50,62).
test(50,63).
test(50,64).
test(50,65).
test(50,66).
test(27,67).
test(16,68).
test(16,69).
test(13,70).
test(37,71).
test(6,72).
test(42,73).
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
test(54,92).
test(54,93).
test(54,94).
test(54,95).
test(54,96).
test(54,97).
test(54,98).
test(54,99).
test(54,100).
test(54,101).
test(54,102).
test(54,103).
test(54,104).
test(54,105).
test(54,106).
test(54,107).
test(54,108).
test(54,109).
test(54,110).
test(54,111).
test(54,112).
test(54,113).
test(54,114).
test(54,115).
test(54,116).
test(24,117).
test(24,118).
test(24,119).
test(24,120).
test(24,121).
test(24,122).
test(24,123).
test(24,124).
test(24,125).
test(24,126).
test(24,127).
test(60,128).
test(60,129).
test(60,130).
test(60,131).
test(60,132).
test(60,133).
test(60,134).
test(60,135).
test(60,136).
test(60,137).
test(60,138).
test(60,139).
test(60,140).
test(60,141).
test(60,142).
test(60,143).
test(49,144).
test(49,145).
test(49,146).
test(49,147).
test(49,148).
test(49,149).
test(49,150).
test(49,151).
test(49,152).
test(49,153).
test(49,154).
test(49,155).
test(49,156).
test(49,157).
test(49,158).
test(49,159).
test(49,160).
test(49,161).
test(49,162).
test(49,163).
test(49,164).
test(49,165).
test(49,166).
test(49,167).
test(49,168).
test(49,169).
test(49,170).
test(49,171).
test(49,172).
test(49,173).
test(49,174).
test(49,175).
test(49,176).
test(49,177).
test(49,178).
test(49,179).
test(49,180).
test(49,181).
test(49,182).
test(49,183).
test(49,184).
test(49,185).
test(49,186).
test(49,187).
test(49,188).
test(43,189).
test(41,190).
test(41,191).
test(41,192).
test(41,193).
test(41,194).
test(41,195).
test(41,196).
test(41,197).
test(41,198).
test(41,199).
test(41,200).
test(41,201).
test(41,202).
test(41,203).
test(41,204).
test(41,205).
test(41,206).
test(41,207).
test(56,208).
test(9,209).
test(52,210).
test(30,211).
test(45,212).
test(33,213).
test(18,214).
test(12,215).
test(2,216).
test(38,217).
test(44,218).
test(25,219).
test(34,220).
test(28,221).
test(26,222).
test(0,223).
test(39,224).
test(40,225).
test(23,226).
test(59,227).
test(3,228).
test(7,229).
test(21,230).
test(20,231).
test(32,232).
test(53,233).
test(46,234).
test(31,235).
test(5,236).
test(1,237).
test(22,238).
test(55,239).
test(10,240).
test(35,241).
test(48,242).
test(11,243).
test(57,244).
test(51,245).
test(19,246).
test(14,247).
test(47,248).
test(58,249).
test(36,250).

1 { run_module(M) : M=0..n_modules-1 }.                         
1 { run_test(N) : test(M,N), run_module(M) }.
                    
:- run_test(200), run_test(26).
:- run_test(157), run_test(19).
:- run_test(148), run_test(108).
:- run_test(144), run_test(223).
:- run_test(129), run_test(3).
:- run_test(121), run_test(164).
:- run_test(99), run_test(57).
:- run_test(126), run_test(227).
:- run_test(13), run_test(244).
:- run_test(16), run_test(137).
:- run_test(216), run_test(160).
:- run_test(49), run_test(30).
:- run_test(147), run_test(199).
:- run_test(1), run_test(132).
:- run_test(206), run_test(15).
:- run_test(226), run_test(248).
:- run_test(174), run_test(163).
:- run_test(72), run_test(155).
:- run_test(50), run_test(105).
:- run_test(97), run_test(232).
:- run_test(88), run_test(162).
:- run_test(103), run_test(233).
:- run_test(45), run_test(27).
:- run_test(106), run_test(94).
:- run_test(112), run_test(76).
:- run_test(116), run_test(134).
:- run_test(12), run_test(192).
:- run_test(231), run_test(246).
:- run_test(6), run_test(22).
:- run_test(117), run_test(48).
:- run_test(84), run_test(83).
:- run_test(92), run_test(107).
:- run_test(191), run_test(10).
:- run_test(224), run_test(193).
:- run_test(188), run_test(53).
:- run_test(229), run_test(41).
:- run_test(44), run_test(28).
:- run_test(8), run_test(180).
:- run_test(73), run_test(39).
:- run_test(146), run_test(241).
:- run_test(115), run_test(238).
:- run_test(64), run_test(168).
:- run_test(245), run_test(130).
:- run_test(165), run_test(139).
:- run_test(142), run_test(23).
:- run_test(59), run_test(133).
:- run_test(171), run_test(159).
:- run_test(109), run_test(38).
:- run_test(35), run_test(93).
:- run_test(175), run_test(225).
:- run_test(45), not run_test(34).
:- not run_test(45), run_test(34).
:- run_test(214), not run_test(174).
:- not run_test(214), run_test(174).
:- run_test(227), not run_test(95).
:- not run_test(227), run_test(95).
:- run_test(91), not run_test(156).
:- not run_test(91), run_test(156).
:- run_test(102), not run_test(31).
:- not run_test(102), run_test(31).
:- run_test(232), not run_test(36).
:- not run_test(232), run_test(36).
:- run_test(47), not run_test(178).
:- not run_test(47), run_test(178).
:- run_test(87), not run_test(189).
:- not run_test(87), run_test(189).
:- run_test(192), not run_test(201).
:- not run_test(192), run_test(201).
:- run_test(103), not run_test(197).
:- not run_test(103), run_test(197).
:- run_test(210), not run_test(165).
:- not run_test(210), run_test(165).
:- run_test(33), not run_test(220).
:- not run_test(33), run_test(220).
:- run_test(68), not run_test(236).
:- not run_test(68), run_test(236).
:- run_test(187), not run_test(195).
:- not run_test(187), run_test(195).
:- run_test(222), not run_test(223).
:- not run_test(222), run_test(223).
:- run_test(71), not run_test(5).
:- not run_test(71), run_test(5).
:- run_test(52), not run_test(234).
:- not run_test(52), run_test(234).
:- run_test(0), not run_test(98).
:- not run_test(0), run_test(98).
:- run_test(42), not run_test(170).
:- not run_test(42), run_test(170).
:- run_test(17), not run_test(228).
:- not run_test(17), run_test(228).
:- run_test(60), not run_test(204).
:- not run_test(60), run_test(204).
:- run_test(134), not run_test(80).
:- not run_test(134), run_test(80).
:- run_test(66), not run_test(231).
:- not run_test(66), run_test(231).
:- run_test(6), not run_test(135).
:- not run_test(6), run_test(135).
:- run_test(40), not run_test(23).
:- not run_test(40), run_test(23).
:- run_test(77), not run_test(108).
:- not run_test(77), run_test(108).
:- run_test(241), not run_test(38).
:- not run_test(241), run_test(38).
:- run_test(92), not run_test(196).
:- not run_test(92), run_test(196).
:- run_test(119), not run_test(63).
:- not run_test(119), run_test(63).
:- run_test(49), not run_test(43).
:- not run_test(49), run_test(43).
:- run_test(152), not run_test(190).
:- not run_test(152), run_test(190).
:- run_test(164), not run_test(140).
:- not run_test(164), run_test(140).
:- run_test(163), not run_test(86).
:- not run_test(163), run_test(86).
:- run_test(56), not run_test(65).
:- not run_test(56), run_test(65).
:- run_test(139), not run_test(51).
:- not run_test(139), run_test(51).
:- run_test(213), not run_test(202).
:- not run_test(213), run_test(202).
:- run_test(29), not run_test(146).
:- not run_test(29), run_test(146).
:- run_test(81), not run_test(67).
:- not run_test(81), run_test(67).
:- run_test(229), not run_test(225).
:- not run_test(229), run_test(225).
:- run_test(24), not run_test(104).
:- not run_test(24), run_test(104).
:- run_test(106), not run_test(180).
:- not run_test(106), run_test(180).
:- run_test(79), not run_test(233).
:- not run_test(79), run_test(233).
:- run_test(169), not run_test(182).
:- not run_test(169), run_test(182).
:- run_test(124), not run_test(176).
:- not run_test(124), run_test(176).
:- run_test(84), not run_test(249).
:- not run_test(84), run_test(249).
:- run_test(101), not run_test(72).
:- not run_test(101), run_test(72).
:- run_test(53), not run_test(54).
:- not run_test(53), run_test(54).
:- run_test(35), not run_test(26).
:- not run_test(35), run_test(26).
:- run_test(27), not run_test(9).
:- not run_test(27), run_test(9).
:- run_test(4), not run_test(154).
:- not run_test(4), run_test(154).
:- run_module(46), run_module(20).
:- run_module(4), run_module(31).
:- run_module(10), run_module(49).
:- run_module(8), run_module(42).
:- run_module(15), run_module(44).
:- run_module(21), run_module(23).
:- run_module(48), run_module(6).
:- run_module(53), run_module(37).
:- run_module(43), run_module(18).
:- run_module(56), run_module(7).
:- run_module(36), run_module(38).
:- run_module(52), run_module(13).
:- run_module(37), not run_module(10).
:- run_module(3), not run_module(22).
:- run_module(39), not run_module(26).
:- run_module(14), not run_module(18).
:- run_module(59), not run_module(56).
:- run_module(43), not run_module(13).
:- run_module(45), not run_module(54).
:- run_module(21), not run_module(9).
:- run_module(4), not run_module(27).
:- run_module(33), not run_module(24).
:- run_module(53), not run_module(8).
:- run_module(48), not run_module(23).
#show run_module/1.
#show run_test/1.