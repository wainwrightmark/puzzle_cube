use crate::core::prelude::*;
use chrono::offset;
use strum::{EnumCount, IntoEnumIterator};

impl CubieCube {

/// Gets a number representing the permutations and locations of the middle layer edges (FR, FL, BL, BR)
    /// Slice is in 0..=11879 in phase 1
    /// Slice is 0..24 in phase 2 - this means that all four middle layer edges are in the middle layer and in one of 24 permutations
    /// Slie is 0 in the solved cube
    pub fn get_slice_sorted(&self) -> u16{
        self.get_edges(8)
    }

    /// Gets a number representing the permutations and locations of the up edges
    /// 0..=11879 in phase 1
    /// 0..=1679 in phase 2
    /// 1656 for solved cube
    pub fn get_u_edges(&self) -> u16{
        self.get_edges(0)
    }

    /// Gets a number representing the permutations and locations of the down edges
    /// 0..=11879 in phase 1
    /// 0..=1679 in phase 2
    /// 1656 for solved cube
    pub fn get_d_edges(&self) -> u16{
        self.get_edges(4)
    }



    /// Gets a number representing the permutations and locations of a block of four edges
    fn get_edges(&self, offset:usize)->u16{
        let mut a = 0;
        let mut x: usize = 0;

        let mut edge4:[usize;4] = [0; 4];

        for j in (0..EdgePosition::COUNT).rev(){

            let edge = self.edge_positions[j] as usize;

            if edge >= offset && edge < offset + 4{ //We are only interested in our four edges
                a += binomial(11 - (j as u16), (x as u16) + 1);
                edge4[3 - x] = edge;
                x += 1;
            }   
        }

        let b = calculate_permutation(edge4, offset) as u16;

        (a * 24) + b
    }

    ///Gets the corner parity
    /// Either 0 or 1
    /// A cube should have the same corner and edge parity
    pub fn get_corner_parity(&self) -> u8 {
        let mut s = 0;
        for i in (1..CornerPosition::COUNT) {
            for j in (0..=i - 1) {
                if self.corner_positions[j] > self.corner_positions[i] {
                    s += 1;
                }
            }
        }
        s % 2
    }

    /// Gets the edge parity
    /// Either 0 or 1
    /// A cube should have the same corner and edge parity
    pub fn get_edge_parity(&self) -> u8{
        let mut s = 0;
        for i in (1..EdgePosition::COUNT) {
            for j in (0..=i - 1) {
                if self.edge_positions[j] > self.edge_positions[i] {
                    s += 1;
                }
            }
        }
        s % 2
    }

    /// Gets the twist of the 8 corners
    /// The twist is in 0..2186 in phase 1
    /// The twist is 0 in phase 2 - this means that all corners are in the solved orientation
    pub fn get_twist(&self)->u16{
        let mut t = 0;

        for ori in self.corner_orientations.into_iter().take(7){
            t = (3 * t) + ori as u16;
        }
        t
    }

    /// Gets the flip of the 12 edges
    /// The flip is in 0..=2047 in phase 1
    /// The flip is zero in phase 2 - this means that all edges are in the solved orientation
    pub fn get_flip(&self)-> u16{
        let mut f = 0;

        for ori in self.edge_orientations.into_iter().take(11){
            f = (2 * f) + ori as u16;
        }
        f
    }

    /// Gets a number representing the locations of the middle layer edges (FR, FL, BL, BR)
    /// Slice is in 0..=494 in phase 1
    /// Slice is 0 in phase 2 - this means that all four middle layer edges are in the middle layer
    pub fn get_slice(&self) -> u16{
        let mut a = 0;
        let mut x = 0;

        for j in (0..EdgePosition::COUNT).rev(){
            if self.edge_positions[j] >= EdgePosition::Fr{ //We are only interested on the four middle layer edges
                a += binomial(11 - (j as u16), x + 1);
                x += 1;
            }   
        }
        a
    }

    


    ///Get a number representing the permutations of the 8 corners
    /// Between 0 and 40319
    /// 0 for solved cube
    pub fn get_corners(&self) -> u16 {

        calculate_permutation(self.corner_positions, 0) as u16
    }
}