use crate::core::prelude::*;

impl FlipSliceSource {
    pub fn create() -> FlipSliceSource {
        let mut flip_slice_class_index = vec![u16::MAX; 2048 * 495];
        let mut flip_slice_symmetry = vec![u8::MIN; 2048 * 495];
        let mut flip_slice_rep = [u32::MIN; 64430];

        let mut class_index: usize = 0;
        let mut cube = CubieCube::default();
        for slice in 0..495 {
            cube.set_slice(slice as u16);

            for flip in 0..2048 {
                cube.set_flip(flip);

                let index = (2048 * (slice as usize)) + (flip as usize);

                if flip_slice_class_index[index] == u16::MAX {
                    flip_slice_class_index[index] = class_index as u16;
                    flip_slice_symmetry[index] = 0;
                    flip_slice_rep[class_index] = index as u32;

                    for sym in 0..16 {
                        let mut ss = SYMMETRY_CUBES_INVERTED[sym].clone();
                        ss = ss.edge_multiply(&cube);
                        ss = ss.edge_multiply(&SYMMETRY_CUBES[sym]);

                        let index_new = (2048 * ss.get_slice() as usize) + ss.get_flip() as usize;
                        if flip_slice_class_index[index_new] == u16::MAX {
                            flip_slice_class_index[index_new] = class_index as u16;
                            flip_slice_symmetry[index_new] = sym as u8;
                        }
                    }

                    class_index += 1;
                }
            }
        }

        FlipSliceSource {
            flip_slice_class_index,
            flip_slice_symmetry,
            flip_slice_rep: flip_slice_rep.into(),
        }
    }
}

impl CornerSymmetriesSource {
    pub fn create() -> CornerSymmetriesSource {
        let mut corner_class_index = [u16::MAX; 40320];
        let mut corner_symmetry = [u8::MIN; 40320];
        let mut corner_rep = [u16::MIN; 2768];

        let mut class_idx: usize = 0;
        let mut cube = CubieCube::default();
        for cp in 0..40320 {
            cube.set_corners(cp as u16);

            if corner_class_index[cp] == u16::MAX {
                corner_class_index[cp] = class_idx as u16;
                corner_symmetry[cp] = 0;
                corner_rep[class_idx] = cp as u16;
            } else {
                continue;
            }

            for s in 0..16 {
                let mut ss = SYMMETRY_CUBES_INVERTED[s].clone();
                ss = ss.corner_multiply(&cube);
                ss = ss.corner_multiply(&SYMMETRY_CUBES[s]);

                let cp_new = ss.get_corners() as usize;
                if corner_class_index[cp_new] == u16::MAX {
                    corner_class_index[cp_new] = class_idx as u16;
                    corner_symmetry[cp_new] = s as u8;
                }
            }
            class_idx += 1;
        }

        CornerSymmetriesSource {
            corner_class_index: corner_class_index.into(),
            corner_symmetry: corner_symmetry.into(),
            corner_rep: corner_rep.into(),
        }
    }
}

impl DataSource {
    pub fn create_corner_slice_depth(moves_source: &MovesSource) -> Vec<u8> {
        let mut table = vec![u8::MAX; 40320 * 24];

        table[0] = 0;
        let mut done = 1;
        let mut depth = 0;

        let mut next: Vec<(usize, usize)> = vec![(0, 0)];

        while done < 40320 * 24 {
            let mut next_next: Vec<(usize, usize)> = Vec::new();
            for (corners, slice) in next {
                for m in Move::PHASE2MOVES {
                    let corners1 = moves_source.corners_move[(18 * corners) + m as usize] as usize;
                    let slice1 = moves_source.slice_sorted_move[(18 * slice) + m as usize] as usize;
                    let idx1 = 24 * corners1 + slice1;

                    assert!(idx1 < 40320 * 24);

                    if table[idx1] == u8::MAX {
                        //this is the first time we have reached this point
                        table[idx1] = depth + 1;
                        next_next.push((corners1, slice1));
                        done += 1;
                    }
                }
            }
            depth += 1;
            next = next_next;
        }

        table
    }
}
