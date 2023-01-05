// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

pub mod ast;
pub mod codegen;
pub mod dal_pointer;
pub mod err_msg;
pub mod lexer;
pub mod parser;

macro_rules! dal_std_dir {
    () => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/std")
    };
}
pub static DAL_STD_DIR: &'static str = dal_std_dir!();

pub static DAL_VERSION: &'static str = env!("CARGO_PKG_VERSION");
