% author: dominik.rusovac@tu-dresden.de
% generated on 2023-02-28
#const n_tests=251.
#const n_modules=61.
test(5,0).
test(5,1).
test(5,2).
test(5,3).
test(5,4).
test(5,5).
test(5,6).
test(5,7).
test(5,8).
test(5,9).
test(5,10).
test(5,11).
test(5,12).
test(49,13).
test(49,14).
test(49,15).
test(49,16).
test(49,17).
test(49,18).
test(49,19).
test(49,20).
test(49,21).
test(49,22).
test(49,23).
test(49,24).
test(49,25).
test(49,26).
test(49,27).
test(49,28).
test(49,29).
test(49,30).
test(49,31).
test(49,32).
test(43,33).
test(43,34).
test(43,35).
test(43,36).
test(43,37).
test(6,38).
test(6,39).
test(6,40).
test(6,41).
test(6,42).
test(6,43).
test(6,44).
test(6,45).
test(6,46).
test(6,47).
test(6,48).
test(6,49).
test(6,50).
test(6,51).
test(12,52).
test(12,53).
test(12,54).
test(12,55).
test(12,56).
test(12,57).
test(12,58).
test(12,59).
test(12,60).
test(12,61).
test(12,62).
test(12,63).
test(12,64).
test(12,65).
test(12,66).
test(29,67).
test(57,68).
test(57,69).
test(3,70).
test(32,71).
test(50,72).
test(40,73).
test(15,74).
test(15,75).
test(15,76).
test(15,77).
test(15,78).
test(15,79).
test(15,80).
test(15,81).
test(15,82).
test(15,83).
test(15,84).
test(15,85).
test(15,86).
test(15,87).
test(15,88).
test(15,89).
test(15,90).
test(15,91).
test(33,92).
test(33,93).
test(33,94).
test(33,95).
test(33,96).
test(33,97).
test(33,98).
test(33,99).
test(33,100).
test(33,101).
test(33,102).
test(33,103).
test(33,104).
test(33,105).
test(33,106).
test(33,107).
test(33,108).
test(33,109).
test(33,110).
test(33,111).
test(33,112).
test(33,113).
test(33,114).
test(33,115).
test(33,116).
test(34,117).
test(34,118).
test(34,119).
test(34,120).
test(34,121).
test(34,122).
test(34,123).
test(34,124).
test(34,125).
test(34,126).
test(34,127).
test(8,128).
test(8,129).
test(8,130).
test(8,131).
test(8,132).
test(8,133).
test(8,134).
test(8,135).
test(8,136).
test(8,137).
test(8,138).
test(8,139).
test(8,140).
test(8,141).
test(8,142).
test(8,143).
test(17,144).
test(17,145).
test(17,146).
test(17,147).
test(17,148).
test(17,149).
test(17,150).
test(17,151).
test(17,152).
test(17,153).
test(17,154).
test(17,155).
test(17,156).
test(17,157).
test(17,158).
test(17,159).
test(17,160).
test(17,161).
test(17,162).
test(17,163).
test(17,164).
test(17,165).
test(17,166).
test(17,167).
test(17,168).
test(17,169).
test(17,170).
test(17,171).
test(17,172).
test(17,173).
test(17,174).
test(17,175).
test(17,176).
test(17,177).
test(17,178).
test(17,179).
test(17,180).
test(17,181).
test(17,182).
test(17,183).
test(17,184).
test(17,185).
test(17,186).
test(17,187).
test(17,188).
test(10,189).
test(1,190).
test(1,191).
test(1,192).
test(1,193).
test(1,194).
test(1,195).
test(1,196).
test(1,197).
test(1,198).
test(1,199).
test(1,200).
test(1,201).
test(1,202).
test(1,203).
test(1,204).
test(1,205).
test(1,206).
test(1,207).
test(2,208).
test(16,209).
test(22,210).
test(27,211).
test(48,212).
test(19,213).
test(51,214).
test(13,215).
test(59,216).
test(35,217).
test(46,218).
test(52,219).
test(23,220).
test(41,221).
test(37,222).
test(44,223).
test(11,224).
test(42,225).
test(55,226).
test(47,227).
test(36,228).
test(7,229).
test(20,230).
test(60,231).
test(30,232).
test(21,233).
test(18,234).
test(14,235).
test(54,236).
test(39,237).
test(4,238).
test(31,239).
test(24,240).
test(45,241).
test(26,242).
test(53,243).
test(9,244).
test(58,245).
test(0,246).
test(25,247).
test(56,248).
test(38,249).
test(28,250).

1 { run_module(M) : M=0..n_modules-1 }.            % guess at least one module             
1 { run_test(N) : test(M,N) } :- run_module(M).    % guess at least one test per module
                    
:- run_test(108), run_test(245).
:- run_test(211), run_test(171).
:- run_test(233), run_test(18).
:- run_test(86), run_test(58).
:- run_test(209), run_test(143).
:- run_test(89), run_test(207).
:- run_test(188), run_test(78).
:- run_test(113), run_test(120).
:- run_test(24), run_test(73).
:- run_test(124), run_test(42).
:- run_test(140), run_test(247).
:- run_test(129), run_test(11).
:- run_test(17), not run_test(197).
:- not run_test(17), run_test(197).
:- run_test(192), not run_test(156).
:- not run_test(192), run_test(156).
:- run_test(98), not run_test(205).
:- not run_test(98), run_test(205).
:- run_test(82), not run_test(165).
:- not run_test(82), run_test(165).
:- run_test(207), not run_test(195).
:- not run_test(207), run_test(195).
:- run_test(116), not run_test(66).
:- not run_test(116), run_test(66).
:- run_test(170), not run_test(211).
:- not run_test(170), run_test(211).
:- run_test(190), not run_test(168).
:- not run_test(190), run_test(168).
:- run_test(36), not run_test(231).
:- not run_test(36), run_test(231).
:- run_test(113), not run_test(47).
:- not run_test(113), run_test(47).
:- run_test(217), not run_test(97).
:- not run_test(217), run_test(97).
:- run_test(124), not run_test(247).
:- not run_test(124), run_test(247).
:- run_module(4), run_module(30).
:- run_module(2), run_module(25).
:- run_module(28), run_module(49).
:- run_module(50), not run_module(19).
:- run_module(45), not run_module(7).
:- run_module(16), not run_module(30).
#show run_module/1.
#show run_test/1.

:- not run_test(250), not run_test(98).
