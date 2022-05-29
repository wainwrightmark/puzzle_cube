use crate::core::prelude::*;
use array_const_fn_init::array_const_fn_init;
use itertools::Itertools;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::*;
use rand::{SeedableRng, prelude::StdRng, Rng};

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
pub struct CubieCube {
    pub edge_positions: [EdgePosition; 12],
    pub corner_positions: [CornerPosition; 8],

    pub edge_orientations: [EdgeOrientation; 12],
    pub corner_orientations: [CornerOrientation; 8],
}

impl Default for CubieCube {
    fn default() -> Self {
        CubieCube::default()
    }
}

impl CubieCube {
    pub const fn default() -> Self {
        Self {
            edge_positions: EdgePosition::DEFAULT_ARRAY,
            corner_positions: CornerPosition::DEFAULT_ARRAY,
            edge_orientations: [EdgeOrientation::Zero; 12],
            corner_orientations: [CornerOrientation::Zero; 8],
        }
    }

    const ARRAYEIGHT: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

    ///Multiply this cube with another cube.
    const fn multiply_corners(&self, other: &Self) -> ([CornerPosition; CornerPosition::COUNT], [CornerOrientation; CornerPosition::COUNT]) {
        let mut corner_positions = [CornerPosition::Urf; CornerPosition::COUNT];
        let mut corner_orientations = [CornerOrientation::Zero; CornerPosition::COUNT];
        let mut c = 0;

        while c < CornerPosition::COUNT {
            let other_c = other.corner_positions[c] as usize;
            corner_positions[c] = self.corner_positions[other_c];

            let ori_a = self.corner_orientations[other_c] as u8;
            let ori_b = other.corner_orientations[c] as u8;

            let ori = if ori_a < 3 {
                if ori_b < 3 {
                    (ori_a + ori_b) % 3
                } else {
                    ((ori_a + ori_b - 3) % 3) + 3
                }
            } else if ori_b < 3 {
                ((ori_a - ori_b) % 3) + 3
            } else {
                ((3 + ori_a - ori_b) % 3)
            };
            corner_orientations[c] = CornerOrientation::from_repr(ori).unwrap();

            c += 1;
        }

        (corner_positions,
            corner_orientations)
    }


    const fn multiply_edges(&self, other: &Self) -> ([EdgePosition; EdgePosition::COUNT], [EdgeOrientation; EdgePosition::COUNT]) {
        let mut edge_positions = [EdgePosition::Ur; EdgePosition::COUNT];
        let mut edge_orientations = [EdgeOrientation::Zero; EdgePosition::COUNT];
        let mut e = 0;

        while e < EdgePosition::COUNT {
            let other_e = other.edge_positions[e] as usize;
            edge_positions[e] = self.edge_positions[other_e];

            let ori_a = self.edge_orientations[other_e] as u8;
            let ori_b = other.edge_orientations[e] as u8;

            let ori = (ori_a + ori_b) % 2;
            edge_orientations[e] = EdgeOrientation::from_repr(ori).unwrap();

            e += 1;
        }

        (edge_positions,
            edge_orientations)
    }

    pub const fn corner_multiply(self, other: &Self) -> Self {
        let (corner_positions, corner_orientations) = self.multiply_corners(other);
        Self{
            corner_positions,
            corner_orientations,
            ..self
        }
    }
    
    pub const fn edge_multiply(self, other: &Self) -> Self {
        let (edge_positions, edge_orientations) = self.multiply_edges(other);
        Self{
            edge_positions,
            edge_orientations,
            ..self
        }
    }

    pub const fn multiply(&self, other: &Self) -> Self {

        let (corner_positions, corner_orientations) = self.multiply_corners(other);
        let (edge_positions, edge_orientations) = self.multiply_edges(other);

        Self { edge_positions, corner_positions, edge_orientations, corner_orientations }
    }

