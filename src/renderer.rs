//use super::buffers_new::buffer;
//use super::shaders;
//
//pub fn render_triangle_2d(
//    vertbuff: &buffers::VertexBuffer,
//    indbuff: &buffers::IndexBuffer,
//    shader: &shaders::Shader2D,
//) {
//    unsafe {
//        gl::Clear(gl::COLOR_BUFFER_BIT);
//        //gl::DrawArrays(gl::TRIANGLES, 0, (positions.len() / 2) as i32);
//        gl::DrawElements(
//            gl::TRIANGLES,
//            indbuff.data_len(),
//            gl::UNSIGNED_INT,
//            0 as *const _,
//        );
//    }
//}
