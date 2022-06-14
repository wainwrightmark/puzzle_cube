use std::default;

use crate::core::prelude::FaceColor::*;
use crate::core::prelude::FaceletPosition::*;
use crate::core::prelude::*;
use array_const_fn_init::array_const_fn_init;
use strum::EnumCount;
use strum_macros::*;

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone, Default, Hash, PartialOrd, Ord)]
pub struct CoordinateCube {
    pub flip: u16,
    pub twist: u16,
    pub slice_sorted: u16,
    pub corners: u16,
    pub u_edges: u16,
    pub d_edges: u16,
    //pub phase: PhaseData,
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

impl From<CubieCube> for CoordinateCube{
    fn from(cube: CubieCube) -> Self {
        Self { flip: cube.get_flip(), twist: cube.get_twist(), slice_sorted: cube.get_slice_sorted(), corners: cube.get_corners(), u_edges: cube.get_u_edges(), d_edges: cube.get_d_edges() }
    }
}

impl CoordinateCube {
    pub fn create_phase_data(&self, data_source: &DataSource) -> PhaseData {
        let slice = self.slice_sorted / 24;

        if (self.flip != 0 || slice != 0 || self.twist != 0) {
            let flip_slice_twist_depth_mod3 = data_source.get_flip_slice_twist_depth_mod_3(
                self.flip,
                self.twist,
                self.slice_sorted,
            );
            PhaseData::Phase1 {
                flip_slice_twist_depth_mod3,
            }
        } else {
            let ud_edges = data_source.get_ud_edges(self.u_edges, self.d_edges);

            if self.corners == 0 && self.slice_sorted == 0 && ud_edges == 0 {
                PhaseData::Solved
            } else {
                let corners_ud_edges_depth_mod3 =
                    data_source.get_corners_ud_edges_depth_3(self.corners, ud_edges);
                let cornslice_depth =
                    data_source.get_cornslice_depth(self.corners, self.slice_sorted);

                PhaseData::Phase2 {
                    cornslice_depth,
                    corners_ud_edges_depth_mod3,
                }
            }
        }
    }


    pub fn after_move(&self, m: Move, moves_source: &MovesSource) -> Self {
        let mu = m as usize;

        let flip = moves_source.flip_move[(Move::COUNT * (self.flip as usize))+ mu];
        let twist = moves_source.twist_move[(Move::COUNT * (self.twist as usize))+ mu];
        let slice_sorted =
            moves_source.slice_sorted_move[(Move::COUNT * (self.slice_sorted as usize))+ mu];
        let corners = moves_source.corners_move[(Move::COUNT * (self.corners as usize))+ mu];
        let u_edges = moves_source.u_edges_move[(Move::COUNT * (self.u_edges as usize))+ mu];
        let d_edges = moves_source.d_edges_move[(Move::COUNT * (self.d_edges as usize))+ mu];

        CoordinateCube {
            flip,
            twist,
            slice_sorted,
            corners,
            u_edges,
            d_edges,
        }
    }
}
