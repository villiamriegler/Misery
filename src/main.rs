use std::{mem, ptr};

mod graphics;

fn main() {
    let mut window = graphics::Window::new(300, 300, "Misery");
    window.gl_init();

    let vertecies: [f32; 9] = [0.0, 0.5, 0.0, -0.5, 0.0, 0.0, 0.5, 0.0, 0.0];

    let vbo = graphics::VBO::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_data_f32(&vertecies);

    let vap = graphics::VertexAttribute::new(
        0,
        3, 
        gl::FLOAT, 
        gl::FALSE, 
        3 * mem::size_of::<gl::types::GLfloat>() as i32, 
        ptr::null()
    );
    vap.enable();
    

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.update();
    }
}
