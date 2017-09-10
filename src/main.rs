extern crate open_gl_bindings;
extern crate sdl2;
extern crate image;

use image::ImageDecoder;

use open_gl_bindings::gl;

use std::fs::File;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_stencil_size(1);
    gl_attr.set_context_major_version(2);
    gl_attr.set_context_minor_version(1);

    let canvas = video_subsystem
        .window("Window", 800, 600)
        .opengl()
        .build()
        .unwrap()
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    let ctx = gl::Gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    canvas.window().gl_set_context_to_current().unwrap();
	
	println!("pre make_texture_from_png");
	make_texture_from_png(&ctx, "images/256.png");
	println!("post make_texture_from_png");
}

fn make_texture_from_png(ctx: &gl::Gl, filename: &str) -> gl::types::GLuint {
    let mut texture = 0;

    if let Ok(image) = File::open(filename) {
		let mut decoder = image::png::PNGDecoder::new(image);
        match (decoder.dimensions(),decoder.read_image()) {
            (Ok((width, height)), Ok(pixels)) => {
				let  pixel_ptr : *const u8 = match pixels {
					image::DecodingResult::U8(v) => v.as_ptr(),
					image::DecodingResult::U16(v) => v.as_ptr() as *const u8,
				};
				
                unsafe {
                    ctx.GenTextures(1, &mut texture as _);

                    ctx.BindTexture(gl::TEXTURE_2D, texture);

                    ctx.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
                    ctx.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
                    ctx.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);
                    ctx.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);
					println!("calling TexImage2D");
                    ctx.TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        gl::RGBA8 as _,
                        width as _,
                        height as _,
                        0,
                        gl::RGBA,
						gl::UNSIGNED_BYTE,
                        pixel_ptr as _,
                    );
                }
            }
            _ => {
                return 0;
            }
        }
    }
    texture
}
