use crate::core::prelude::*;
use array_const_fn_init::array_const_fn_init;
use itertools::Itertools;
use serde::__private::de;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::*;
use rand::{SeedableRng, prelude::StdRng, Rng};

impl DataSource {

    fn set_corners_ud_edges_depth3 (ix: usize, value: u32, tab: &mut Vec<u32>){
        let shift = (ix % 16) * 2;
        let base = ix >> 4;
        let mut tb = tab[base];
        tb &= !(3u32 << shift) & u32::MAX;
        tb |= value << shift;

        tab[base] = tb ;
    }

    fn get_corners_ud_edges_depth3 (ix:usize, tab: &Vec<u32>)->u32 {
        let mut y = tab[ix / 16];
        y >>= (ix % 16) * 2;
        (y & 3)
    }

    pub fn create_phase_2_edge_merge() -> Vec<u16>{
        let mut table = vec![u16::MIN; 24 *1680];   

        let mut cu = CubieCube::default();
        let mut cd = CubieCube::default();
        let mut cud_edge_positions = CubieCube::default().edge_positions;

        for i in 0..1680{
            cu.set_u_edges(i);
            for j in 0..70{
                cd.set_d_edges(j * 24);

                let mut invalid = false;

                for e in 0..8{
                    let position: EdgePosition ;

                    if cu.edge_positions[e].is_up(){
                        position = cu.edge_positions[e];
                    }
                    else if cd.edge_positions[e].is_down(){
                        position = cd.edge_positions[e];
                    }
                    else{
                        invalid = true;
                        break;
                    }
                    cud_edge_positions[e] = position;
                }

                if invalid {continue;}

                for k in 0..24{
                    cd.set_d_edges((j * 24) + k);
                    for e in 0..8{
                        if cu.edge_positions[e].is_up(){
                            cud_edge_positions[e] = cu.edge_positions[e];
                        }
                        else if cd.edge_positions[e].is_down(){
                            cud_edge_positions[e] = cd.edge_positions[e];
                        }
                    }

                    let mut cube = CubieCube::default();
                cube.edge_positions = cud_edge_positions;

                table[(24 * (i as usize)) + (k as usize)] = cube.get_ud_edges().unwrap();
                }
                
            }
        }

        table
    }

    pub fn create_phase_2_pruning(moves_source: &MovesSource, corners_source: &CornerSymmetriesSource) -> Vec<u32> {

        let c_sym = DataSource::make_corners_sym(corners_source);
        let mut table = vec![u32::MAX; 40320 * 2768 / 16];        

        let ud_edge = 0;
        Self::set_corners_ud_edges_depth3(0, 0, &mut table);
        let mut done = 1;
        let mut depth = 0u32;

        while depth < 10 {
            let depth3 = depth % 3;
            let mut idx = 0;
            let mult = if depth > 9{1} else{2};

            for c_class_index in 0..2768{
                let mut ud_edge: u16 = 0;
                while ud_edge < 40320 {
                    if idx % 16 == 0 && table[idx / 16] == u32::MAX && ud_edge < (40320 -16){
                        ud_edge+=16;
                        idx += 16;
                        continue;
                    }

                    if Self::get_corners_ud_edges_depth3(idx, &table) == depth3{
                        let corner = corners_source.corner_rep[c_class_index];

                        let mut move_index = 0;
                        for m in Move::PHASE2MOVES{
                            let udedge1a = moves_source.get_ud_edge(ud_edge, move_index);
                            let corner1 =  moves_source.get_corners(corner, m);

                            let c1_class_index = corners_source.corner_class_index[corner1 as usize];
                            let c1_symmetry = corners_source.corner_symmetry[corner1 as usize];

                            let udedge1_conj = moves_source.get_ud_edges_conj(udedge1a, c1_symmetry);

                            let idx1 = (40320 * (c1_class_index as usize)) + udedge1_conj as usize;

                            if Self::get_corners_ud_edges_depth3(idx1, &table) ==3{
                                Self::set_corners_ud_edges_depth3(idx1, (depth + 1) % 3, &mut table);
                                done+=1;

                                let mut sym = c_sym[c1_class_index as usize];
                                if sym != 1{
                                    for j in 1..16{
                                        sym >>=1;
                                        if sym % 2 ==1{
                                            let u_d_edge2 = moves_source.get_ud_edges_conj(udedge1_conj, j);

                                            let idx2 = (40320 * (c1_class_index as usize)) + u_d_edge2 as usize;

                                            if Self::get_corners_ud_edges_depth3(idx2, &table) == 3{
                                                Self::set_corners_ud_edges_depth3(idx2, (depth + 1) % 3, &mut table);
                                                done +=1;
                                            }
                                        }
                                    }
                                }
                            }
                            move_index+=1;
                        }
                    }
                    ud_edge+=1;
                    idx+=1;
                }
            }
            depth +=1;
        }

        table
    }

    fn make_corners_sym(corners_source: &CornerSymmetriesSource)->[u16;2768] {
        let mut cc = CubieCube::default();
        let mut c_sym = [u16::MIN; 2768];
        for i in 0..2768{
            let rep = corners_source.corner_rep[i];
            cc.set_corners(rep);
    
            for s in 0..16{
                let mut ss = SYMMETRY_CUBES[s].clone();
                ss = ss.corner_multiply(&cc);
                ss =ss.corner_multiply(&SYMMETRY_CUBES_INVERTED[s] );
                if ss.get_corners() == rep{
                    let q = 1 << s;
                    c_sym[i] |= q;
                }
            }
        }
        c_sym
    }
}

