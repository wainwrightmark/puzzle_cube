use crate::core::prelude::*;

use log::debug;
use std::rc::Rc;
use yewdux::prelude::*;

#[derive(Store, Default)]
pub struct DataState {
    pub data: Option<Rc<DataSource>>,
}

impl DataState {
    pub fn is_generated(&self) -> bool {
        self.data.is_some()
    }

    pub fn with_generate_data(self: Rc<Self>)-> Rc<Self>{
        if self.is_generated() {
            self
        } else {
            debug!("Generating solve data");
            let start_instant = instant::Instant::now();
            let data = DataSource::create();
            let diff = instant::Instant::now() - start_instant;

            debug!("Solve generated in {:?}", diff);

            Self{
                data:Some(data.into())
            }.into()

        }
    }
}

impl PartialEq for DataState {
    fn eq(&self, other: &Self) -> bool {
        self.data.is_some() == other.data.is_some()
    }
}