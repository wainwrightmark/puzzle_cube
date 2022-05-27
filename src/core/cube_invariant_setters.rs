use crate::core::prelude::*;
use num::ToPrimitive;

use strum::{EnumCount, IntoEnumIterator};
impl CubieCube {


    pub fn set_twist(&mut self, twist: u16) {
        set_from_number_representation
        ::<CornerOrientation, {CornerPosition::COUNT}, 3>
        (&mut self.corner_orientations, twist as usize)
    }

    pub fn set_flip(&mut self, flip: u16) {
        set_from_number_representation
        ::<EdgeOrientation, {EdgePosition::COUNT}, 2>
        (&mut self.edge_orientations, flip as usize)
    }

    pub fn set_corners(&mut self, corners: u16){
        let mut corner_positions = CornerPosition::DEFAULT_ARRAY;
        reorder_to_permutation::<CornerPosition, 8> (&mut corner_positions, corners as usize);
        self.corner_positions =corner_positions;
    }

    pub fn set_slice(&mut self, slice: u16){
        self.set_slice_sorted(slice * 24)
    }

    pub fn set_slice_sorted(&mut self, slice_sorted: u16){
        let mut a = slice_sorted.to_i32().unwrap() / 24;
        let permutation = slice_sorted % 24;

        let mut edges: [EdgePosition; 4] = [EdgePosition::Fr, EdgePosition::Fl, EdgePosition::Bl, EdgePosition::Br];
        reorder_to_permutation::<EdgePosition, 4> (&mut edges, permutation  as usize);

        let mut slice_x = 4;
        let mut other_x = 0;

        for j in 0..EdgePosition::COUNT{
            let a1: i32 = a - (binomial(11 - j, slice_x) as i32);

            if a1 >= 0{
                self.edge_positions[j] = edges[4 - slice_x];
                slice_x -= 1;
                a = a1;
            }
            else {
                self.edge_positions[j] = EdgePosition::DEFAULT_ARRAY[other_x];
                other_x +=1;
            }
        }
    }

    /// Sets the ud edges
    /// Caused an invalid cube if not in phase 2 - the final four edges must be in the final four positions
    pub fn set_ud_edges(&mut self, edges: u16){

        let mut edge_positions = EdgePosition::DEFAULT_NON_SLICE_EDGES;
        reorder_to_permutation::<EdgePosition, 8> (&mut edge_positions, edges as usize);        
        self.edge_positions[0..8].copy_from_slice(&edge_positions);
    }

    /// Sets the edges to one of the 12 factorial possibilities
    pub fn set_edges(&mut self, edges: usize){
        let mut edge_positions = EdgePosition::DEFAULT_ARRAY;
        reorder_to_permutation::<EdgePosition, 12> (&mut edge_positions, edges);        
        self.edge_positions.copy_from_slice(&edge_positions);
    }

    pub fn set_u_edges(&mut self, edges: u16){
        let mut a = edges.to_i32().unwrap() / 24;
        let permutation = edges % 24;

        let mut edges: [EdgePosition; 4] = [EdgePosition::Ur, EdgePosition::Uf, EdgePosition::Ul, EdgePosition::Ub];
        reorder_to_permutation::<EdgePosition, 4> (&mut edges, permutation  as usize);

        let mut slice_x = 4;
        let mut other_x = 0;

        for j in 0..EdgePosition::COUNT{
            let a1: i32 = a - (binomial(11 - j, slice_x) as i32);

            if a1 >= 0{
                self.edge_positions[j] = edges[4 - slice_x];
                slice_x -= 1;
                a = a1;
            }
            else {
                self.edge_positions[j] = EdgePosition::DEFAULT_ARRAY[other_x + 4];
                other_x +=1;
            }
        }

        self.edge_positions.rotate_left(4);
    }
    
    pub fn set_d_edges(&mut self, edges: u16){
        let mut a = edges.to_i32().unwrap() / 24;
        let permutation = edges % 24;

        let mut edges: [EdgePosition; 4] = [EdgePosition::Dr, EdgePosition::Df, EdgePosition::Dl, EdgePosition::Db];
        reorder_to_permutation::<EdgePosition, 4> (&mut edges, permutation  as usize);

        let mut slice_x = 4;
        let mut other_x = 0;

        for j in 0..EdgePosition::COUNT{
            let a1: i32 = a - (binomial(11 - j, slice_x) as i32);

            if a1 >= 0{
                self.edge_positions[j] = edges[4 - slice_x];
                slice_x -= 1;
                a = a1;
            }
            else {
                let other_index = if(other_x <4){other_x}else{other_x+ 4};
                self.edge_positions[j] = EdgePosition::DEFAULT_ARRAY[other_index];
                other_x +=1;
            }
        }

        self.edge_positions.rotate_left(4);
    }

    

}