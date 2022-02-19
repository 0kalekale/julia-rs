/*
    mandelbrot set in opengl with glium
    Copyright (C) 2022  kalekale.anon@gmail.com

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

#[macro_use]
extern crate glium;
mod rect;

fn main(){
    	// Setup glium bolier plate
	use glium::{glutin, Surface};

	let mut event_loop = glutin::event_loop::EventLoop::new();
	let wb = glutin::window::WindowBuilder::new().with_inner_size(glutin::dpi::PhysicalSize::new(800,600));
	let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
	let display = glium::Display::new(wb, cb, &event_loop).unwrap();

	let vertexBuffer = glium::VertexBuffer::new(&display, &rect::vertices).unwrap();
	let index = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &rect::indices).unwrap();

    	// Vertex and Fragment shader programs
    	// TODO: open this shit from a file instead of having it built in 
	let vertex_shader = r#"
		#version 140
		in vec3 position;
		uniform mat4 matrix;
		out vec4 pos;
		void main() {
			pos =  matrix*vec4(position, 1.0);
			gl_Position = pos;
		}
	"#;

	let fragment_shader = r#"
	#version 330 core
	in vec4 gl_FragCoord;
	uniform int max_it;
	out vec4 frag_color;
	uniform float c_re;
	uniform float c_im;
	int get_iterations() {
		float real;
		float imag;

		real = ((gl_FragCoord.x / 800.0 - 0.5)*4); /* * 0.0625/8) - 0.75; */
		imag = ((gl_FragCoord.y / 600.0 - 0.5)*3); /* * 0.046875/8)+0.05; */

    		int iterations = 0;
    		float const_real = real;
    		float const_imag = imag;
    		while (iterations < max_it) {
       			float tmp_real = real;
        		real = (real * real - imag * imag) + c_re;
        		imag = (2.0 * tmp_real * imag) + c_im;

        		float dist = real * real + imag * imag;

        		if (dist > 4.0)
        			break;

        		++iterations;
    			}
    		return iterations;
		}
		vec4 return_color() {
    		int iter = get_iterations();
    		if (iter == max_it) {	
        		gl_FragDepth = 0.0f;
        		return vec4(0.0f, 0.0f, 0.0f, 1.0f);
    		}
    		float iterations = float(iter) / max_it;
		if (iterations>max_it/2) {
			return vec4(iterations/2, 0.0, iterations, 1.0f);
		} 
		else {   
    			return vec4(iterations/5, 0.0, iterations/5, 1.0f);
		}
	}
 
	void main() {
    		frag_color = return_color();
	}	
	"#;

    	// Define the shader program 
	let program = glium::Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();

    	// Iterations 
    	let mut i = 65i32;
    	// real and complex parts of e^x*i complex number
    	let mut c_re = 0.0f32;
    	let mut c_im = 0.0f32;
    
	event_loop.run(move |ev, _, control_flow| {	
	    	if (c_re>1.0) { 
			c_re = -1.0;
		}
	    	if (c_im>1.0) {
			c_im = -1.0;
		}
	    
	    	println!("iterations: {} c_re: {}, c_im: {}", i, c_re, c_im);	
		let uniforms = uniform! {
			matrix: [
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 1.0, 1.0, 0.0],
				[0.0, 0.0, 0.0, 1.0f32],	
			],
			max_it: i,
			c_re: c_re,
		    	c_im: c_im
		};

		let params = glium::DrawParameters {
    			depth: glium::Depth {
        			test: glium::draw_parameters::DepthTest::IfLess,
        			write: true,
        			.. Default::default()
    			},
    			.. Default::default()
		};
	
		let mut target = display.draw();
		target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
		target.draw(&vertexBuffer, &index, &program, &uniforms, &params).unwrap();
		target.finish().unwrap();	

		display.gl_window().window().request_redraw();		
		let next_frame_time = std::time::Instant::now() + std::time::Duration::new(1, 0);
		*control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
	
		match ev {
			glutin::event::Event::WindowEvent { event, ..} => match event {
				glutin::event::WindowEvent::CloseRequested => {
					*control_flow = glutin::event_loop::ControlFlow::Exit;
					return;
				},
				glutin::event::WindowEvent::KeyboardInput {input, ..} => match input.scancode {
					0x0d => i+=1,
					0x0c => i-=1,
					17 => c_re += 0.001, //w
					31 => c_im += 0.001, //s
					_ => return,
				}
			_ => return,
			}

		_ => (),
		}
	});
}
