#![no_implicit_prelude]
extern crate core;
extern crate dylib_signatures;

use core::{ iter::Iterator, cmp::Ord };
use dylib_signatures::hot::*;



#[no_mangle]
pub fn render
( mut input : SliceWidthHeight) { composite(&mut input) }
struct CheckSig;
impl Hot for CheckSig 
{ const RENDER : RenderFunctionSig = render;
}

type RgbU32 = u32;

fn composite 
( input : &mut SliceWidthHeight)
{ bg(input, rgb(255,255,255))
; diagonal_test(input, 50, rgb(10, 150, 220))
; diagonal_test(input, 30, rgb(0, 0, 30))
; 
}

fn bg
( SliceWidthHeight { slice, .. } : &mut SliceWidthHeight, color : RgbU32)
{ for each in &mut**slice { *each = color }
}

#[inline] fn red   (r : u8)->RgbU32 {(r as u32) << 16}
#[inline] fn green (g : u8)->RgbU32 {(g as u32) << 8 }
#[inline] fn blue  (b : u8)->RgbU32 { b as u32       }

#[inline] fn rgb   (r : u8, g : u8, b : u8) -> RgbU32 { red(r)|green(g)|blue(b) }

fn row_bounds(stride_w : u32, row : u32)->[usize ;2] 
{ let start = (row  * stride_w) as usize
; let end   = start + stride_w  as usize
; [start, end]
}

fn arr_idx<T>((x,y):(T,T)) -> [T;2] {[x,y]}
fn index_offset(stride_w : u32, [w,h] : [u32;2])->usize { (stride_w*h + w) as usize } 



fn diagonal_test
( SliceWidthHeight { slice, width, height } : &mut SliceWidthHeight, radius : usize, color : RgbU32)
{ for idx @ [_,h] in (0..*width).zip(0..*height).map(arr_idx)
  { let [start, end] = row_bounds(*width, h)
  ; for r in -(radius as isize)..=radius as isize
    { let i = index_offset(*width, idx) as isize + r
    ; (*slice)[i.clamp(start as isize, (end as isize)-1) as usize] = color
    ;
    }
  }
}

fn basic_test
( SliceWidthHeight { slice, width, height } : SliceWidthHeight)
{ for h in 0..height { let stride = h*width; for w in 0..width
  { let [r,g,b] = [ w % 255, h % 255, (w*h)%255]
  
  ; slice[ (stride + w) as usize] = r << 16 | g << 8 | b
  ;
  }}
}
