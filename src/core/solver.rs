use std::cell::Cell;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

use crate::core::prelude::*;
use crate::core::prelude::FaceletPosition::*;
use crate::core::prelude::FaceColor::*;
use itertools::Itertools;
use strum::EnumCount;
use strum::IntoEnumIterator;
use strum_macros::*;
use array_const_fn_init::array_const_fn_init;

use super::coordinate_cube;


pub struct SolveSettings{
    pub max_moves : u8,
    pub stopping_length: u8,
    pub max_iterations: usize
}

impl Default for SolveSettings {
    fn default() -> Self {
        Self { max_moves: 24, stopping_length: 20, max_iterations: 1000 }
    }
}

pub struct Solver{

}

impl Solver{
    pub fn get_solution(base_cube: CubieCube, data_source: Rc<DataSource>, settings: SolveSettings ) -> Option< Vec<Move>>{

        let mut queue: BinaryHeap<SearchState> = BinaryHeap::new();
        
        
        for rotation in 0u8..=2u8{

            for inv_i in 0..=1{
                let inverted = inv_i > 0;

                let rotated_cube =

                match rotation {
                    0=>base_cube.clone(),
                    1=>{
                        URF3_SYMMETRY.multiply(&base_cube).multiply(&URF3_SYMMETRY).multiply(&URF3_SYMMETRY)
                    }
                    _=>URF3_SYMMETRY.multiply(&URF3_SYMMETRY).multiply(&base_cube).multiply(&URF3_SYMMETRY)
                };

                let cube: CoordinateCube = if inverted{rotated_cube.invert().into()} else{rotated_cube.into()};

                let phase_data = cube.create_phase_data(&data_source);

                queue.push(SearchState { cube, 
                    phase_data, 
                    moves: 0,
                     previous: PreviousState::Start { inverted, rotation }, deepening: false });
            }
            
        }


        let mut coordinator = SerialSolveCoordinator{
            data_source,
            solution: None,
            max_moves: settings.max_moves,
            seen: HashMap::new(),
            queue
        };

        
        coordinator.solve(settings.stopping_length, settings.max_iterations)
        .map(|s|s.get_moves())
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct SearchState{
    cube: CoordinateCube,
    phase_data: PhaseData,
    moves: u8,
    previous: PreviousState,
    deepening: bool

}

impl PartialOrd for SearchState{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_priority().partial_cmp(&other.get_priority())
    }
}

impl Ord for SearchState{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_priority().cmp(&other.get_priority())
    }
}

impl SearchState {

    pub fn get_moves(&self)-> Vec<Move>{
        //apply rotations and inversions
        let mut moves: Vec<Move> = Vec::new();

        let mut prev = self.previous.clone();

        loop {

            match prev {
                PreviousState::Start { inverted, rotation } => {
                    if inverted{
                        moves = moves.into_iter().map(|m|m.inverse()).collect_vec();
                    }
                    else{
                        moves.reverse();
                    }

                    if rotation != 0{
                        moves = moves.into_iter().map(|m|m.rotate(rotation)).collect_vec();
                    }
                    
                    return moves;
                },
                PreviousState::Move { state, m } => {
                    moves.push(m);
                    prev = state.previous.clone();
                },
            }
        }
    }


    /// Gets the priority of this solution.
    /// Cubes closer to being solved have higher priority
    /// Solutions with more moves have lower priority
    pub fn get_priority(&self) -> u8{
        match self.phase_data {
            PhaseData::Phase1 { flip_slice_twist_depth_mod3 } => {                
                if self.deepening{
                    (100 - self.moves.max(40))
                }
                else{
                    (50 - self.moves.max(40))
                }
            },
            PhaseData::Phase2 { cornslice_depth, corners_ud_edges_depth_mod3 } => {
                200 - (2 * cornslice_depth.max(12)) - self.moves.max(40)
            },
            PhaseData::Solved => u8::MAX - self.moves.max(50),
        }
    }

