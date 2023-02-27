% author: dominik.rusovac@tu-dresden.de
% generated on 2023-02-27
#const n_tests=251.
#const n_modules=61.
test(50,0).
test(50,1).
test(50,2).
test(50,3).
test(50,4).
test(50,5).
test(50,6).
test(50,7).
test(50,8).
test(50,9).
test(50,10).
test(50,11).
test(50,12).
test(53,13).
test(53,14).
test(53,15).
test(53,16).
test(53,17).
test(53,18).
test(53,19).
test(53,20).
test(53,21).
test(53,22).
test(53,23).
test(53,24).
test(53,25).
test(53,26).
test(53,27).
test(53,28).
test(53,29).
test(53,30).
test(53,31).
test(53,32).
test(54,33).
test(54,34).
test(54,35).
test(54,36).
test(54,37).
test(0,38).
test(0,39).
test(0,40).
test(0,41).
test(0,42).
test(0,43).
test(0,44).
test(0,45).
test(0,46).
test(0,47).
test(0,48).
test(0,49).
test(0,50).
test(0,51).
test(7,52).
test(7,53).
test(7,54).
test(7,55).
test(7,56).
test(7,57).
test(7,58).
test(7,59).
test(7,60).
test(7,61).
test(7,62).
test(7,63).
test(7,64).
test(7,65).
test(7,66).
test(42,67).
test(10,68).
test(10,69).
test(57,70).
test(14,71).
test(32,72).
test(49,73).
test(58,74).
test(58,75).
test(58,76).
test(58,77).
test(58,78).
test(58,79).
test(58,80).
test(58,81).
test(58,82).
test(58,83).
test(58,84).
test(58,85).
test(58,86).
test(58,87).
test(58,88).
test(58,89).
test(58,90).
test(58,91).
test(16,92).
test(16,93).
test(16,94).
test(16,95).
test(16,96).
test(16,97).
test(16,98).
test(16,99).
test(16,100).
test(16,101).
test(16,102).
test(16,103).
test(16,104).
test(16,105).
test(16,106).
test(16,107).
test(16,108).
test(16,109).
test(16,110).
test(16,111).
test(16,112).
test(16,113).
test(16,114).
test(16,115).
test(16,116).
test(23,117).
test(23,118).
test(23,119).
test(23,120).
test(23,121).
test(23,122).
test(23,123).
test(23,124).
test(23,125).
test(23,126).
test(23,127).
test(9,128).
test(9,129).
test(9,130).
test(9,131).
test(9,132).
test(9,133).
test(9,134).
test(9,135).
test(9,136).
test(9,137).
test(9,138).
test(9,139).
test(9,140).
test(9,141).
test(9,142).
test(9,143).
test(43,144).
test(43,145).
test(43,146).
test(43,147).
test(43,148).
test(43,149).
test(43,150).
test(43,151).
test(43,152).
test(43,153).
test(43,154).
test(43,155).
test(43,156).
test(43,157).
test(43,158).
test(43,159).
test(43,160).
test(43,161).
test(43,162).
test(43,163).
test(43,164).
test(43,165).
test(43,166).
test(43,167).
test(43,168).
test(43,169).
test(43,170).
test(43,171).
test(43,172).
test(43,173).
test(43,174).
test(43,175).
test(43,176).
test(43,177).
test(43,178).
test(43,179).
test(43,180).
test(43,181).
test(43,182).
test(43,183).
test(43,184).
test(43,185).
test(43,186).
test(43,187).
test(43,188).
test(15,189).
test(20,190).
test(20,191).
test(20,192).
test(20,193).
test(20,194).
test(20,195).
test(20,196).
test(20,197).
test(20,198).
test(20,199).
test(20,200).
test(20,201).
test(20,202).
test(20,203).
test(20,204).
test(20,205).
test(20,206).
test(20,207).
test(51,208).
test(30,209).
test(52,210).
test(19,211).
test(47,212).
test(8,213).
test(45,214).
test(13,215).
test(36,216).
test(31,217).
test(26,218).
test(48,219).
test(6,220).
test(55,221).
test(34,222).
test(35,223).
test(28,224).
test(1,225).
test(2,226).
test(18,227).
test(25,228).
test(5,229).
test(56,230).
test(22,231).
test(21,232).
test(12,233).
test(29,234).
test(44,235).
test(11,236).
test(59,237).
test(3,238).
test(40,239).
test(27,240).
test(41,241).
test(46,242).
test(4,243).
test(17,244).
test(33,245).
test(24,246).
test(37,247).
test(38,248).
test(39,249).
test(60,250).

