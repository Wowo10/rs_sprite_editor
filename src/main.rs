extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate sdl2;
extern crate sdl2imgui;

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

	let mut exit = false;

	while !exit {
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
				} => exit = true,
				_ => {}
			}
		}

		let ui = imgui_sdl2.frame(&window, &mut imgui, &event_pump);
		ui.show_demo_window(&mut true);

		unsafe {
			gl::ClearColor(0.2, 0.2, 0.2, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}

		renderer.render(ui);

		window.gl_swap_window();

		::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
	}
}
