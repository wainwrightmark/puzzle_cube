use std::default;
use std::vec;

use crate::core::prelude::FaceColor::*;
use crate::core::prelude::FaceletPosition::*;
use crate::core::prelude::*;
use array_const_fn_init::array_const_fn_init;
use strum_macros::*;
use strum::{EnumCount, IntoEnumIterator};

pub trait CubeProperty<const NMOVES:usize, const NVALUES: usize> {
    fn is_edges(&self)->bool;
    fn defined_moves(&self)->[Move; NMOVES];

    fn get_value(&self, cube: &CubieCube)-> u16;
    fn set_value(&self, cube: &mut CubieCube, value: u16);

    fn create(&self)-> Vec<u16>{

        //let mut array: [u16; NVALUES * NMOVES] = [u16::default(); NVALUES * NMOVES];
        let mut v = Vec::<u16>::new();
        v.reserve_exact(NVALUES * NMOVES);

        for value in 0..NVALUES{

            let mut cube = CubieCube::default();
            self.set_value(&mut cube, value as u16);
            for m_index in 0..NMOVES{
                let m = self.defined_moves()[m_index];
                let applied = if self.is_edges()
                {
                    m.apply_edges(&cube)
                }
                else{
                    m.apply_corners(&cube)
                };                

                let new_value = self.get_value(&applied);

                v.push(new_value);
                //let index = value * NMOVES  + m_index;
                //v[index] = new_value;
            }
        }

        v
        //array.into()
    }
    
    // fn create2(&self)-> [u16; NVALUES * NMOVES]{

    //     let mut array: [u16; NVALUES * NMOVES] = [u16::default(); NVALUES * NMOVES];

    //     array
    // }
}

pub struct SliceProperty {}

impl CubeProperty< 18, 495> for SliceProperty {
    fn is_edges(&self)->bool {
        true
    }

    fn defined_moves(&self)->[Move; 18] {
        Move::ALLMOVES
    }

    fn get_value(&self, cube: &CubieCube)-> u16 {
        cube.get_slice()
    }

    fn set_value(&self, cube: &mut CubieCube, value: u16) {
        cube.set_slice(value)
    }
}

pub struct FlipProperty {}

impl CubeProperty< 18, 2048> for FlipProperty {
    fn is_edges(&self)->bool {
        true
    }

    fn defined_moves(&self)->[Move; 18] {
        Move::ALLMOVES
    }

    fn get_value(&self, cube: &CubieCube)-> u16 {
        cube.get_flip()
    }

    fn set_value(&self, cube: &mut CubieCube, value: u16) {
        cube.set_flip(value)
    }
}

pub struct DownEdgesProperty {}

impl CubeProperty< 18, 11880> for DownEdgesProperty {
    fn is_edges(&self)->bool {
        true
    }

    fn defined_moves(&self)->[Move; 18] {
        Move::ALLMOVES
    }

    fn get_value(&self, cube: &CubieCube)-> u16 {
        cube.get_d_edges()
    }

    fn set_value(&self, cube: &mut CubieCube, value: u16) {
        cube.set_d_edges(value)
    }
}

pub struct UpEdgesProperty {}

impl CubeProperty<18, 11880> for UpEdgesProperty {
    fn is_edges(&self)->bool {
        true
    }

    fn defined_moves(&self)->[Move; 18] {
        Move::ALLMOVES
    }

    fn get_value(&self, cube: &CubieCube)-> u16 {
        cube.get_u_edges()
    }

    fn set_value(&self, cube: &mut CubieCube, value: u16) {
        cube.set_u_edges(value)
    }
}

pub struct UpDownEdgesProperty {}

impl CubeProperty< 10, 40320> for UpDownEdgesProperty {
    fn is_edges(&self)->bool {
        true
    }

    fn defined_moves(&self)->[Move; 10] {
        Move::PHASE2MOVES
    }

    fn get_value(&self, cube: &CubieCube)-> u16 {
        cube.get_ud_edges().unwrap()
    }

    fn set_value(&self, cube: &mut CubieCube, value: u16) {
        cube.set_ud_edges(value)
    }
}

pub struct SliceSortedProperty {}

impl CubeProperty< 18, 11880> for SliceSortedProperty {
    fn is_edges(&self)->bool {
        true
    }

    fn defined_moves(&self)->[Move; 18] {
        Move::ALLMOVES
    }

    fn get_value(&self, cube: &CubieCube)-> u16 {
        cube.get_slice_sorted()
    }

    fn set_value(&self, cube: &mut CubieCube, value: u16) {
        cube.set_slice_sorted(value)
    }
}

pub struct CornersProperty {}

impl CubeProperty<18, 40320> for CornersProperty {
    fn is_edges(&self)->bool {
        false
    }

    fn defined_moves(&self)->[Move; 18] {
        Move::ALLMOVES
    }

    fn get_value(&self, cube: &CubieCube)-> u16 {
        cube.get_corners()
    }

    fn set_value(&self, cube: &mut CubieCube, value: u16) {
        cube.set_corners(value)
    }
}

pub struct TwistProperty {}

impl CubeProperty<18, 2187> for TwistProperty {
    fn is_edges(&self)->bool {
        false
    }

    fn defined_moves(&self)->[Move; 18] {
        Move::ALLMOVES
    }

    fn get_value(&self, cube: &CubieCube)-> u16 {
        cube.get_twist()
    }

    fn set_value(&self, cube: &mut CubieCube, value: u16) {
        cube.set_twist(value)
    }
}



