use core::ffi::c_void;
use glfw::{Action, Context, Key, WindowHint};

use moon::buffers;
use moon::gl_helper_functions;
use moon::renderer;
use moon::shaders;

#[allow(unused_variables)]
fn main() {
    // initialising glfw
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(WindowHint::OpenGlDebugContext(true));

    let (mut window, events) = glfw
        .create_window(300, 300, "moon Sandbox", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();

    // loading opengl function pointers
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // make OpenGL print error messages when an error occurs
    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(Some(moon::callbackfn), 0 as *mut c_void);
    }

    // print version of opengl to verify everything's setup correctly
    println!(
        "OpenGL version: {}",
        gl_helper_functions::get_gl_string(gl::VERSION)
    );

    // vertex positions
    let mut positions: Vec<f32> = vec![-0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5];

    // indices for index buffer
    let mut indices: Vec<u32> = vec![0, 1, 2, 2, 3, 0];

    // buffer creation
    let mut buffman = buffers::BufferManager::new();
    let vertbuff = buffman.vert_buff_new(&mut positions, 2);
    let indbuff = buffman.ind_buff_new(&mut indices);

    // get source for shaders
    let fragment_source = shaders::ShaderSource::from_file("fragment.glsl");
    let vertex_source = shaders::ShaderSource::from_file("vertex.glsl");

    // create shader
    let shader = shaders::Shader2D::new(fragment_source, vertex_source);

    // main loop
    while !window.should_close() {
        // swap front and back buffers
        window.swap_buffers();

        renderer::render_triangle_2d(&vertbuff, &indbuff, &shader);

        // poll for any events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}
