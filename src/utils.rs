use fixed::prelude::*;

pub type FixedNum = fixed::types::I16F16;

pub trait SimNum: ToFixed {}
impl SimNum for FixedNum {}
impl SimNum for i32 {}