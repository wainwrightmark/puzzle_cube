use crate::core::prelude::*;

//#[derive(BorshSerialize, BorshDeserialize)]
pub struct DataSource {
    ///Indicates the minimum number of phase 2 moves required to solve the corners and slice. Indexed by corners * 24 + slice
    pub corner_slice_depth: Vec<u8>,

    pub phase_2_pruning: Vec<u32>,
    pub phase_2_edge_merge: Vec<u16>,
    pub phase_1_pruning: Vec<u32>,

    pub flip_slice_source: FlipSliceSource,

    pub corners_source: CornerSymmetriesSource,

    pub moves_source: MovesSource,
}

pub struct CornerSymmetriesSource {
    pub corner_class_index: Vec<u16>,
    pub corner_symmetry: Vec<u8>,
    pub corner_rep: Vec<u16>,
}

pub struct FlipSliceSource {
    pub flip_slice_class_index: Vec<u16>,
    pub flip_slice_symmetry: Vec<u8>,
    pub flip_slice_rep: Vec<u32>,
}

impl DataSource {
    pub fn create() -> Self {
        let moves_source = MovesSource::create();
        let corners_source = CornerSymmetriesSource::create();
        let flip_slice_source = FlipSliceSource::create();
        let phase_2_edge_merge = Self::create_phase_2_edge_merge();
        let corner_slice_depth = Self::create_corner_slice_depth(&moves_source);
        let phase_2_pruning = Self::create_phase_2_pruning(&moves_source, &corners_source);
        let phase_1_pruning = Self::create_phase_1_pruning(&moves_source, &flip_slice_source);

        Self {
            moves_source,
            corner_slice_depth,
            corners_source,
            flip_slice_source,
            phase_2_edge_merge,
            phase_1_pruning,
            phase_2_pruning,
        }
    }

    pub fn get_flip_slice_twist_depth_mod_3(&self, flip: u16, twist: u16, slice_sorted: u16) -> u8 {
        let slice = slice_sorted / 24;
        let flip_slice = (NFLIP * (slice as usize)) + (flip as usize);

        let class_index = self.flip_slice_source.flip_slice_class_index[flip_slice] as usize;
        let flip_slice_sym = self.flip_slice_source.flip_slice_symmetry[flip_slice];

        let twist_conj = self.moves_source.get_twist_conj(twist, flip_slice_sym) as usize;

        let ix = NTWIST * class_index + twist_conj;

        let mut y = self.phase_1_pruning[ix / 16];
        y >>= (ix % 16) * 2;
        let r = y & 3;

        r as u8
    }

    pub fn get_ud_edges(&self, u_edges: u16, d_edges: u16) -> u16 {
        let index = 24 * u_edges + (d_edges % 24);

        self.phase_2_edge_merge[index as usize]
    }

    pub fn get_corners_ud_edges_depth_3(&self, corners: u16, ud_edges: u16) -> u8 {
        let corner_class_index = self.corners_source.corner_class_index[corners as usize];
        let corner_sym = self.corners_source.corner_symmetry[corners as usize];

        let ud_edges_conj = self.moves_source.get_ud_edges_conj(ud_edges, corner_sym);
        let index = NUDEDGES * (corner_class_index as usize) + (ud_edges_conj as usize);

        let mut y = self.phase_2_pruning[index / 16];
        y >>= (index % 16) * 2;
        (y & 3) as u8
    }

    pub fn get_cornslice_depth(&self, corners: u16, slice_sorted: u16) -> u8 {
        let index = (24 * (corners as usize)) + slice_sorted as usize;

        self.corner_slice_depth[index]
    }

    pub fn create_up_down_edges_conjugation() -> Vec<u16> {
        let mut table: Vec<u16> = Vec::new();
        table.reserve_exact(40320 * 16);

        for edges in 0..40320 {
            let mut edges_cube = CubieCube::default();
            edges_cube.set_ud_edges(edges);

            for symmetry in 0..16 {
                let mut sym_cube = SYMMETRY_CUBES[symmetry].clone();
                sym_cube = sym_cube.edge_multiply(&edges_cube);
                sym_cube = sym_cube.edge_multiply(&SYMMETRY_CUBES_INVERTED[symmetry]);
                let ud_edges = sym_cube.get_ud_edges().unwrap();
                table.push(ud_edges);
            }
        }

        table
    }

