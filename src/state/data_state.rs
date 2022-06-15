use crate::core::prelude::*;
use crate::state::prelude::*;
use itertools::Itertools;
use num::ToPrimitive;
use serde::*;
use std::default;
use std::lazy::OnceCell;
use std::rc::Rc;
use yewdux::prelude::*;

#[derive(Store, Default)]
pub struct DataState {
    pub data: OnceCell<Rc<DataSource>>
}

impl DataState{
    pub fn get_data_source(&self)->Rc<DataSource>{
        let r = self.data.get_or_init(||DataSource::create().into()).clone();        
        r
    }
}

impl PartialEq for DataState{
    fn eq(&self, other: &Self) -> bool {
        self.data.get().is_some() == other.data.get().is_some()
    }
}