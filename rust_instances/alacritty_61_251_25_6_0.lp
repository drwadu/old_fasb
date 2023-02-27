% author: dominik.rusovac@tu-dresden.de
% generated on 2023-02-27
#const n_tests=251.
#const n_modules=61.
test(3,0).
test(3,1).
test(3,2).
test(3,3).
test(3,4).
test(3,5).
test(3,6).
test(3,7).
test(3,8).
test(3,9).
test(3,10).
test(3,11).
test(3,12).
test(50,13).
test(50,14).
test(50,15).
test(50,16).
test(50,17).
test(50,18).
test(50,19).
test(50,20).
test(50,21).
test(50,22).
test(50,23).
test(50,24).
test(50,25).
test(50,26).
test(50,27).
test(50,28).
test(50,29).
test(50,30).
test(50,31).
test(50,32).
test(1,33).
test(1,34).
test(1,35).
test(1,36).
test(1,37).
test(43,38).
test(43,39).
test(43,40).
test(43,41).
test(43,42).
test(43,43).
test(43,44).
test(43,45).
test(43,46).
test(43,47).
test(43,48).
test(43,49).
test(43,50).
test(43,51).
test(23,52).
test(23,53).
test(23,54).
test(23,55).
test(23,56).
test(23,57).
test(23,58).
test(23,59).
test(23,60).
test(23,61).
test(23,62).
test(23,63).
test(23,64).
test(23,65).
test(23,66).
test(0,67).
test(12,68).
test(12,69).
test(8,70).
test(14,71).
test(45,72).
test(47,73).
test(46,74).
test(46,75).
test(46,76).
test(46,77).
test(46,78).
test(46,79).
test(46,80).
test(46,81).
test(46,82).
test(46,83).
test(46,84).
test(46,85).
test(46,86).
test(46,87).
test(46,88).
test(46,89).
test(46,90).
test(46,91).
test(10,92).
test(10,93).
test(10,94).
test(10,95).
test(10,96).
test(10,97).
test(10,98).
test(10,99).
test(10,100).
test(10,101).
test(10,102).
test(10,103).
test(10,104).
test(10,105).
test(10,106).
test(10,107).
test(10,108).
test(10,109).
test(10,110).
test(10,111).
test(10,112).
test(10,113).
test(10,114).
test(10,115).
test(10,116).
test(48,117).
test(48,118).
test(48,119).
test(48,120).
test(48,121).
test(48,122).
test(48,123).
test(48,124).
test(48,125).
test(48,126).
test(48,127).
test(49,128).
test(49,129).
test(49,130).
test(49,131).
test(49,132).
test(49,133).
test(49,134).
test(49,135).
test(49,136).
test(49,137).
test(49,138).
test(49,139).
test(49,140).
test(49,141).
test(49,142).
test(49,143).
test(2,144).
test(2,145).
test(2,146).
test(2,147).
test(2,148).
test(2,149).
test(2,150).
test(2,151).
test(2,152).
test(2,153).
test(2,154).
test(2,155).
test(2,156).
test(2,157).
test(2,158).
test(2,159).
test(2,160).
test(2,161).
test(2,162).
test(2,163).
test(2,164).
test(2,165).
test(2,166).
test(2,167).
test(2,168).
test(2,169).
test(2,170).
test(2,171).
test(2,172).
test(2,173).
test(2,174).
test(2,175).
test(2,176).
test(2,177).
test(2,178).
test(2,179).
test(2,180).
test(2,181).
test(2,182).
test(2,183).
test(2,184).
test(2,185).
test(2,186).
test(2,187).
test(2,188).
test(42,189).
test(29,190).
test(29,191).
test(29,192).
test(29,193).
test(29,194).
test(29,195).
test(29,196).
test(29,197).
test(29,198).
test(29,199).
test(29,200).
test(29,201).
test(29,202).
test(29,203).
test(29,204).
test(29,205).
test(29,206).
test(29,207).
test(54,208).
test(38,209).
test(36,210).
test(41,211).
test(56,212).
test(59,213).
test(11,214).
test(30,215).
test(55,216).
test(25,217).
test(27,218).
test(35,219).
test(44,220).
test(13,221).
test(60,222).
test(32,223).
test(51,224).
test(26,225).
test(16,226).
test(21,227).
test(4,228).
test(39,229).
test(57,230).
test(53,231).
test(15,232).
test(24,233).
test(5,234).
test(40,235).
test(31,236).
test(33,237).
test(17,238).
test(18,239).
test(9,240).
test(52,241).
test(58,242).
test(19,243).
test(6,244).
test(34,245).
test(20,246).
test(22,247).
test(7,248).
test(37,249).
test(28,250).

1 { run_module(M) : M=0..n_modules-1 }.                         
1 { run_test(N) : test(M,N), run_module(M) }.
                    
:- run_test(156), run_test(185).
:- run_test(67), run_test(17).
:- run_test(159), run_test(26).
:- run_test(142), run_test(240).
:- run_test(217), run_test(79).
:- run_test(11), run_test(153).
:- run_test(179), run_test(224).
:- run_test(208), run_test(97).
:- run_test(223), run_test(182).
:- run_test(113), run_test(116).
:- run_test(122), run_test(111).
:- run_test(210), run_test(48).
:- run_test(214), not run_test(52).
:- not run_test(214), run_test(52).
:- run_test(99), not run_test(250).
:- not run_test(99), run_test(250).
:- run_test(202), not run_test(138).
:- not run_test(202), run_test(138).
:- run_test(245), not run_test(36).
:- not run_test(245), run_test(36).
:- run_test(247), not run_test(23).
:- not run_test(247), run_test(23).
:- run_test(116), not run_test(213).
:- not run_test(116), run_test(213).
:- run_test(134), not run_test(234).
:- not run_test(134), run_test(234).
:- run_test(83), not run_test(89).
:- not run_test(83), run_test(89).
:- run_test(40), not run_test(4).
:- not run_test(40), run_test(4).
:- run_test(205), not run_test(185).
:- not run_test(205), run_test(185).
:- run_test(95), not run_test(154).
:- not run_test(95), run_test(154).
:- run_test(175), not run_test(65).
:- not run_test(175), run_test(65).
:- run_module(15), run_module(52).
:- run_module(36), run_module(27).
:- run_module(47), run_module(14).
:- run_module(41), not run_module(46).
:- run_module(27), not run_module(13).
:- run_module(0), not run_module(18).
#show run_module/1.
#show run_test/1.