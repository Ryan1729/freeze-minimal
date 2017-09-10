extern crate open_gl_bindings;
extern crate sdl2;

extern crate common;
extern crate image_decoding;

use open_gl_bindings::gl;

use std::fs::File;

use std::str;

use common::*;

macro_rules! opengl_error_check {
    () => {

        if cfg!(debug_assertions) {
            #[allow(unused_unsafe)] {
                if let Some(ref resources) = unsafe { RESOURCES.as_ref() } {
                    let mut err;
                    while {
                        err = unsafe { resources.ctx.GetError() };
                        err != gl::NO_ERROR
                    }
                    {
                        let err_str = match err {
                            gl::INVALID_ENUM => "INVALID_ENUM",
                            gl::INVALID_VALUE => "INVALID_VALUE",
                            gl::INVALID_OPERATION => "INVALID_OPERATION",
                            gl::STACK_OVERFLOW => "STACK_OVERFLOW",
                            gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
                            gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
                            _ => "Unknown error type",
                        };
                        println!("OpenGL error: {}({}) on line {} of {}",
                            err_str,
                            err,
                            line!(),
                            file!()
                        );
                    }
                    if err != gl::NO_ERROR {
                        panic!();
                    }
                }
            }
        }
    }
}

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
        .window("Window", INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT)
        .opengl()
        .build()
        .unwrap()
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();

    let ctx = gl::Gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    canvas.window().gl_set_context_to_current().unwrap();
	
	let pixels: Vec<u8> = vec![240; 256 * 256 * 4];
	
	let mut texture = 0;
	unsafe {
		ctx.GenTextures(1, &mut texture as _);

		ctx.BindTexture(gl::TEXTURE_2D, texture);

		ctx.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
		ctx.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
		ctx.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);
		ctx.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);
		println!("TexImage2D");
		ctx.TexImage2D(
			gl::TEXTURE_2D,
			0,
			gl::RGBA8 as _,
			256 as _,
			256 as _,
			0,
			gl::RGBA,
			gl::UNSIGNED_BYTE,
			pixels.as_ptr() as _,
		);
	}
	
	println!("pre make_texture_from_png");
	//make_texture_from_png(&ctx, "images/texture0.png");
	make_texture_from_png(&ctx, "images/256.png");
	println!("post make_texture_from_png");
	println!("GOT HERE");
}

fn make_texture_from_png(ctx: &gl::Gl, filename: &str) -> gl::types::GLuint {
    let mut texture = 0;

    if let Ok(image) = File::open(filename) {
		println!("decode_png");
        match image_decoding::decode_png(image) {
            (Ok((width, height)), Ok(colortype), Ok(pixels)) => {
				println!("decode_png");
                let (external_format, data_type) = match colortype {
                    image_decoding::ColorType::RGB(8) => (gl::RGB, gl::UNSIGNED_BYTE),
                    image_decoding::ColorType::RGB(16) => (gl::RGB, gl::UNSIGNED_SHORT),
                    image_decoding::ColorType::RGBA(8) => (gl::RGBA, gl::UNSIGNED_BYTE),
                    image_decoding::ColorType::RGBA(16) => (gl::RGBA, gl::UNSIGNED_SHORT),
                    _ => {
                        //TODO make this case more distinct
                        return 0;
                    }
                };

				println!("e: {:?}", (gl::RGBA, gl::UNSIGNED_BYTE));
				println!("g: {:?}", (external_format, data_type));
				
				let  v : Vec<u8> = match pixels {
                            image_decoding::DecodingResult::U8(v) => v,
                            image_decoding::DecodingResult::U16(v) => panic!(),
                        };
				
				
                unsafe {
                    ctx.GenTextures(1, &mut texture as _);

                    ctx.BindTexture(gl::TEXTURE_2D, texture);

                    ctx.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
                    ctx.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
                    ctx.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);
                    ctx.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);
					println!("TexImage2D");
                    ctx.TexImage2D(
                        gl::TEXTURE_2D,
                        0,
                        gl::RGBA8 as _,
                        width as _,
                        height as _,
                        0,
                        external_format,
                        data_type,
                        v.as_ptr() as _,
                    );
                }
            }
            _ => {
                return 0;
            }
        }
    }
    return texture;
}
