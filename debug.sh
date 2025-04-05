#! /bin/bash

# Hello! Let's start debugging homados :)
printf -- "\n>------------------------------------------------------<\n"
printf -- "                   Starting Debug! :)\n"
printf -- ">------------------------------------------------------<\n\n"

# Set our working directory, then display it.
DIR="$(dirname "$(realpath "$0")")"/
printf -- " - Current Directory: %s\n\n" "$DIR"

# Announce task, then perform
printf -- ">------------------------------------------------------<\n"
printf -- " Testing all sound types on 10 second duration sine.\n"
printf -- ">------------------------------------------------------<\n\n"

cargo run "./homados Output/debug" "debug sound 000--silence" -t silence
cargo run "./homados Output/debug" "debug sound 001--dc offset full-scale" -t dc
cargo run "./homados Output/debug" "debug sound 002--sine 440hz" -t sin
cargo run "./homados Output/debug" "debug sound 003--cosine 440hz" -t cos
cargo run "./homados Output/debug" "debug sound 004--sweep lin sin" -t "sweep_lin_sin"
cargo run "./homados Output/debug" "debug sound 005--sweep exp sin" -t "sweep_exp_sin"
cargo run "./homados Output/debug" "debug sound 006--clipped sin -18dBFS 440hz" -t "clip_sin" --Param1db -18.0617997398
cargo run "./homados Output/debug" "debug sound 007--quantized sin 440hz" -t "quant_sin"
cargo run "./homados Output/debug" "debug sound 008--sawtooth 440hz" -t "saw"
cargo run "./homados Output/debug" "debug sound 009--square 440hz" -t "sqr"
cargo run "./homados Output/debug" "debug sound 010--triangle 440hz" -t "tri"
cargo run "./homados Output/debug" "debug sound 011--sharktooth 440hz" -t "shark"
cargo run "./homados Output/debug" "debug sound 012--unit impulse" -t "unit_impulse"
cargo run "./homados Output/debug" "debug sound 013--dirac comb 440hz" -t "dirac_comb"
cargo run "./homados Output/debug" "debug sound 014--random uniform noise" -t "random"
cargo run "./homados Output/debug" "debug sound 015--white uniform distribution" -t "white"
cargo run "./homados Output/debug" "debug sound 016--white normal distribution" -t "white_normal" --Param1 1.0
cargo run "./homados Output/debug" "debug sound 017--white triangular distribution" -t "white_tri"
cargo run "./homados Output/debug" "debug sound 018--white binary distribution" -t "white_bin"
cargo run "./homados Output/debug" "debug sound 019--pink-kellet econ" -t "pke"
cargo run "./homados Output/debug" "debug sound 020--pink-kellet ref" -t "pk3"
cargo run "./homados Output/debug" "debug sound 021--brown" -t "brown"
cargo run "./homados Output/debug" "debug sound 022--blue-kellet econ" -t "blue_pke"
cargo run "./homados Output/debug" "debug sound 023--blue-kellet ref" -t "blue_pk3"
cargo run "./homados Output/debug" "debug sound 024--violet" -t "violet"
cargo run "./homados Output/debug" "debug sound 025a--pseudo velvet 0.5" -t "pseudo_velvet" --Param1 0.5
cargo run "./homados Output/debug" "debug sound 025b--pseudo velvet 0.99" -t "pseudo_velvet" --Param1 0.995


# Announce task, then perform
printf -- ">------------------------------------------------------<\n"
printf -- " Testing all window types on 10 second duration sine.\n"
printf -- ">------------------------------------------------------<\n\n"

