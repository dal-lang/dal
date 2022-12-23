// Copyright (c) 2022 Ade M Ramdani
//
// This file is part of the dal.
// See the LICENSE file for licensing information.

use crate::ErrColor;

pub mod analyze;
pub mod dal_llvm;

pub struct Codegen;

pub enum OutType {
    Unknown,
    Executable,
    Library,
    Object,
}

pub enum BuildType {
    Debug,
    Release,
}

impl Codegen {
    pub fn set_build_type(&self, build_type: BuildType) {}

    pub fn set_is_static(&self, is_static: bool) {}

    pub fn set_strip(&self, strip: bool) {}

    pub fn set_verbose(&self, verbose: bool) {}

    pub fn set_errmsg_color(&self, errmsg_color: ErrColor) {}

    pub fn set_out_type(&self, out_type: OutType) {}

    pub fn set_out_name(&self, out_name: String) {}

    pub fn add_root_code(&self, root_code: String) {}

    pub fn link(&self, lib: String) {}
}