    pub fn iterate(self, coordinator : &mut SerialSolveCoordinator) -> Option<Self> {
        
        match self.phase_data{
            PhaseData::Phase1 { flip_slice_twist_depth_mod3 } => {
                
                let next_depth = (flip_slice_twist_depth_mod3+ 3) % 3;

                if next_depth + 1 + self.moves >= coordinator.max_moves{return None;}//no way to solve in time

                let next_previous_state = Arc::new(self.clone());

                for m in Move::iter().filter(|&m|self.can_do_move(m)){
                    let next_cube = self.cube.after_move(m, &coordinator.data_source.moves_source);
                    let next_phase = next_cube.create_phase_data(&coordinator.data_source);
                    let next_is_deepening = match next_phase {
                        PhaseData::Phase1 { flip_slice_twist_depth_mod3 } => flip_slice_twist_depth_mod3 == next_depth,
                        PhaseData::Phase2 { cornslice_depth, corners_ud_edges_depth_mod3 } => true,
                        PhaseData::Solved => true,
                    };

                    if next_is_deepening || (!self.deepening && self.moves <= 2){ //Past a certain point, only do deepening moves
                        let next_state = Self{
                            cube: next_cube,
                            phase_data: next_phase,
                            moves: self.moves +1,
                            previous: PreviousState::Move { state: next_previous_state.clone(), m },
                            deepening: next_is_deepening
                            
                        };

                        coordinator.maybe_add_search(next_state);
                    }

                }


                None
            },
            PhaseData::Phase2 { cornslice_depth, corners_ud_edges_depth_mod3 } => {
                
                if cornslice_depth >= (coordinator.max_moves - self.moves).min(11){
                    return None;
                }

                let next_depth = (corners_ud_edges_depth_mod3+ 3) % 3;                
                
                let next_previous_state = Arc::new(self.clone());

                for m in Move::PHASE2MOVES.into_iter().filter(|&m|self.can_do_move(m)){
                    let next_cube = self.cube.after_move(m, &coordinator.data_source.moves_source);
                    let next_phase = next_cube.create_phase_data(&coordinator.data_source);
                    let next_is_deepening = match next_phase {
                        PhaseData::Phase1 { flip_slice_twist_depth_mod3 } => false, //should be unreachable
                        PhaseData::Phase2 { cornslice_depth, corners_ud_edges_depth_mod3 } => corners_ud_edges_depth_mod3 == next_depth,
                        PhaseData::Solved => true,
                    };

                    if next_is_deepening{
                        let next_state = Self{
                            cube: next_cube,
                            phase_data: next_phase,
                            moves: self.moves +1,
                            previous: PreviousState::Move { state: next_previous_state.clone(), m },
                            deepening: next_is_deepening
                            
                        };

                        coordinator.maybe_add_search(next_state);
                    }
                }
                None
            },
            PhaseData::Solved => Some(self),
        }
    }

    fn can_do_move(&self, m: Move)-> bool{
        if let PreviousState::Move { state: _, m:mprev }  = self.previous{
            if !mprev.can_precede(m){
                return false;
            }
        }
        true
    }

    
}

#[derive(PartialEq, Eq, Clone)]
pub enum PreviousState{
    Start{inverted: bool,rotation: u8,},
    Move{state: Arc<SearchState>, m: Move}
}



pub struct SerialSolveCoordinator{
    pub data_source : Rc<DataSource> ,
    pub solution :Option<SearchState>,
    pub max_moves: u8,
    pub queue: BinaryHeap<SearchState>,
    pub seen: HashMap<CoordinateCube, u8>
}



impl SerialSolveCoordinator{

    pub fn solve(&mut self,stopping_length: u8, max_iterations: usize) -> Option<SearchState> {

        let mut iterations = 0;
        while iterations < max_iterations  {
            if let Some(next) = self.queue.pop(){
                if let Some(solution) = next.iterate(self){
                    if self.try_add_solution(solution.clone()) && solution.moves < stopping_length {
                        return Some(solution);
                    }
                }
            }
            else {
                return self.solution.clone();
            }

            iterations+=1;
        }
        None
    }

    fn try_add_solution(&mut self, state : SearchState) -> bool{
        if let Some(current) = &self.solution{
            if current.moves <= state.moves{
                return false;
            }
        }

        self.max_moves = state.moves - 1;
        self.solution = Some(state);        
        true
    }

    pub fn maybe_add_search(&mut self, state: SearchState){

        let cube = state.cube.clone();
        let moves = state.moves;

        loop{

            let insert_result = self.seen.try_insert(cube.clone(), moves);

            match insert_result {
                Ok(_) =>{
                    self.queue.push(state);
                    return;
                } ,
                Err(entry) => {
                    if entry.value <= moves{
                        return; //The previous route was as good as or better than this
                    }
                    else{
                        self.seen.remove_entry(&cube.clone());//This was a better path. Try inserting it again
                    }
                },
            }
        }
        

    }
}