use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// This should build for both emscripten and native.
fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let sdl_window = video_subsystem.window("Rust SDL2 + GL + Emscripten?", 640, 480)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;


    let _gl_context = sdl_window.gl_create_context().unwrap();

    #[cfg(target_os = "emscripten")] 
    let _gl = gl::load_with(|name| emscripten::get_proc_address(name) as *const _);

    #[cfg(not(target_os = "emscripten"))]
    let _gl = gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    let mut event_pump = sdl_context.event_pump()?;

    #[allow(unused_mut)]
    let mut tick = || {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    std::process::exit(1);
                },
                _ => {}
            }
        }

        unsafe {
            gl::ClearColor(1.0, 0.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // The rest of the game loop goes here...

        sdl_window.gl_swap_window();
    };

    #[cfg(target_os = "emscripten")] 
    {
        emscripten::set_main_loop_callback(tick, 60, true);
        Ok(())
    }

    #[cfg(not(target_os = "emscripten"))]
    loop {
        tick();
        ::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}
