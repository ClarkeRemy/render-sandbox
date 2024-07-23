pub mod hot 
{ 
  pub struct SliceWidthHeight<'a> { pub slice : &'a mut[u32], pub width : u32, pub height : u32 }
  pub type RenderFunctionSig = fn( SliceWidthHeight );

  pub trait Hot
  { const RENDER : RenderFunctionSig; }
}
pub use hot::Hot;
