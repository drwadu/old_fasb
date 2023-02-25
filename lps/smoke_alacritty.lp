% author: dominik.rusovac@tu-dresden.de
% generated on 2023-02-06
#const n_tests=252.
#const n_modules=62.
test(16,0).
test(16,1).
test(16,2).
test(16,3).
test(16,4).
test(16,5).
test(16,6).
test(16,7).
test(16,8).
test(16,9).
test(16,10).
test(16,11).
test(16,12).
test(23,13).
test(23,14).
test(23,15).
test(23,16).
test(23,17).
test(23,18).
test(23,19).
test(23,20).
test(23,21).
test(23,22).
test(23,23).
test(23,24).
test(23,25).
test(23,26).
test(23,27).
test(23,28).
test(23,29).
test(23,30).
test(23,31).
test(23,32).
test(56,33).
test(56,34).
test(56,35).
test(56,36).
test(56,37).
test(57,38).
test(57,39).
test(57,40).
test(57,41).
test(57,42).
test(57,43).
test(57,44).
test(57,45).
test(57,46).
test(57,47).
test(57,48).
test(57,49).
test(57,50).
test(57,51).
test(17,52).
test(17,53).
test(17,54).
test(17,55).
test(17,56).
test(17,57).
test(17,58).
test(17,59).
test(17,60).
test(17,61).
test(17,62).
test(17,63).
test(17,64).
test(17,65).
test(17,66).
test(54,67).
test(60,68).
test(60,69).
test(12,70).
test(4,71).
test(14,72).
test(36,73).
test(42,74).
test(42,75).
test(42,76).
test(42,77).
test(42,78).
test(42,79).
test(42,80).
test(42,81).
test(42,82).
test(42,83).
test(42,84).
test(42,85).
test(42,86).
test(42,87).
test(42,88).
test(42,89).
test(42,90).
test(42,91).
test(18,92).
test(18,93).
test(18,94).
test(18,95).
test(18,96).
test(18,97).
test(18,98).
test(18,99).
test(18,100).
test(18,101).
test(18,102).
test(18,103).
test(18,104).
test(18,105).
test(18,106).
test(18,107).
test(18,108).
test(18,109).
test(18,110).
test(18,111).
test(18,112).
test(18,113).
test(18,114).
test(18,115).
test(18,116).
test(38,117).
test(38,118).
test(38,119).
test(38,120).
test(38,121).
test(38,122).
test(38,123).
test(38,124).
test(38,125).
test(38,126).
test(38,127).
test(13,128).
test(13,129).
test(13,130).
test(13,131).
test(13,132).
test(13,133).
test(13,134).
test(13,135).
test(13,136).
test(13,137).
test(13,138).
test(13,139).
test(13,140).
test(13,141).
test(13,142).
test(13,143).
test(53,144).
test(53,145).
test(53,146).
test(53,147).
test(53,148).
test(53,149).
test(53,150).
test(53,151).
test(53,152).
test(53,153).
test(53,154).
test(53,155).
test(53,156).
test(53,157).
test(53,158).
test(53,159).
test(53,160).
test(53,161).
test(53,162).
test(53,163).
test(53,164).
test(53,165).
test(53,166).
test(53,167).
test(53,168).
test(53,169).
test(53,170).
test(53,171).
test(53,172).
test(53,173).
test(53,174).
test(53,175).
test(53,176).
test(53,177).
test(53,178).
test(53,179).
test(53,180).
test(53,181).
test(53,182).
test(53,183).
test(53,184).
test(53,185).
test(53,186).
test(53,187).
test(53,188).
test(32,189).
test(37,190).
test(37,191).
test(37,192).
test(37,193).
test(37,194).
test(37,195).
test(37,196).
test(37,197).
test(37,198).
test(37,199).
test(37,200).
test(37,201).
test(37,202).
test(37,203).
test(37,204).
test(37,205).
test(37,206).
test(37,207).
test(15,208).
test(1,209).
test(41,210).
test(11,211).
test(19,212).
test(27,213).
test(55,214).
test(45,215).
test(20,216).
test(51,217).
test(28,218).
test(25,219).
test(29,220).
test(33,221).
test(46,222).
test(34,223).
test(5,224).
test(50,225).
test(40,226).
test(59,227).
test(9,228).
test(35,229).
test(8,230).
test(43,231).
test(47,232).
test(21,233).
test(39,234).
test(26,235).
test(44,236).
test(3,237).
test(31,238).
test(10,239).
test(30,240).
test(52,241).
test(6,242).
test(24,243).
test(48,244).
test(58,245).
test(0,246).
test(2,247).
test(7,248).
test(22,249).
test(49,250).
1 { run_module(N) : N=0..n_modules }.
{ run_test(T,N) : test(M,N), T=0..K } :- run_module(M), K=#count {1:test(M,_)}.
#show run_module/1.
#show run_test/2.

