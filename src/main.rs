extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate sdl2;
extern crate sdl2imgui;

use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::time::Duration;

fn main() {
	let sdl_context = sdl2::init().unwrap();
	let video = sdl_context.video().unwrap();

	{
		let gl_attr = video.gl_attr();
		gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
		gl_attr.set_context_version(3, 1);
	}

	let window = video
		.window("sdl2imgui", 800, 600)
		.position_centered()
		.opengl()
		.build()
		.unwrap();
	// .position_centered()
	// .resizable()
	// .opengl()
	// .build()
	// .unwrap();

	let mut canvas = window.into_canvas()
        .accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();

	canvas.set_draw_color(sdl2::pixels::Color::RGBA(0,0,0,255));

	let mut timer = sdl_context.timer().unwrap();

    //let mut event_pump = sdl_context.event_pump().unwrap();

	let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("assets/characters.bmp")).unwrap();
    let texture = texture_creator.create_texture_from_surface(&temp_surface).unwrap();

    let frames_per_anim = 4;
    let sprite_tile_size = (32,32);

    // Baby - walk animation
    let mut source_rect_0 = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_0 = Rect::new(0, 0, sprite_tile_size.0*4, sprite_tile_size.0*4);
    dest_rect_0.center_on(Point::new(-64,120));

    // King - walk animation
    let mut source_rect_1 = Rect::new(0, 32, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_1 = Rect::new(0, 32, sprite_tile_size.0*4, sprite_tile_size.0*4);
    dest_rect_1.center_on(Point::new(0,240));

    // Soldier - walk animation
    let mut source_rect_2 = Rect::new(0, 64, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_2 = Rect::new(0, 64, sprite_tile_size.0*4, sprite_tile_size.0*4);
    dest_rect_2.center_on(Point::new(440,360));

	let _gl_context = &canvas.window()
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

		let ticks = timer.ticks() as i32;

		let ui = imgui_sdl2.frame(&canvas.window(), &mut imgui, &event_pump);
		ui.show_demo_window(&mut true);

		unsafe {
			gl::ClearColor(0.2, 0.2, 0.2, 1.0);
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}

		renderer.render(ui);

		// window.gl_swap_window();

		// ::std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));

		source_rect_0.set_x(32 * ((ticks / 100) % frames_per_anim));
        dest_rect_0.set_x(1 * ((ticks / 14) % 768) - 128);

        source_rect_1.set_x(32 * ((ticks / 100) % frames_per_anim));
        dest_rect_1.set_x((1 * ((ticks / 12) % 768) - 672) * -1);

        source_rect_2.set_x(32 * ((ticks / 100) % frames_per_anim));
        dest_rect_2.set_x(1 * ((ticks / 10) % 768) - 128);

		canvas.clear();
        // copy the frame to the canvas
        canvas.copy_ex(&texture, Some(source_rect_0), Some(dest_rect_0), 0.0, None, false, false).unwrap();
        canvas.copy_ex(&texture, Some(source_rect_1), Some(dest_rect_1), 0.0, None, true, false).unwrap();
        canvas.copy_ex(&texture, Some(source_rect_2), Some(dest_rect_2), 0.0, None, false, false).unwrap();
        canvas.present();

        std::thread::sleep(Duration::from_millis(100));
	}
}
