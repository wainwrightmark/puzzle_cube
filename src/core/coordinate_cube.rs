use std::default;

use crate::core::prelude::FaceColor::*;
use crate::core::prelude::FaceletPosition::*;
use crate::core::prelude::*;
use array_const_fn_init::array_const_fn_init;
use strum::EnumCount;
use strum_macros::*;

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone, Default, Hash)]
pub struct CoordinateCube {
    pub flip: u16,
    pub twist: u16,
    pub slice_sorted: u16,
    pub corners: u16,
    pub u_edges: u16,
    pub d_edges: u16,
    pub phase: PhaseData,
}

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone, Default, Hash)]
pub enum PhaseData {
    Phase1 {
        flip_slice_twist_depth_mod3: u8,
    },
    Phase2 {
        cornslice_depth: u8,
        corners_ud_edges_depth_mod3: u8,
    },
    #[default]
    Solved,
}


impl CoordinateCube {
    pub fn create(
        flip: u16,
        twist: u16,
        slice_sorted: u16,
        corners: u16,
        u_edges: u16,
        d_edges: u16,
        data_source: &DataSource,
    ) -> Self {
        let slice = slice_sorted / NPERM4;

        let phase: PhaseData = if (flip != 0 || slice != 0 || twist != 0) {
            let flip_slice_twist_depth_mod3 =
                data_source.get_flip_slice_twist_depth_mod_3(flip, twist, slice_sorted);
            PhaseData::Phase1 {
                flip_slice_twist_depth_mod3,
            }
        } else {
            let ud_edges = data_source.get_ud_edges(u_edges, d_edges);

            if corners == 0 && slice_sorted == 0 && ud_edges == 0 {
                PhaseData::Solved
            } else {
                let corners_ud_edges_depth_mod3 =
                    data_source.get_corners_ud_edges_depth_3(corners, ud_edges);
                let cornslice_depth = data_source.get_cornslice_depth(corners, slice_sorted);

                PhaseData::Phase2 {
                    cornslice_depth,
                    corners_ud_edges_depth_mod3,
                }
            }
        };

        Self {
            flip,
            twist,
            slice_sorted,
            corners,
            u_edges,
            d_edges,
            phase,
        }
    }


    pub fn after_move(&self, m: Move, data_source: &DataSource,)-> Self{
        let mu = m as usize;

        let flip = data_source.moves_source.flip_move[(NMOVE * self.flip) as usize + mu];
        let twist  = data_source.moves_source.twist_move[(NMOVE * self.twist) as usize + mu];
        let slice_sorted  = data_source.moves_source.slice_sorted_move[(NMOVE * self.slice_sorted) as usize + mu];
        let corners  = data_source.moves_source.corners_move[(NMOVE * self.corners) as usize + mu];
        let u_edges  = data_source.moves_source.u_edges_move[(NMOVE * self.u_edges) as usize + mu];
        let d_edges  = data_source.moves_source.d_edges_move[(NMOVE * self.d_edges) as usize + mu];

        CoordinateCube::create(flip, twist, slice_sorted, corners, u_edges, d_edges, data_source)
    }



    
}