1 { run_module(M) : M=0..n_modules-1 }.                         
1 { run_test(N) : test(M,N), run_module(M) }.
                    
:- run_test(241), run_test(95).
:- run_test(123), run_test(113).
:- run_test(174), run_test(86).
:- run_test(244), run_test(11).
:- run_test(99), run_test(130).
:- run_test(58), run_test(250).
:- run_test(122), run_test(237).
:- run_test(43), run_test(97).
:- run_test(207), run_test(189).
:- run_test(114), run_test(204).
:- run_test(216), run_test(46).
:- run_test(109), run_test(19).
:- run_test(53), run_test(202).
:- run_test(188), run_test(7).
:- run_test(179), run_test(85).
:- run_test(112), run_test(151).
:- run_test(62), run_test(219).
:- run_test(198), run_test(93).
:- run_test(23), run_test(243).
:- run_test(0), run_test(118).
:- run_test(195), run_test(178).
:- run_test(83), run_test(18).
:- run_test(138), run_test(146).
:- run_test(14), run_test(133).
:- run_test(12), run_test(158).
:- run_test(66), not run_test(118).
:- not run_test(66), run_test(118).
:- run_test(68), not run_test(46).
:- not run_test(68), run_test(46).
:- run_test(197), not run_test(191).
:- not run_test(197), run_test(191).
:- run_test(148), not run_test(200).
:- not run_test(148), run_test(200).
:- run_test(58), not run_test(33).
:- not run_test(58), run_test(33).
:- run_test(90), not run_test(249).
:- not run_test(90), run_test(249).
:- run_test(155), not run_test(115).
:- not run_test(155), run_test(115).
:- run_test(130), not run_test(185).
:- not run_test(130), run_test(185).
:- run_test(82), not run_test(38).
:- not run_test(82), run_test(38).
:- run_test(176), not run_test(107).
:- not run_test(176), run_test(107).
:- run_test(183), not run_test(60).
:- not run_test(183), run_test(60).
:- run_test(137), not run_test(53).
:- not run_test(137), run_test(53).
:- run_test(250), not run_test(45).
:- not run_test(250), run_test(45).
:- run_test(104), not run_test(89).
:- not run_test(104), run_test(89).
:- run_test(35), not run_test(202).
:- not run_test(35), run_test(202).
:- run_test(110), not run_test(23).
:- not run_test(110), run_test(23).
:- run_test(80), not run_test(47).
:- not run_test(80), run_test(47).
:- run_test(77), not run_test(172).
:- not run_test(77), run_test(172).
:- run_test(113), not run_test(37).
:- not run_test(113), run_test(37).
:- run_test(83), not run_test(133).
:- not run_test(83), run_test(133).
:- run_test(134), not run_test(145).
:- not run_test(134), run_test(145).
:- run_test(235), not run_test(244).
:- not run_test(235), run_test(244).
:- run_test(247), not run_test(9).
:- not run_test(247), run_test(9).
:- run_test(119), not run_test(181).
:- not run_test(119), run_test(181).
:- run_test(138), not run_test(14).
:- not run_test(138), run_test(14).
:- run_module(6), run_module(55).
:- run_module(23), run_module(47).
:- run_module(19), run_module(2).
:- run_module(39), run_module(3).
:- run_module(25), run_module(37).
:- run_module(27), run_module(7).
:- run_module(29), not run_module(9).
:- run_module(39), not run_module(8).
:- run_module(34), not run_module(22).
:- run_module(3), not run_module(38).
:- run_module(5), not run_module(41).
:- run_module(33), not run_module(43).
#show run_module/1.
#show run_test/1.