use crate::core::prelude::*;
use array_const_fn_init::array_const_fn_init;
use itertools::Itertools;
use serde_with::*;
use strum::EnumCount;
use strum::IntoEnumIterator;

#[serde_as]
#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)]
pub struct FaceletCube {
    #[serde_as(as = "[_; 54]")]
    pub facelets: [Option<FaceColor>; 54],
}

const fn get_cleared_face(i: usize) -> Option<FaceColor> {
    if i % 9 == 4 {
        Some(match i / 9 {
            0 => FaceColor::Up,
            1 => FaceColor::Right,
            2 => FaceColor::Front,
            3 => FaceColor::Down,
            4 => FaceColor::Left,
            _ => FaceColor::Back,
        })
    } else {
        None
    }
}

impl FaceletCube {
    pub const CLEARED: FaceletCube = FaceletCube {
        facelets: array_const_fn_init![get_cleared_face; 54],
    };

    ///checks that there is the correct number of each color
    pub fn validate_colors(&self) -> Result<(), String> {
        let counts = self.facelets.into_iter().counts_by(|f| f);

        for c in counts {
            if let Some(color) = c.0 {
                if c.1 != 9 {
                    return Err(format!("{} appears {} times.", color, c.1));
                }
            } else {
                return Err("Not all colors set".to_string());
            }
        }

        Ok(())
    }
}

impl TryFrom<FaceletCube> for CubieCube {
    type Error = String;

    fn try_from(cube: FaceletCube) -> Result<Self, Self::Error> {
        if let Err(e) = cube.validate_colors() {
            return Err(e);
        }

        let mut edge_positions = [EdgePosition::Ur; 12];
        let mut corner_positions = [CornerPosition::Urf; 8];

        let mut edge_orientations = [EdgeOrientation::Zero; 12];
        let mut corner_orientations = [CornerOrientation::Zero; 8];

        for c in CornerPosition::DEFAULT_ARRAY {
            let fac = CornerPosition::CORNERFACELETS[c as usize];
            let ori = (0..3)
                .find(|&i: &usize| {
                    cube.facelets[fac[i] as usize] == Some(FaceColor::Up)
                        || cube.facelets[fac[i] as usize] == Some(FaceColor::Down)
                })
                .ok_or("Bad Corner")?;

            let col1 = cube.facelets[fac[(ori + 1) % 3] as usize];
            let col2 = cube.facelets[fac[(ori + 2) % 3] as usize];

            let j = CornerPosition::DEFAULT_ARRAY
                .into_iter()
                .find(|&j| {
                    let col = CornerPosition::CORNERCOLORS[j as usize];
                    col1 == Some(col[1]) && col2 == Some(col[2])
                })
                .ok_or("Bad Corner")?;

            corner_positions[c as usize] = j;
            corner_orientations[c as usize] = CornerOrientation::from_repr(ori as u8).unwrap();
        }

        for e in EdgePosition::DEFAULT_ARRAY {
            let fac = EdgePosition::EDGEFACELETS[e as usize];

            let j = EdgePosition::DEFAULT_ARRAY
                .into_iter()
                .filter_map(|j| {
                    let c = EdgePosition::EDGECOLORS[j as usize];
                    let c0 = c[0];
                    let c1 = c[1];

                    if cube.facelets[fac[0] as usize] == Some(c0)
                        && cube.facelets[fac[1] as usize] == Some(c1)
                    {
                        Some((j, 0))
                    } else if cube.facelets[fac[0] as usize] == Some(c1)
                        && cube.facelets[fac[1] as usize] == Some(c0)
                    {
                        Some((j, 1))
                    } else {
                        None
                    }
                })
                .next()
                .ok_or("bad edge")?;

            edge_positions[e as usize] = j.0;
            edge_orientations[e as usize] = EdgeOrientation::from_repr(j.1).unwrap();
        }

        Ok(CubieCube {
            corner_positions,
            corner_orientations,
            edge_orientations,
            edge_positions,
        })
    }
}

impl From<CubieCube> for FaceletCube {
    fn from(cube: CubieCube) -> Self {
        let mut facelets: [Option<FaceColor>; 54] = [None; 54];

        //set corner colors
        for c in 0..CornerPosition::COUNT {
            let corner = cube.corner_positions[c];
            let ori = cube.corner_orientations[c] as usize;

            for k in 0..3 {
                let i = CornerPosition::CORNERFACELETS[c][(k + ori) % 3] as usize;
                let color = CornerPosition::CORNERCOLORS[corner as usize][k];
                facelets[i] = Some(color);
            }
        }

        //set edge colors
        for e in 0..EdgePosition::COUNT {
            let edge = cube.edge_positions[e];

            let ori = cube.edge_orientations[e] as usize;

            for k in 0..2 {
                let i = EdgePosition::EDGEFACELETS[e][(k + ori) % 2] as usize;

                let color = EdgePosition::EDGECOLORS[edge as usize][k];

                facelets[i] = Some(color);
            }
        }

        for c in FaceColor::iter() {
            let centre = 4 + ((c as usize) * 9);
            facelets[centre] = Some(c);
        }

        FaceletCube { facelets }
    }
}

impl Default for FaceletCube {
    fn default() -> Self {
        CubieCube::default().into()
    }
}
