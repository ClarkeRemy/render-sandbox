#![no_implicit_prelude]
pub(crate) mod prelude;

use dylib_signatures::hot::SliceWidthHeight;
use prelude::*;
use winit::
{ event::{Event, WindowEvent}
, event_loop::{ControlFlow, EventLoop}
, window::WindowBuilder
};


// internal crates

// use dylib_signatures::*;
// use dylib::Dlib;

pub(crate) mod dylib;

fn main()
{ use std::time::Instant

; let event_loop  = EventLoop::new()
; let window      = WindowBuilder::new().build(&event_loop).unwrap()
; let context     = unsafe { softbuffer::Context::new(&window) }.unwrap()
; let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap()
; let [mut now, mut last] = [Instant::now(), Instant::now()]

; event_loop.run(move |event, _, control_flow| 
  { *control_flow = ControlFlow::Poll

  ;println!("-- : {}, +- : {}, -+ : {}", -5.1%-3.2,   5.1%-3.2,  -5.1%3.2 )
  ; match event 
    { 
       Event::RedrawRequested(window_id) if window_id == window.id()
    =>{redraw(&window, &mut surface, dylib::hot_lib::render).unwrap_or(())}
  
    _refresh if now.duration_since(last) > core::time::Duration::from_secs(2)
    =>{ last = now 
      ; redraw(&window, &mut surface, dylib::hot_lib::render).unwrap_or(())
      }

    | Event::WindowEvent { window_id, event : WindowEvent::CloseRequested }  
      if window_id == window.id()
    =>{ *control_flow = ControlFlow::Exit }

    | _else 
    =>{}
    }
    now = Instant::now();
  })
;
}


fn redraw (window : &winit::window::Window, surface : &mut softbuffer::Surface, render_function : fn(SliceWidthHeight)) 
-> Result<(),softbuffer::SoftBufferError>
{ let size = window.inner_size()      
; let [width, height] = [size.width, size.height].map(|x| x.max(1))
; let nz_u32 = |x : u32| unsafe {core::num::NonZeroU32::new_unchecked(x)} // Safety: x.max(1) is never 0

; surface.resize(nz_u32(width), nz_u32(height))?

; let mut buffer = surface.buffer_mut()?
; let slice = &mut buffer[..]

; render_function(SliceWidthHeight { slice, width, height });

  buffer.present()
}
