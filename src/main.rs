extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate sdl2;
extern crate sdl2imgui;

use imgui::*;

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video = sdl_context.video().unwrap();

	{
		let gl_attr = video.gl_attr();
		gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
		gl_attr.set_context_version(3, 0);
	}

	let window = video
		.window("sdl2imgui", 800, 600)
		.position_centered()
		.resizable()
		.opengl()
		.build()
		.unwrap();

	let _gl_context = window
		.gl_create_context()
		.expect("Couldn't create GL context");
	gl::load_with(|s| video.gl_get_proc_address(s) as _);

	let mut imgui = imgui::ImGui::init();
	imgui.set_ini_filename(None);

	let mut imgui_sdl2 = sdl2imgui::ImguiSdl2::new(&mut imgui);

	let renderer =
		imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

	let mut event_pump = sdl_context.event_pump().unwrap();

	//let mut exit = false;

	let mut canvas = window.into_canvas().build().unwrap();

	'running: loop {
		use sdl2::event::Event;
		use sdl2::keyboard::Keycode;

		for event in event_pump.poll_iter() {
			imgui_sdl2.handle_event(&mut imgui, &event);
			if imgui_sdl2.ignore_event(&event) {
				continue;
			}

			match event {
				Event::Quit { .. }
				| Event::KeyDown {
					keycode: Some(Keycode::Escape),
					..
				} => break 'running,
				_ => {}
			}
		}

		let ui = imgui_sdl2.frame(&canvas.window(), &mut imgui, &event_pump);

		let window = ui
			.window(im_str!("ImguiWindow"))
			.size((300.0, 300.0), ImGuiCond::Always);

		window.build(|| {
			ui.text(im_str!("Pick a color"));
		});

		canvas.window_mut().gl_make_current(&_gl_context).unwrap();
		unsafe {
			gl::ClearColor(0.2, 0.2, 0.8, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}
		unsafe { gl::Flush() };

		canvas.set_draw_color(sdl2::pixels::Color::RGB(200, 20, 20));
		canvas
			.draw_rect(sdl2::rect::Rect::new(100, 100, 200, 200))
			.unwrap();
		unsafe { gl::Flush() };

		canvas.window_mut().gl_make_current(&_gl_context).unwrap();
		renderer.render(ui);
		unsafe { gl::Flush() };

		canvas.present();

		::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
	}
}
