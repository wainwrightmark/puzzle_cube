

use crate::core::prelude::*;






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
        y & 3
    }

    pub fn create_phase_1_pruning(
        moves_source: &MovesSource,
        flip_slice_source: &FlipSliceSource,
    ) -> Vec<u32> {
        let total = NFLIPSLICECLASS * NTWIST;
        let fs_sym = DataSource::make_flip_slice_sym(flip_slice_source);
        let mut table = vec![u32::MAX; (total / 16) + 1];

        let _twist = 0;
        Self::set_flip_slice_twist_depth3(0, 0, &mut table);
        let mut done = 1;
        let mut depth = 0u32;

        let mut next = vec![(0,0)];//next is tuples of (fs_class_idx, twist)
        let mut current: Vec<(usize, u16)> = Vec::new();
        current.reserve_exact(7950748); //magic number
        next.reserve_exact(7950748);

        while depth < 9 {
            
            std::mem::swap(&mut next,&mut current);

            for (fs_class_idx, twist) in current.drain(..){

                let flip_slice = flip_slice_source.flip_slice_rep[fs_class_idx];
                let flip = (flip_slice % 2048) as u16;
                let slice = (flip_slice >> 11) as u16;
                for m in Move::ALLMOVES {
                    let flip_after_move = moves_source.get_flip(flip, m);
                    let slice_after_move = moves_source.get_slice(slice, m);
                    let flip_slice_after_move =
                        ((slice_after_move as usize) << 11) + flip_after_move as usize;
                    let fs_class_idx_after_move = flip_slice_source.flip_slice_class_index
                        [flip_slice_after_move]
                        as usize;
                    let fs_symmetry_after_move =
                        flip_slice_source.flip_slice_symmetry[flip_slice_after_move];
                    let twist_after_move = moves_source.get_twist_conj(
                        moves_source.get_twist(twist, m),
                        fs_symmetry_after_move,
                    );
                    let idx_after_move =
                        (2187 * fs_class_idx_after_move) + twist_after_move as usize;

                    if Self::get_flip_slice_twist_depth3(idx_after_move, &table) == 3 {
                        Self::set_flip_slice_twist_depth3(
                            idx_after_move,
                            (depth + 1) % 3,
                            &mut table,
                        );
                        done += 1;
                        if depth < 8{
                            next.push((fs_class_idx_after_move, twist_after_move));
                        }
                        

                        let mut sym = fs_sym[fs_class_idx_after_move];
                        if sym != 1 {
                            for j in 1..16 {
                                sym = sym >> 1;
                                if sym % 2 == 1 {
                                    let twist_after_move_and_symmetry =
                                        moves_source.get_twist_conj(twist_after_move, j);
                                    let index_after_move_and_symmetry = (2187
                                        * fs_class_idx_after_move)
                                        + twist_after_move_and_symmetry as usize;
                                    if Self::get_flip_slice_twist_depth3(
                                        index_after_move_and_symmetry,
                                        &table,
                                    ) == 3
                                    {
                                        Self::set_flip_slice_twist_depth3(
                                            index_after_move_and_symmetry,
                                            (depth + 1) % 3,
                                            &mut table,
                                        );
                                        done += 1;
                                        if depth < 8{
                                            next.push((fs_class_idx_after_move, twist_after_move_and_symmetry));
                                        }
                                        
                                    }
                                }
                            }
                        }
                    }
                }
            }
            depth = depth + 1;
        }

        while done < total {
            let depth3 = depth % 3;

            let mut idx = 0;
            for fs_class_index in 0..NFLIPSLICECLASS {
                let mut twist = 0;
                while twist < 2187 {

                    let is_match = Self::get_flip_slice_twist_depth3(idx, &table) == 3;

                    if is_match {
                        let flip_slice = flip_slice_source.flip_slice_rep[fs_class_index];
                        let flip = (flip_slice % 2048) as u16;
                        let slice = (flip_slice >> 11) as u16;
                        for m in Move::ALLMOVES {
                            let flip_after_move = moves_source.get_flip(flip, m);
                            let slice_after_move = moves_source.get_slice(slice, m);
                            let flip_slice_after_move =
                                ((slice_after_move as usize) << 11) + flip_after_move as usize;
                            let fs_class_idx_after_move = flip_slice_source.flip_slice_class_index
                                [flip_slice_after_move]
                                as usize;
                            let fs_symmetry_after_move =
                                flip_slice_source.flip_slice_symmetry[flip_slice_after_move];
                            let twist_after_move = moves_source.get_twist_conj(
                                moves_source.get_twist(twist, m),
                                fs_symmetry_after_move,
                            );
                            let idx_after_move =
                                (2187 * fs_class_idx_after_move) + twist_after_move as usize;

                                if Self::get_flip_slice_twist_depth3(idx_after_move, &table)
                                == depth3
                            {
                                Self::set_flip_slice_twist_depth3(
                                    idx,
                                    (depth + 1) % 3,
                                    &mut table,
                                );
                                done += 1;
                                break;
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


    pub fn make_flip_slice_sym(flip_slice_source: &FlipSliceSource) -> [u16; NFLIPSLICECLASS] {
        let mut cc = CubieCube::default();
        let mut fs_sym = [u16::MIN; NFLIPSLICECLASS];
        for i in 0..NFLIPSLICECLASS {
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
