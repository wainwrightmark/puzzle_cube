use crate::core::prelude::*;
use array_const_fn_init::array_const_fn_init;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::*;

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
            edge_positions: EdgePosition::default_array(),
            corner_positions: CornerPosition::default_array(),
            edge_orientations: [EdgeOrientation::Zero; 12],
            corner_orientations: [CornerOrientation::Zero; 8],
        }
    }

    const ARRAYEIGHT: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    const ARRAYTWELVE: [usize; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    ///Multiply this cube with another cube.
    fn corner_multiply(self, other: &Self) -> Self {
        let corner_positions = CubieCube::ARRAYEIGHT.map(|c| {
            let other_c = other.corner_positions[c] as usize;
            self.corner_positions[other_c]
        });
        let corner_orientations =
            CubieCube::ARRAYEIGHT.map(|c| self.calc_corner_multiply_orientation(c, &other)); //defaults

        Self {
            corner_orientations,
            corner_positions,
            ..self
        }
    }

    const fn calc_corner_multiply_orientation(&self, c: usize, other: &Self) -> CornerOrientation {
        let other_c = other.corner_positions[c] as usize;
        let ori_a = self.corner_orientations[other_c] as u8;
        let ori_b = other.corner_orientations[c] as u8;

        let ori = if ori_a < 3 {
            if ori_b < 3 {
                (ori_a + ori_b) % 3
            } else {
                ((ori_a + ori_b - 3) % 3) + 3
            }
        } else {
            if ori_b < 3 {
                ((ori_a - ori_b) % 3) + 3
            } else {
                ((3 + ori_a - ori_b) % 3)
            }
        };
        CornerOrientation::from_repr(ori).unwrap()
    }

    fn edge_multiply(self, other: &Self) -> Self {
        let edge_positions = CubieCube::ARRAYTWELVE.map(|e| {
            let other_e = other.edge_positions[e] as usize;
            self.edge_positions[other_e]
        });

        let edge_orientations = CubieCube::ARRAYTWELVE.map(|e| {
            let other_e = other.edge_positions[e] as usize;
            let ori_a = self.corner_orientations[other_e] as u8;
            let ori_b = other.corner_orientations[e] as u8;

            let ori = ori_a + ori_b % 2;
            EdgeOrientation::from_repr(ori).unwrap()
        });

        Self {
            edge_orientations,
            edge_positions,
            ..self
        }
    }
    pub fn multiply(self, other: &Self) -> Self {
        self.corner_multiply(other).edge_multiply(other)
    }

    pub fn set_twist(&mut self, mut twist: usize) {
        let mut twist_parity = 0;

        for i in (0..7).rev() {
            self.corner_orientations[i] = CornerOrientation::from_repr((twist % 3) as u8).unwrap();
            twist_parity += twist % 3;
            twist /= 3;
        }

        self.corner_orientations[7] =
            CornerOrientation::from_repr(((3 - (twist_parity % 3)) % 3) as u8).unwrap();
    }

    pub fn set_flip(&mut self, mut flip: usize) {
        let mut flip_parity = 0;

        for i in (0..11).rev() {
            self.edge_orientations[i] = EdgeOrientation::from_repr((flip % 2) as u8).unwrap();
            flip_parity += flip % 2;
            flip /= 2;
        }

        self.edge_orientations[11] =
            EdgeOrientation::from_repr(((2 - (flip_parity % 2)) % 2) as u8).unwrap();
    }

    pub fn invert(self) -> Self {
        let mut edge_positions = [EdgePosition::Ur; EdgePosition::COUNT];
        let mut edge_orientations = [EdgeOrientation::Zero; EdgePosition::COUNT];
        let mut corner_positions = [CornerPosition::Urf; CornerPosition::COUNT];
        let mut corner_orientations = [CornerOrientation::Zero; CornerPosition::COUNT];

        for e in EdgePosition::iter() {
            let i = self.edge_positions[e as usize] as usize;
            let ori = self.edge_orientations[i];
            edge_positions[i] = e;
            edge_orientations[e as usize] = ori;
        }

        for c in CornerPosition::iter() {
            let i = self.corner_positions[c as usize] as usize;
            let ori = self.corner_orientations[i] as u8;
            let new_ori = match ori {
                1..=2 => 3 - ori,
                _ => ori,
            };

            corner_positions[i] = c;
            corner_orientations[c as usize] = CornerOrientation::from_repr(new_ori).unwrap();
        }

        CubieCube {
            edge_positions,
            corner_positions,
            edge_orientations,
            corner_orientations,
        }
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
