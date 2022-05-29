use crate::core::prelude::*;
use array_const_fn_init::array_const_fn_init;
use itertools::Itertools;
use serde::__private::de;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::*;
use rand::{SeedableRng, prelude::StdRng, Rng};


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
}