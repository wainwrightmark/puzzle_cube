use crate::core::prelude::*;
use array_const_fn_init::array_const_fn_init;
use itertools::Itertools;
use serde::__private::de;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::*;
use rand::{SeedableRng, prelude::StdRng, Rng};

impl CornerSymmetriesSource{
    pub fn create()-> CornerSymmetriesSource{
        
        let mut corner_class_index = [u16::MAX; 40320];
        let mut corner_symmetry= [u8::MIN;40320];
        let mut corner_rep = [u16::MIN;2768];

        let mut class_idx:usize = 0;
        let mut cube = CubieCube::default();        
        for cp in 0..40320{
            cube.set_corners(cp as u16);

            if corner_class_index[cp] == u16::MAX{
                corner_class_index[cp] = class_idx as u16;
                corner_symmetry[cp] = 0;
                corner_rep[class_idx] = cp as u16;
            }
            else {
                continue;
            }

            for s in 0..16{
                let mut ss = SYMMETRY_CUBES_INVERTED[s].clone();
                ss = ss.corner_multiply(&cube);
                ss = ss.corner_multiply(&SYMMETRY_CUBES[s]);

                let cp_new = ss.get_corners() as usize;
                if corner_class_index[cp_new] == u16::MAX{
                    corner_class_index[cp_new] = class_idx as u16;
                    corner_symmetry[cp_new] = s as u8;
                }

            }
            class_idx+= 1;
        }

        CornerSymmetriesSource { corner_class_index: corner_class_index.into(), corner_symmetry: corner_symmetry.into(), corner_rep: corner_rep.into()}
    }
}


impl DataSource{
    pub fn create_corner_slice_depth(moves_source: &MovesSource)-> Vec<u8>
    {
        let mut table = vec![u8::MAX;40320 * 24];
                    

        table[0] = 0;
        let mut done = 1;
        let mut depth = 0;

        let mut next:Vec<(usize, usize)> = vec![(0,0)];

        while(done < 40320 * 24) {

            let mut next_next:Vec<(usize, usize)> = Vec::new();
            for (corners, slice) in next {
                for m in Move::PHASE2MOVES{
                    let corners1 = moves_source.corners_move [(18 * corners) + m as usize] as usize;
                    let slice1 = moves_source.slice_sorted_move [(18 * slice) + m as usize] as usize;
                    let idx1 = (24 * corners1 + slice1) ;

                    assert!(idx1 < 40320 * 24);
        
                    if table[idx1] == u8::MAX{//this is the first time we have reached this point
                        table[idx1] = depth + 1;
                        next_next.push((corners1, slice1));
                        done+=1;
                    }
        
                }
            }
            depth+=1;
            next= next_next;            
        }       

        table
    }

    pub fn create_up_down_edges_conjugation() -> Vec<u16>{
        let mut table:Vec<u16> = Vec::new();
        table.reserve_exact(40320 * 16);

        for edges in 0..40320{
            let mut edges_cube = CubieCube::default();
            edges_cube.set_ud_edges(edges);            

            for symmetry in 0..16{
                let mut sym_cube = SYMMETRY_CUBES[symmetry].clone();
                sym_cube = sym_cube.edge_multiply(&edges_cube);
                sym_cube = sym_cube.edge_multiply(&SYMMETRY_CUBES_INVERTED[symmetry]);
                let ud_edges = sym_cube.get_ud_edges().unwrap();
                table.push(ud_edges);
            }
        }

        table
    }
}