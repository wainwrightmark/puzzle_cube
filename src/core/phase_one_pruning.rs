use std::process::id;

use crate::core::prelude::*;
use array_const_fn_init::array_const_fn_init;
use itertools::Itertools;
use rand::{prelude::StdRng, Rng, SeedableRng};
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::*;

impl DataSource {
    fn set_flip_slice_twist_depth3(ix: usize, value: u32, tab: &mut Vec<u32>) {
        let shift = (ix % 16) * 2;
        let base = ix >> 4;
        let mut tb = tab[base];
        tb &= !(3u32 << shift) & u32::MAX;
        tb |= value << shift;

        tab[base] = tb;
    }

    fn get_flip_slice_twist_depth3(ix: usize, tab: &Vec<u32>) -> u32 {
        let mut y = tab[ix / 16];
        y >>= (ix % 16) * 2;
        (y & 3)
    }

    pub fn create_phase_1_pruning(
        moves_source: &MovesSource,
        flip_slice_source: &FlipSliceSource,
    ) -> Vec<u32> {
        let total = 64430 * 2187;
        let fs_sym = DataSource::make_flip_slice_sym(flip_slice_source);
        let mut table = vec![u32::MAX; (total / 16) + 1];

        let twist = 0;
        Self::set_flip_slice_twist_depth3(0, 0, &mut table);
        let mut done = 1;
        let mut depth = 0u32;
        let mut back_search = false;

        while done < total {
            let depth3 = depth % 3;
            if depth == 9 {
                back_search = true;
            }
            let mult = if depth < 8 { 5 } else { 1 };

            let mut idx = 0;
            for fs_class_index in 0..64430 {
                let mut twist = 0;
                while twist < 2187 {
                    if !back_search
                        && idx % 16 == 0
                        && table[idx / 16] == u32::MAX
                        && twist < (2187 - 16)
                    {
                        twist += 16;
                        idx += 16;
                        continue;
                    }

                    let is_match = if (back_search) {
                        Self::get_flip_slice_twist_depth3(idx, &table) == 3
                    } else {
                        Self::get_flip_slice_twist_depth3(idx, &table) == depth3
                    };

                    if is_match {
                        let flip_slice = flip_slice_source.flip_slice_rep[fs_class_index];
                        let flip = (flip_slice % 2048) as u16;
                        let slice = (flip_slice >> 11) as u16;
                        for m in Move::ALLMOVES {
                            let flip1 = moves_source.get_flip(flip, m);
                            let slice1 = moves_source.get_slice(slice, m);
                            let flip_slice1 = ((slice1 as usize) << 11) + flip1 as usize;
                            let fs_class_idx = flip_slice_source.flip_slice_class_index
                                [flip_slice1]
                                as usize;
                            let fs_symmetry =
                                flip_slice_source.flip_slice_symmetry[flip_slice1];
                            let twist1 = moves_source
                                .get_twist_conj(moves_source.get_twist(twist, m), fs_symmetry);
                            let idx1 = ((2187 * fs_class_idx) + twist1 as usize);

                            if back_search {
                                if Self::get_flip_slice_twist_depth3(idx1, &table) == depth3 {
                                    Self::set_flip_slice_twist_depth3(
                                        idx,
                                        (depth + 1) % 3,
                                        &mut table,
                                    );
                                    done += 1;
                                }
                            } else {
                                if Self::get_flip_slice_twist_depth3(idx1, &table) == 3 {
                                    Self::set_flip_slice_twist_depth3(
                                        idx1,
                                        (depth + 1) % 3,
                                        &mut table,
                                    );
                                    done += 1;

                                    let mut sym = fs_sym[fs_class_idx];
                                    if sym != 1 {
                                        for j in 1..16 {
                                            sym = sym >> 1;
                                            if sym % 2 == 1 {
                                                let twist2 = moves_source.get_twist_conj(twist1, j);
                                                let idx2 = (2187 * fs_class_idx) + twist2 as usize;
                                                if Self::get_flip_slice_twist_depth3(idx2, &table)
                                                    == 3
                                                {
                                                    Self::set_flip_slice_twist_depth3(
                                                        idx2,
                                                        (depth + 1) % 3,
                                                        &mut table,
                                                    );
                                                    done += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    twist += 1;
                    idx += 1;
                }
            }

            depth += 1;
        }

        table
    }

    pub fn make_flip_slice_sym(flip_slice_source: &FlipSliceSource) -> [u16; 64430] {
        let mut cc = CubieCube::default();
        let mut fs_sym = [u16::MIN; 64430];
        for i in 0..64430 {
            let rep = flip_slice_source.flip_slice_rep[i];
            let rep_mod_flip = (rep % 2048) as u16;
            let rep_div_flip = (rep / 2048) as u16; 

            cc.set_slice(rep_div_flip);
            cc.set_flip(rep_mod_flip);
            for s in 0..16 {
                let mut ss = SYMMETRY_CUBES[s].clone();
                ss = ss.edge_multiply(&cc);
                ss = ss.edge_multiply(&SYMMETRY_CUBES_INVERTED[s]);
                let slice = ss.get_slice();
                if slice == rep_div_flip && ss.get_flip() == rep_mod_flip {
                    let q = 1 << s;
                    fs_sym[i] |= q;
                }
            }
        }
        fs_sym
    }
}
