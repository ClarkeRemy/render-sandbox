use crate::prelude::*;
// dynamically linked in debug
#[cfg(debug_assertions)]
#[hot_lib_reloader::hot_module(dylib = "hot")]
pub(crate) mod hot_lib {
  use crate::prelude::dylib_prelude::*;
  use dylib_signatures::hot::*;
  hot_functions_from_file!{"dylibs/hot/src/lib.rs"} 
}

// statically linked in release
#[cfg(not(debug_assertions))]
use hot as hot_lib;


use dylib_signatures::hot::*;
pub(crate) struct Dlib;
impl Hot for Dlib { const RENDER : RenderFunctionSig = hot_lib::render; }
