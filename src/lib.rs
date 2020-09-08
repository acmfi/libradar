pub mod analysis;
pub mod apk;
pub mod callgraph;
pub mod config;
pub mod disass;

use crate::analysis::Analysis;

pub struct Context {}

impl Context {
    pub fn new() -> Context {
        todo!();
    }

    pub fn analysis(&self) -> Box<dyn Analysis> {
        todo!();
    }
}