cargo run "./homados Output/debug" "debug window 000--default" -t sine -w def
cargo run "./homados Output/debug" "debug window 001--linear-out" -t sine -w lin_out
cargo run "./homados Output/debug" "debug window 002--linear-in" -t sine -w lin_in
cargo run "./homados Output/debug" "debug window 003--linear-io" -t sine -w lin_io
cargo run "./homados Output/debug" "debug window 004--linear-oi" -t sine -w lin_oi
cargo run "./homados Output/debug" "debug window 005--exp1-out" -t sine -w exp1_out
cargo run "./homados Output/debug" "debug window 006--exp1-in" -t sine -w exp1_in
cargo run "./homados Output/debug" "debug window 007--exp1-io" -t sine -w exp1_io
cargo run "./homados Output/debug" "debug window 008--exp1-oi" -t sine -w exp1_oi
cargo run "./homados Output/debug" "debug window 009--exp2-out" -t sine -w exp2_out
cargo run "./homados Output/debug" "debug window 010--exp2-in" -t sine -w exp2_in
cargo run "./homados Output/debug" "debug window 011--exp2-io" -t sine -w exp2_io
cargo run "./homados Output/debug" "debug window 012--exp2-oi" -t sine -w exp2_oi
cargo run "./homados Output/debug" "debug window 013--exp3-out" -t sine -w exp3_out
cargo run "./homados Output/debug" "debug window 014--exp3-in" -t sine -w exp3_in
cargo run "./homados Output/debug" "debug window 015--exp3-io" -t sine -w exp3_io
cargo run "./homados Output/debug" "debug window 016--exp3-oi" -t sine -w exp3_oi
cargo run "./homados Output/debug" "debug window 017--exp4-out" -t sine -w exp4_out
cargo run "./homados Output/debug" "debug window 018--exp4-in" -t sine -w exp4_in
cargo run "./homados Output/debug" "debug window 019--exp4-io" -t sine -w exp4_io
cargo run "./homados Output/debug" "debug window 020--exp4-oi" -t sine -w exp4_oi
cargo run "./homados Output/debug" "debug window 021--exp5-out" -t sine -w exp5_out
cargo run "./homados Output/debug" "debug window 022--exp5-in" -t sine -w exp5_in
cargo run "./homados Output/debug" "debug window 023--exp5-io" -t sine -w exp5_io
cargo run "./homados Output/debug" "debug window 024--exp5-oi" -t sine -w exp5_oi
cargo run "./homados Output/debug" "debug window 025--log1-out" -t sine -w log1_out
cargo run "./homados Output/debug" "debug window 026--log1-in" -t sine -w log1_in
cargo run "./homados Output/debug" "debug window 027--log1-io" -t sine -w log1_io
cargo run "./homados Output/debug" "debug window 028--log1-oi" -t sine -w log1_oi
cargo run "./homados Output/debug" "debug window 029--log2-out" -t sine -w log2_out
cargo run "./homados Output/debug" "debug window 030--log2-in" -t sine -w log2_in
cargo run "./homados Output/debug" "debug window 031--log2-io" -t sine -w log2_io
cargo run "./homados Output/debug" "debug window 032--log2-oi" -t sine -w log2_oi
cargo run "./homados Output/debug" "debug window 033--eqp1-out" -t sine -w eqp1_out
cargo run "./homados Output/debug" "debug window 034--eqp1-in" -t sine -w eqp1_in
cargo run "./homados Output/debug" "debug window 035--eqp1-io" -t sine -w eqp1_io
cargo run "./homados Output/debug" "debug window 036--eqp1-oi" -t sine -w eqp1_oi
cargo run "./homados Output/debug" "debug window 037--eqp2-out" -t sine -w eqp2_out
cargo run "./homados Output/debug" "debug window 038--eqp2-in" -t sine -w eqp2_in
cargo run "./homados Output/debug" "debug window 039--eqp2-io" -t sine -w eqp2_io
cargo run "./homados Output/debug" "debug window 040--eqp2-oi" -t sine -w eqp2_oi
cargo run "./homados Output/debug" "debug window 041--sc1-out" -t sine -w sc1_out
cargo run "./homados Output/debug" "debug window 042--sc1-in" -t sine -w sc1_in
cargo run "./homados Output/debug" "debug window 043--sc1-io" -t sine -w sc1_io
cargo run "./homados Output/debug" "debug window 044--sc1-oi" -t sine -w sc1_oi
cargo run "./homados Output/debug" "debug window 045--sc2-out" -t sine -w sc2_out
cargo run "./homados Output/debug" "debug window 046--sc2-in" -t sine -w sc2_in
cargo run "./homados Output/debug" "debug window 047--sc2-io" -t sine -w sc2_io
cargo run "./homados Output/debug" "debug window 048--sc2-oi" -t sine -w sc2_oi
cargo run "./homados Output/debug" "debug window 049--sc3-out" -t sine -w sc3_out
cargo run "./homados Output/debug" "debug window 050--sc3-in" -t sine -w sc3_in
cargo run "./homados Output/debug" "debug window 051--sc3-io" -t sine -w sc3_io
cargo run "./homados Output/debug" "debug window 052--sc3-oi" -t sine -w sc3_oi
cargo run "./homados Output/debug" "debug window 053--sc4-out" -t sine -w sc4_out
cargo run "./homados Output/debug" "debug window 054--sc4-in" -t sine -w sc4_in
cargo run "./homados Output/debug" "debug window 055--sc4-io" -t sine -w sc4_io
cargo run "./homados Output/debug" "debug window 056--sc4-oi" -t sine -w sc4_oi
cargo run "./homados Output/debug" "debug window 057--chs-out" -t sine -w chs_out
cargo run "./homados Output/debug" "debug window 058--chs-in" -t sine -w chs_in
cargo run "./homados Output/debug" "debug window 059--chs-io" -t sine -w chs_io
cargo run "./homados Output/debug" "debug window 060--chs-oi" -t sine -w chs_oi
cargo run "./homados Output/debug" "debug window 061--chsg-out" -t sine -w chsg_out
cargo run "./homados Output/debug" "debug window 062--chsg-in" -t sine -w chsg_in
cargo run "./homados Output/debug" "debug window 063--chsg-io" -t sine -w chsg_io
cargo run "./homados Output/debug" "debug window 064--chsg-oi" -t sine -w chsg_oi
cargo run "./homados Output/debug" "debug window 065--sscf-out" -t sine -w sscf_out
cargo run "./homados Output/debug" "debug window 066--sscf-in" -t sine -w sscf_in
cargo run "./homados Output/debug" "debug window 067--sscf-io" -t sine -w sscf_io
cargo run "./homados Output/debug" "debug window 068--sscf-oi" -t sine -w sscf_oi
cargo run "./homados Output/debug" "debug window 069--tet-out" -t sine -w tet_out
cargo run "./homados Output/debug" "debug window 070--tet-in" -t sine -w tet_in
cargo run "./homados Output/debug" "debug window 071--tet-io" -t sine -w tet_io
cargo run "./homados Output/debug" "debug window 072--tet-oi" -t sine -w tet_oi
cargo run "./homados Output/debug" "debug window 073--slg-out" -t sine -w slg_out
cargo run "./homados Output/debug" "debug window 074--slg-in" -t sine -w slg_in
cargo run "./homados Output/debug" "debug window 075--slg-io" -t sine -w slg_io
cargo run "./homados Output/debug" "debug window 076--slg-oi" -t sine -w slg_oi
