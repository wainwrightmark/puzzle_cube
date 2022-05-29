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
    
    pub corners_source : CornerSymmetriesSource,
    

    pub moves_source: MovesSource
}

pub struct CornerSymmetriesSource{
    pub corner_class_index : Vec<u16>,
    pub corner_symmetry : Vec<u8>,
    pub corner_rep : Vec<u16>,
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
        
        self.twist_conjugation[(twist << 4) as usize + flip_slice_sym as usize]
    }

    pub fn get_ud_edges(&self, u_edges: u16, d_edges: u16)-> u16{
        let index = 24 * u_edges + (d_edges % 24);
        
        self.phase_2_edge_merge[index as usize]
    }

    pub fn get_corners_ud_edges_depth_3(&self, corners:  u16, ud_edges: u16) -> u8 {
        let corner_class_index = self.corners_source.corner_class_index[corners as usize];
        let corner_sym = self.corners_source. corner_symmetry[corners as usize];

        let ud_edges_conj = self.get_ud_edges_conj(ud_edges, corner_sym);
        let index = (NUDEDGES * corner_class_index + ud_edges_conj) as usize; 

        let mut y = self.phase_2_pruning[index / 16];
        y >>= (index % 16) * 2;
        (y & 3) as u8
    }

    pub fn get_ud_edges_conj(&self, ud_edges: u16, corner_sym: u8) -> u16 {
        let idx = (ud_edges << 4) as usize + corner_sym as usize;
        
        self.u_d_edges_conjugation[idx]
    }

    pub fn get_cornslice_depth(&self, corners: u16, slice_sorted: u16)-> u8{
        let index = (24 * corners + slice_sorted) as usize;
        

        self.corner_slice_depth[index]
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

        let corners_move = CornersProperty::create(&CornersProperty{});
        let twist_move = TwistProperty::create(&TwistProperty{});
        let flip_move = FlipProperty::create(&FlipProperty{});
        let slice_sorted_move = SliceSortedProperty::create(&SliceSortedProperty{});
        let u_edges_move = UpEdgesProperty::create(&UpEdgesProperty{});
        let d_edges_move = DownEdgesProperty::create(&DownEdgesProperty{});
        let u_d_edges_move = UpDownEdgesProperty::create(&UpDownEdgesProperty{});
        

        Self { twist_move, flip_move, slice_sorted_move, u_edges_move, d_edges_move, u_d_edges_move, corners_move }        
    }



}