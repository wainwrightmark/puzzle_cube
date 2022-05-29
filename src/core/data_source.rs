use crate::core::prelude::*;
use array_const_fn_init::array_const_fn_init;
use itertools::Itertools;
use serde::__private::de;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::*;
use rand::{SeedableRng, prelude::StdRng, Rng};



//#[derive(BorshSerialize, BorshDeserialize)]
pub struct DataSource{

    ///Indicates the minimum number of phase 2 moves required to solve the corners and slice. Indexed by corners * 24 + slice
    pub corner_slice_depth: Vec<u8>,
    pub u_d_edges_conjugation: Vec<u16>,
    pub phase_2_pruning: Vec<u32>,
    pub phase_2_edge_merge: Vec<u16>,
    pub phase_1_pruning: Vec<u32>,

    pub twist_conjugation : Vec<u16>,

    pub flip_slice_class_index : Vec<u16>,
    pub flic_slice_symmetry : Vec<u8>,
    pub flip_slice_rep : Vec<u32>,
    
    
    pub corner_class_index : Vec<u16>,
    pub corner_symmetry : Vec<u8>,
    pub corner_rep : Vec<u16>,

    pub moves_source: MovesSource
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
}



impl DataSource{
    pub fn get_flip_slice_twist_depth_mod_3(&self, flip: u16, twist: u16, slice_sorted: u16,) -> u8{
        let slice = slice_sorted / 24;
        let flip_slice = (NFLIP * slice  + flip) as usize;

        let class_index = self.flip_slice_class_index[flip_slice];
        let flip_slice_sym = self.flic_slice_symmetry[flip_slice];

        let twist_conj = self.get_twist_conj(twist, flip_slice_sym);

        let ix = (NTWIST * class_index + twist_conj) as usize;

        let mut y = self.phase_1_pruning[ix / 16];
        y >>= (ix % 16) * 2;
        let r = y & 3;

        r as u8
    }

    pub fn get_twist_conj(&self, twist: u16, flip_slice_sym : u8)->u16{
        let r = self.twist_conjugation[(twist << 4) as usize + flip_slice_sym as usize];
        return r;
    }

    pub fn get_ud_edges(&self, u_edges: u16, d_edges: u16)-> u16{
        let index = 24 * u_edges + (d_edges % 24);
        let r = self.phase_2_edge_merge[index as usize];
        r
    }

    pub fn get_corners_ud_edges_depth_3(&self, corners:  u16, ud_edges: u16) -> u8 {
        let corner_class_index = self.corner_class_index[corners as usize];
        let corner_sym = self.corner_symmetry[corners as usize];

        let ud_edges_conj = self.get_ud_edges_conj(ud_edges, corner_sym);
        let index = (NUDEDGES * corner_class_index + ud_edges_conj) as usize; 

        let mut y = self.phase_2_pruning[index / 16];
        y >>= (index % 16) * 2;
        (y & 3) as u8
    }

    pub fn get_ud_edges_conj(&self, ud_edges: u16, corner_sym: u8) -> u16 {
        let idx = (ud_edges << 4) as usize + corner_sym as usize;
        let r = self.u_d_edges_conjugation[idx];
        r
    }

    pub fn get_cornslice_depth(&self, corners: u16, slice_sorted: u16)-> u8{
        let index = (24 * corners + slice_sorted) as usize;
        let r = self.corner_slice_depth[index];

        r
    }
}



//#[derive(BorshSerialize, BorshDeserialize)]
pub struct MovesSource{
    pub twist_move : Vec<u16>,
    pub flip_move :  Vec<u16>,
    pub slice_sorted_move :  Vec<u16>,
    pub u_edges_move : Vec<u16>,
    pub d_edges_move :  Vec<u16>,
    pub u_d_edges_move :  Vec<u16>,
    pub corners_move :  Vec<u16>,
}

impl MovesSource{
    pub fn create()-> Self{

        let corners_move = CornersProperty::create(&CornersProperty{}).into();
        let twist_move = TwistProperty::create(&TwistProperty{}).into();
        let flip_move = FlipProperty::create(&FlipProperty{}).into();
        let slice_sorted_move = SliceSortedProperty::create(&SliceSortedProperty{}).into();
        let u_edges_move = UpEdgesProperty::create(&UpEdgesProperty{}).into();
        let d_edges_move = DownEdgesProperty::create(&DownEdgesProperty{}).into();
        let u_d_edges_move = UpDownEdgesProperty::create(&UpDownEdgesProperty{}).into();
        

        Self { twist_move, flip_move, slice_sorted_move, u_edges_move, d_edges_move, u_d_edges_move, corners_move }        
    }



}