use crate::core::prelude::*;





use std::rc::Rc;
use yewdux::prelude::*;
use log::debug;

#[derive(Store, Default)]
pub struct DataState {
    pub data: Option<Rc<DataSource>>,
}

impl DataState{
    
    pub fn is_generated(&self)-> bool{
        self.data.is_some()
    }
}

impl PartialEq for DataState{
    fn eq(&self, other: &Self) -> bool {
        self.data.is_some() == other.data.is_some()
    }
}

pub struct GenerateMsg{}

impl Reducer<DataState> for GenerateMsg {
    fn apply(&self, state: Rc<DataState>) -> Rc<DataState> {
        if state.is_generated(){
            state
        }
        else{
            debug!("Generating solve data");
            let start_instant = instant::Instant::now();
            let data = DataSource::create();
            let diff = instant::Instant::now() - start_instant;

            debug!("Solve generated in {:?}", diff);

            let state = DataState{data: Some(data.into())};

            state.into()

        }
    }
}