    pub const fn invert(&self) -> Self {
        let mut edge_positions = [EdgePosition::Ur; EdgePosition::COUNT];
        let mut edge_orientations = [EdgeOrientation::Zero; EdgePosition::COUNT];
        let mut corner_positions = [CornerPosition::Urf; CornerPosition::COUNT];
        let mut corner_orientations = [CornerOrientation::Zero; CornerPosition::COUNT];

        let mut index = 0;
        while (index < EdgePosition::COUNT) {
            let i = self.edge_positions[index] as usize;
            edge_positions[i] = EdgePosition::from_repr(index as u8).unwrap();
            index += 1;
        }

        index = 0;
        while (index < EdgePosition::COUNT) {
            let i = edge_positions[index] as usize; //note: not self.edge_positions
            let ori = self.edge_orientations[i];
            edge_orientations[index] = ori;
            index += 1;
        }
        
        index = 0;
        while (index < CornerPosition::COUNT) {
            let i = self.corner_positions[index] as usize;
            corner_positions[i] = CornerPosition::from_repr(index as u8).unwrap();
            index += 1;
        }
        
        index = 0;
        while (index < CornerPosition::COUNT) {
            let i = corner_positions[index] as usize; //note: not self.corner_positions
            let ori = self.corner_orientations[i] as u8;
            let new_ori = match ori {
                1..=2 => 3 - ori,
                _ => ori,
            };
            corner_orientations[index] = CornerOrientation::from_repr(new_ori).unwrap();
            index += 1;
        }

        CubieCube {
            edge_positions,
            corner_positions,
            edge_orientations,
            corner_orientations,
        }
    }

    pub fn verify(&self)->Result<(), &str>{
        let unique_edges = self.edge_positions.into_iter().dedup().count();
        if unique_edges < EdgePosition::COUNT{
            return Err("There are duplicate edges");
        }
        
        let unique_corners = self.corner_positions.into_iter().dedup().count();
        if unique_corners < CornerPosition::COUNT{
            return Err("There are duplicate corners");
        }
        
        let edge_flip:u8 = self.edge_orientations.into_iter().map(|x|x as u8) .sum();
        if edge_flip % 2 != 0{
            return Err("Total Edge flip is wrong");
        }

        let corner_twist:u8 = self.corner_orientations.into_iter().map(|x|x as u8).sum();
        if corner_twist % 3 != 0{
            return Err("Total Corner twist is wrong");
        }

        let edge_parity = self.get_edge_parity();
        let corner_parity = self.get_corner_parity();

        if edge_parity != corner_parity{
            return Err("Edge and corner parities are not equal");
        }

        Ok(())
    }

    pub fn random_cube(seed: u64) -> Self{
        let mut cube = CubieCube::default();
        
        let mut rng: StdRng = rand::SeedableRng::seed_from_u64(seed);
        
        cube.set_edges(rng.gen_range(0..479001600));
        let edge_parity = cube.get_edge_parity();
        loop {
            cube.set_corners(rng.gen_range(0..40320));
            let corner_parity = cube.get_corner_parity();
            if edge_parity == corner_parity{
                break;
            }            
        }

        cube.set_flip(rng.gen_range(0..2048));
        cube.set_twist(rng.gen_range(0..2187));

        cube
    }
}

impl Into<FaceletCube> for CubieCube {
    fn into(self) -> FaceletCube {
        let mut facelets = [FaceColor::Up; 54];

        //set corner colors
        for c in 0..CornerPosition::COUNT {
            let corner = self.corner_positions[c];
            let ori = self.corner_orientations[c] as usize;

            for k in 0..3 {
                let i = CornerPosition::CORNERFACELETS[c][(k + ori) % 3] as usize;
                let color = CornerPosition::CORNERCOLORS[c][k];
                facelets[i] = color;
            }
        }

        //set edge colors
        for e in 0..EdgePosition::COUNT {
            let edge = self.edge_positions[e];
            let ori = self.edge_orientations[e] as usize;

            for k in 0..2 {
                let i = EdgePosition::EDGEFACELETS[e][(k + ori) % 2] as usize;
                let color = EdgePosition::EDGECOLORS[e][k];

                facelets[i] = color;
            }
        }

        for c in FaceColor::iter() {
            let centre = 4 + ((c as usize) * 9);
            facelets[centre] = c;
        }

        FaceletCube { facelets }
    }
}
