use crate::core::prelude::*;







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
        y & 3
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

        let _ud_edge = 0;
        Self::set_corners_ud_edges_depth3(0, 0, &mut table);
        let mut depth = 0u32;
        let mut done = 1;

        let mut next = vec![(0,0)];//next is tuples of (corner_class_index, ud_edges_conj)
        let mut current: Vec<(u16, u16)> = Vec::new();
        while depth < 10 {
            
            std::mem::swap(&mut next,&mut current);

            for (corner_class_index, ud_edge) in current.drain(..){
                let mut move_index = 0;
                let corners = corners_source.corner_rep[corner_class_index as usize];
                for m in Move::PHASE2MOVES{
                    let ud_edges_after_move = moves_source.get_ud_edge(ud_edge, move_index);
                    let corners_after_move =  moves_source.get_corners(corners, m);

                    let corner_class_index_after_move = corners_source.corner_class_index[corners_after_move as usize];
                    let corner_symmetry_after_move = corners_source.corner_symmetry[corners_after_move as usize];

                    let udedge1_conj = moves_source.get_ud_edges_conj(ud_edges_after_move, corner_symmetry_after_move);

                    let idx1 = (40320 * (corner_class_index_after_move as usize)) + udedge1_conj as usize;

                    if Self::get_corners_ud_edges_depth3(idx1, &table) ==3{ //this index has not yet been set
                        Self::set_corners_ud_edges_depth3(idx1, (depth + 1) % 3, &mut table);
                        next.push((corner_class_index_after_move, udedge1_conj));
                        done+= 1;

                        let mut sym = c_sym[corner_class_index_after_move as usize];
                        if sym != 1{
                            for j in 1..16{
                                sym >>=1;
                                if sym % 2 ==1{
                                    let ud_edges_after_move_and_symmetry = moves_source.get_ud_edges_conj(udedge1_conj, j);

                                    let idx2 = (40320 * (corner_class_index_after_move as usize)) + ud_edges_after_move_and_symmetry as usize;

                                    if Self::get_corners_ud_edges_depth3(idx2, &table) == 3{ //this index has not yet been set
                                        Self::set_corners_ud_edges_depth3(idx2, (depth + 1) % 3, &mut table);
                                        next.push((corner_class_index_after_move, ud_edges_after_move_and_symmetry));
                                        done+= 1;
                                    }
                                }
                            }
                        }
                    }
                    move_index+=1;
                }
            }
            depth = depth + 1;
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