:- run_test(T,N), run_test(T,N'), N!=N'. 

%*
test to integer mapping
cli::tests::completions 0
cli::tests::dynamic_title_ignoring_options_by_default 1
cli::tests::dynamic_title_not_overridden_by_config 2
cli::tests::dynamic_title_overridden_by_options 3
cli::tests::float_option_as_value 4
cli::tests::invalid_hex_to_decimal 5
cli::tests::invalid_option_as_value 6
cli::tests::parse_general_class 7
cli::tests::parse_instance_class 8
cli::tests::parse_invalid_class 9
cli::tests::valid_decimal 10
cli::tests::valid_hex_to_decimal 11
cli::tests::valid_option_as_value 12
config::bindings::tests::binding_matches_different_action 13
config::bindings::tests::binding_matches_identical_mode 14
config::bindings::tests::binding_matches_itself 15
config::bindings::tests::binding_matches_mode_notmode 16
config::bindings::tests::binding_matches_modes 17
config::bindings::tests::binding_matches_notmodes 18
config::bindings::tests::binding_matches_partial_intersection 19
config::bindings::tests::binding_mismatches_notmode 20
config::bindings::tests::binding_mismatches_unrelated 21
config::bindings::tests::binding_trigger_input 22
config::bindings::tests::binding_trigger_modes 23
config::bindings::tests::binding_trigger_mods 24
config::bindings::tests::binding_trigger_notmodes 25
config::bindings::tests::binding_with_mode_matches_empty_mode 26
config::bindings::tests::binding_without_mode_matches_any_mode 27
config::bindings::tests::mods_binding_requires_strict_match 28
config::serde_utils::tests::merge_mapping 29
config::serde_utils::tests::merge_primitive 30
config::serde_utils::tests::merge_sequence 31
config::tests::config_read_eof 32
display::damage::tests::damage_rect_math 33
display::hint::tests::closed_bracket_does_not_result_in_infinite_iterator 34
display::hint::tests::collect_unique_hyperlinks 35
display::hint::tests::hint_label_generation 36
display::hint::tests::visible_regex_match_covers_entire_viewport 37
input::tests::double_click 38
input::tests::multi_click_separate_buttons 39
input::tests::process_binding_appcursormode_appkeypadmode_nomod_require_appcursor 40
input::tests::process_binding_appcursormode_nomod_require_appcursor 41
input::tests::process_binding_fail_with_extra_mods 42
input::tests::process_binding_nomode_controlmod 43
input::tests::process_binding_nomode_nomod_require_appcursor 44
input::tests::process_binding_nomode_nomod_require_not_appcursor 45
input::tests::process_binding_nomode_nomod_require_shift 46
input::tests::process_binding_nomode_shiftmod_require_shift 47
input::tests::single_click 48
input::tests::single_middle_click 49
input::tests::single_right_click 50
input::tests::triple_click 51
message_bar::tests::add_newline_for_button 52
message_bar::tests::appends_close_button 53
message_bar::tests::empty_with_shortterm 54
message_bar::tests::hide_button_when_too_narrow 55
message_bar::tests::hide_truncated_when_too_narrow 56
message_bar::tests::multiline_close_button_first_line 57
message_bar::tests::pop 58
message_bar::tests::remove_duplicates 59
message_bar::tests::remove_target 60
message_bar::tests::splits_on_length 61
message_bar::tests::splits_on_newline 62
message_bar::tests::strip_whitespace_at_linebreak 63
message_bar::tests::truncates_long_messages 64
message_bar::tests::wrap_on_words 65
message_bar::tests::wrap_with_unicode 66
renderer::text::builtin_font::tests::builtin_line_drawing_glyphs_coverage 67
string::tests::into_shortened_with_shortener 68
string::tests::into_shortened_without_shortener 69
config_deserialize 70
field_replacement 71
replace_derive 72
replace_flatten 73
ansi::tests::parse_control_attribute 74
ansi::tests::parse_designate_g0_as_line_drawing 75
ansi::tests::parse_designate_g1_as_line_drawing_and_invoke 76
ansi::tests::parse_invalid_legacy_rgb_colors 77
ansi::tests::parse_invalid_number 78
ansi::tests::parse_invalid_rgb_colors 79
ansi::tests::parse_number_too_large 80
ansi::tests::parse_osc104_reset_all_colors 81
ansi::tests::parse_osc104_reset_all_colors_no_semicolon 82
ansi::tests::parse_osc104_reset_color 83
ansi::tests::parse_osc4_set_color 84
ansi::tests::parse_terminal_identity_csi 85
ansi::tests::parse_terminal_identity_esc 86
ansi::tests::parse_truecolor_attr 87
ansi::tests::parse_valid_legacy_rgb_colors 88
ansi::tests::parse_valid_number 89
ansi::tests::parse_valid_rgb_colors 90
ansi::tests::parse_zsh_startup 91
grid::storage::tests::grow_after_zero 92
grid::storage::tests::grow_before_zero 93
grid::storage::tests::indexing 94
grid::storage::tests::indexing_above_inner_len 95
grid::storage::tests::initialize 96
grid::storage::tests::rotate 97
grid::storage::tests::rotate_wrap_zero 98
grid::storage::tests::shrink_after_zero 99
grid::storage::tests::shrink_before_and_after_zero 100
grid::storage::tests::shrink_before_zero 101
grid::storage::tests::shrink_then_grow 102
grid::storage::tests::truncate_invisible_lines 103
grid::storage::tests::truncate_invisible_lines_beginning 104
grid::storage::tests::with_capacity 105
grid::tests::grow_reflow 106
grid::tests::grow_reflow_disabled 107
grid::tests::grow_reflow_multiline 108
grid::tests::scroll_down 109
grid::tests::scroll_down_with_history 110
grid::tests::scroll_up 111
grid::tests::shrink_reflow 112
grid::tests::shrink_reflow_disabled 113
grid::tests::shrink_reflow_empty_cell_inside_line 114
grid::tests::shrink_reflow_twice 115
grid::tests::test_iter 116
index::tests::add 117
index::tests::add_clamp 118
index::tests::add_grid_clamp 119
index::tests::add_none_clamp 120
index::tests::add_wrap 121
index::tests::location_ordering 122
index::tests::sub 123
index::tests::sub_clamp 124
index::tests::sub_grid_clamp 125
index::tests::sub_none_clamp 126
index::tests::sub_wrap 127
selection::tests::across_adjacent_lines_upward_final_cell_exclusive 128
selection::tests::between_adjacent_cells_left_to_right 129
selection::tests::between_adjacent_cells_right_to_left 130
selection::tests::block_is_empty 131
selection::tests::block_selection 132
selection::tests::line_selection 133
selection::tests::range_intersection 134
selection::tests::rotate_in_region_down 135
selection::tests::rotate_in_region_up 136
selection::tests::rotate_in_region_up_block 137
selection::tests::selection_bigger_then_smaller 138
selection::tests::semantic_selection 139
selection::tests::simple_is_empty 140
selection::tests::simple_selection 141
selection::tests::single_cell_left_to_right 142
selection::tests::single_cell_right_to_left 143
term::cell::tests::cell_size_is_below_cap 144
term::cell::tests::line_length_works 145
term::cell::tests::line_length_works_with_wrapline 146
term::color::tests::contrast 147
term::search::tests::end_on_fullwidth 148
term::search::tests::fullwidth 149
term::search::tests::include_linebreak_left 150
term::search::tests::include_linebreak_right 151
term::search::tests::leading_spacer 152
term::search::tests::multibyte_unicode 153
term::search::tests::nested_regex 154
term::search::tests::no_match_left 155
term::search::tests::no_match_right 156
term::search::tests::regex_left 157
term::search::tests::regex_right 158
term::search::tests::reverse_search_dead_recovery 159
term::search::tests::singlecell_fullwidth 160
term::search::tests::skip_dead_cell 161
term::search::tests::wide_without_spacer 162
term::search::tests::wrap_around_to_another_end 163
term::search::tests::wrapping 164
term::search::tests::wrapping_into_fullwidth 165
term::tests::block_selection_works 166
term::tests::clear_saved_lines 167
term::tests::clearing_scrollback_resets_display_offset 168
term::tests::clearing_scrollback_sets_vi_cursor_into_viewport 169
term::tests::clearing_viewport_keeps_history_position 170
term::tests::clearing_viewport_with_vi_mode_keeps_history_position 171
term::tests::damage_cursor_movements 172
term::tests::damage_public_usage 173
term::tests::full_damage 174
term::tests::grid_serde 175
term::tests::grow_lines_updates_active_cursor_pos 176
term::tests::grow_lines_updates_inactive_cursor_pos 177
term::tests::input_line_drawing_character 178
term::tests::line_selection_works 179
term::tests::parse_cargo_version 180
term::tests::scroll_display_page_down 181
term::tests::scroll_display_page_up 182
term::tests::semantic_selection_works 183
term::tests::shrink_lines_updates_active_cursor_pos 184
term::tests::shrink_lines_updates_inactive_cursor_pos 185
term::tests::simple_selection_works 186
term::tests::vi_cursor_keep_pos_on_scrollback_buffer 187
term::tests::window_title 188
tty::unix::test_get_pw_entry 189
vi_mode::tests::motion_bracket 190
vi_mode::tests::motion_first_occupied 191
vi_mode::tests::motion_high_middle_low 192
vi_mode::tests::motion_semantic_left_end 193
vi_mode::tests::motion_semantic_left_start 194
vi_mode::tests::motion_semantic_right_end 195
vi_mode::tests::motion_semantic_right_start 196
vi_mode::tests::motion_simple 197
vi_mode::tests::motion_start_end 198
vi_mode::tests::motion_word 199
vi_mode::tests::scroll_over_bottom 200
vi_mode::tests::scroll_over_top 201
vi_mode::tests::scroll_semantic 202
vi_mode::tests::scroll_simple 203
vi_mode::tests::scroll_word 204
vi_mode::tests::semantic_wide 205
vi_mode::tests::simple_wide 206
vi_mode::tests::word_wide 207
alt_reset 208
clear_underline 209
colored_reset 210
colored_underline 211
csi_rep 212
decaln_reset 213
deccolm_reset 214
delete_chars_reset 215
delete_lines 216
erase_chars_reset 217
erase_in_line 218
fish_cc 219
grid_reset 220
history 221
hyperlinks 222
indexed_256_colors 223
insert_blank_reset 224
issue_855 225
ll 226
newline_with_cursor_beyond_scroll_region 227
region_scroll_down 228
row_reset 229
saved_cursor 230
saved_cursor_alt 231
scroll_up_reset 232
selective_erasure 233
sgr 234
tab_rendering 235
tmux_git_log 236
tmux_htop 237
underline 238
vim_24bitcolors_bce 239
vim_large_window_scroll 240
vim_simple_edit 241
vttest_cursor_movement_1 242
vttest_insert 243
vttest_origin_mode_1 244
vttest_origin_mode_2 245
vttest_scroll 246
vttest_tab_clear_set 247
wrapline_alt_toggle 248
zerowidth 249
zsh_tab_completion 250
None -1

module to integer mapping
vttest_scroll 0
clear_underline 1
vttest_tab_clear_set 2
tmux_htop 3
field_replacement 4
insert_blank_reset 5
vttest_cursor_movement_1 6
wrapline_alt_toggle 7
saved_cursor 8
region_scroll_down 9
vim_24bitcolors_bce 10
colored_underline 11
config_deserialize 12
selection 13
replace_derive 14
alt_reset 15
cli 16
message_bar 17
grid 18
csi_rep 19
delete_lines 20
selective_erasure 21
zerowidth 22
config 23
vttest_insert 24
fish_cc 25
tab_rendering 26
decaln_reset 27
erase_in_line 28
grid_reset 29
vim_large_window_scroll 30
underline 31
tty 32
history 33
indexed_256_colors 34
row_reset 35
replace_flatten 36
vi_mode 37
index 38
sgr 39
ll 40
colored_reset 41
ansi 42
saved_cursor_alt 43
tmux_git_log 44
delete_chars_reset 45
hyperlinks 46
scroll_up_reset 47
vttest_origin_mode_1 48
zsh_tab_completion 49
issue_855 50
erase_chars_reset 51
vim_simple_edit 52
term 53
renderer 54
deccolm_reset 55
display 56
input 57
vttest_origin_mode_2 58
newline_with_cursor_beyond_scroll_region 59
string 60
None -1
*%