    pub fn create_twist_conjugation() -> Vec<u16> {
        let mut table: Vec<u16> = Vec::new();
        table.reserve_exact(2187 * 16);

        for twist in 0..2187 {
            let mut cubie = CubieCube::default();
            cubie.set_twist(twist);

            for symmetry in 0..16 {
                let mut sym_cube = SYMMETRY_CUBES[symmetry].clone();
                sym_cube = sym_cube.corner_multiply(&cubie);
                sym_cube = sym_cube.corner_multiply(&SYMMETRY_CUBES_INVERTED[symmetry]);
                let new_twist = sym_cube.get_twist();
                table.push(new_twist);
            }
        }

        table
    }
}

//#[derive(BorshSerialize, BorshDeserialize)]
pub struct MovesSource {
    pub twist_move: Vec<u16>,
    pub flip_move: Vec<u16>,
    pub slice_sorted_move: Vec<u16>,
    pub u_edges_move: Vec<u16>,
    pub d_edges_move: Vec<u16>,
    pub u_d_edges_move: Vec<u16>,
    pub corners_move: Vec<u16>,
    pub u_d_edges_conjugation: Vec<u16>,

    pub twist_conjugation: Vec<u16>,
}

impl MovesSource {
    pub fn create() -> Self {
        let corners_move = CornersProperty::create(&CornersProperty {});
        let twist_move = TwistProperty::create(&TwistProperty {});
        let flip_move = FlipProperty::create(&FlipProperty {});
        let slice_sorted_move = SliceSortedProperty::create(&SliceSortedProperty {});
        let u_edges_move = UpEdgesProperty::create(&UpEdgesProperty {});
        let d_edges_move = DownEdgesProperty::create(&DownEdgesProperty {});
        let u_d_edges_move = UpDownEdgesProperty::create(&UpDownEdgesProperty {});

        let u_d_edges_conjugation = DataSource::create_up_down_edges_conjugation();
        let twist_conjugation = DataSource::create_twist_conjugation();

        Self {
            twist_move,
            flip_move,
            slice_sorted_move,
            u_edges_move,
            d_edges_move,
            u_d_edges_move,
            corners_move,
            u_d_edges_conjugation,
            twist_conjugation,
        }
    }

    pub fn get_ud_edge(&self, prev: u16, m: usize) -> u16 {
        self.u_d_edges_move[((prev as usize) * 10) + m]
    }

    pub fn get_u_edge(&self, prev: u16, m: Move) -> u16 {
        self.u_edges_move[((prev as usize) * 18) + m as usize]
    }
    pub fn get_d_edge(&self, prev: u16, m: Move) -> u16 {
        self.d_edges_move[((prev as usize) * 18) + m as usize]
    }

    pub fn get_slice_sorted(&self, prev: u16, m: Move) -> u16 {
        self.slice_sorted_move[((prev as usize) * 18) + m as usize]
    }

    pub fn get_slice(&self, prev: u16, m: Move) -> u16 {
        self.slice_sorted_move[((prev as usize) * 18 * 24) + m as usize] / 24
    }

    pub fn get_corners(&self, prev: u16, m: Move) -> u16 {
        self.corners_move[((prev as usize) * 18) + m as usize]
    }

    pub fn get_twist(&self, prev: u16, m: Move) -> u16 {
        self.twist_move[((prev as usize) * 18) + m as usize]
    }

    pub fn get_flip(&self, prev: u16, m: Move) -> u16 {
        self.flip_move[((prev as usize) * 18) + m as usize]
    }

    pub fn get_ud_edges_conj(&self, ud_edges: u16, corner_sym: u8) -> u16 {
        let cs = corner_sym as usize;

        let idx = ((ud_edges as usize) << 4) + cs;

        self.u_d_edges_conjugation[idx]
    }

    pub fn get_twist_conj(&self, twist: u16, flip_slice_sym: u8) -> u16 {
        self.twist_conjugation[((twist as usize) << 4) + flip_slice_sym as usize]
    }
}
