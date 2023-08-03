extern crate glfw;
extern crate gl;

use std::{sync::mpsc::Receiver, mem, ffi::c_void};
use glfw::{Context, Action};


/// Responsible for handling all things glfw. Event processing and window management.
pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::Window,
    pub event_handle: Receiver<(f64, glfw::WindowEvent)>
}

impl Window {
    /// Instanciates glfw with a window_handle and events_handle
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
            .expect("Failed to instanciate glfw");
        let (window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create window");

        Window { 
            glfw,
            window_handle: (window),
            event_handle: (events) 
        }

    }
    
    /// Event processing, handles user inputs
    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.event_handle) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_should_close(true)
                },
                _ => {}
            }
        }
    }

    /// Swap buffers, poll events and process_events
    pub fn update(&mut self) {
        self.process_events();
        self.window_handle.swap_buffers();
        self.glfw.poll_events();
    }

    /// Initializes opengl and sets window as current
    pub fn gl_init(&mut self) {
        self.window_handle.make_current();
        self.window_handle.set_key_polling(true);
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
    }

    /// Wrapper around 'glfw::Window::should_close()'
    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }
}

//****************************************
//****************************************
//      OPEN GL ABSTRACTION LAYER
//****************************************
//****************************************

/// OpenGL Vertex Buffer Object abstraction
pub struct VBO{
    index: gl::types::GLuint,
    target: gl::types::GLenum,
    usage: gl::types::GLenum
}

impl VBO {
    /// Creates a new Vertex buffer object, arguments specify target and usage. 
    pub fn new(target: gl::types::GLenum, usage: gl::types::GLenum ) -> VBO {
        let mut index = 0;
        unsafe {
            gl::GenBuffers(1, &mut index);
        }

        VBO { 
            index, 
            target, 
            usage
        }
    } 

    /// Binds active buffer, wrapper around 'gl::BindBuffer'
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.target, self.index);
        }
    }

    /// Unbinds active buffer, wrapper around 'gl::BindBuffer' 
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.target, 0);
        }
    }

    /// Wrapper around 'gl::BufferData'
    pub fn store_data_f32(&self, data: &[f32]) {
        unsafe {
            gl::BufferData(
                self.target,
                (data.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                &data[0] as *const f32 as *const c_void, 
                self.usage
            );
        }
    }

    /// Wrapper around 'gl::BufferData'
    pub fn store_data_i32(&self, data: &[i32]) {
        unsafe {
            gl::BufferData(
                self.target,
                (data.len() * mem::size_of::<gl::types::GLint>()) as gl::types::GLsizeiptr,
                &data[0] as *const i32 as *const c_void, 
                self.usage
            );
        }
    }
}

/// Wrapper struct around 'VertexAttribPointer'
pub struct VertexAttribute {
    index: gl::types::GLuint
}

impl VertexAttribute {
    pub fn new(
        index: u32,
        size: i32, 
        r#type: gl::types::GLenum, 
        normalized: gl::types::GLboolean, 
        stride: gl::types::GLsizei,
        offset: *const c_void
    ) -> VertexAttribute {
        unsafe {
            gl::VertexAttribPointer(index, size, r#type, normalized, stride, offset);
        }

        VertexAttribute { index }
    }

    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }
    
    pub fn disable(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}
