use crate::core::prelude::*;

use strum::EnumCount;
impl CubieCube {
    ///Gets the corner parity
    /// Either 0 or 1
    /// A cube should have the same corner and edge parity
    pub fn get_corner_parity(&self) -> u8 {
        calculate_position_parity(&self.corner_positions)
    }

    /// Gets the edge parity
    /// Either 0 or 1
    /// A cube should have the same corner and edge parity
    pub fn get_edge_parity(&self) -> u8 {
        calculate_position_parity(&self.edge_positions)
    }

    /// Gets the twist of the 8 corners
    /// 0..2186 in phase 1
    /// 0 in phase 2 - this means that all corners are in the solved orientation
    pub fn get_twist(&self) -> u16 {
        calculate_number_representation::<CornerOrientation, 8>(&self.corner_orientations, 3) as u16
    }

    /// Gets the flip of the 12 edges
    /// 0..2048 in phase 1
    /// 0in phase 2 - this means that all edges are in the solved orientation
    pub fn get_flip(&self) -> u16 {
        calculate_number_representation::<EdgeOrientation, 12>(&self.edge_orientations, 2) as u16
    }

    /// Gets a number representing the locations of the middle layer edges (FR, FL, BL, BR)
    /// 0..=494 in phase 1
    /// 0 in phase 2 - this means that all four middle layer edges are in the middle layer
    pub fn get_slice(&self) -> u16 {
        let mut a = 0;
        let mut x = 0;

        for j in (0..EdgePosition::COUNT).rev() {
            if self.edge_positions[j] >= EdgePosition::Fr {
                //We are only interested on the four middle layer edges
                a += binomial(11 - j, x + 1);
                x += 1;
            }
        }
        a as u16
    }

    /// Gets a number representing the permutations and locations of the middle layer edges (FR, FL, BL, BR)
    /// 0..11880 in phase 1
    /// Slice is 0..24 in phase 2 - this means that all four middle layer edges are in the middle layer and in one of 24 permutations
    /// Slie is 0 in the solved cube
    pub fn get_slice_sorted(&self) -> u16 {
        self.get_edges(8, 0)
    }

    /// Gets a number representing the permutations and locations of the up edges
    /// 0..11880 in phase 1
    /// 0..1680 in phase 2
    /// 1656 for solved cube
    pub fn get_u_edges(&self) -> u16 {
        self.get_edges(0, 4)
    }

    /// Gets a number representing the permutations and locations of the down edges
    /// 0..11880 in phase 1
    /// 0..1680 in phase 2
    /// 1656 for solved cube
    pub fn get_d_edges(&self) -> u16 {
        self.get_edges(4, 4)
    }

    /// Gets a number representing the permutations and locations of a block of four edges
    fn get_edges(&self, offset: usize, rot_right: usize) -> u16 {
        let mut a = 0;
        let mut x: usize = 0;

        let mut edge4: [usize; 4] = [0; 4];
        let ep_mod: [EdgePosition; 12];
        if rot_right > 0 {
            let mut epm = self.edge_positions;
            epm.rotate_right(rot_right);
            ep_mod = epm;
        } else {
            ep_mod = self.edge_positions;
        }

        for j in (0..EdgePosition::COUNT).rev() {
            let edge = ep_mod[j] as usize;

            if edge >= offset && edge < offset + 4 {
                //We are only interested in our four edges
                a += binomial(11 - j, x + 1);
                edge4[3 - x] = edge;
                x += 1;
            }
        }

        let b = calculate_permutation(edge4, offset) as u16;

        (a * 24) as u16 + b
    }

    /// Gets a number representing the permutation of the U and D edges
    /// None in phase 1
    /// 0..40320
    pub fn get_ud_edges(&self) -> Option<u16> {
        if self
            .edge_positions
            .into_iter()
            .take(8)
            .any(|x| x >= EdgePosition::Fr)
        {
            return None; //One of the middle layer edges is in a different layer -> We are not in phase 1
        }

        (calculate_permutation::<EdgePosition, 8>(self.edge_positions[0..8].try_into().unwrap(), 0)
            as u16)
            .into()
    }

    ///Get a number representing the permutations of the 8 corners
    /// Between 0 and 40319
    /// 0 for solved cube
    pub fn get_corners(&self) -> u16 {
        calculate_permutation(self.corner_positions, 0) as u16
    }
}
