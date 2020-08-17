use super::buffers::buffer;
use std::convert::TryInto;

pub fn render_triangle_2d<I, V>(bufferm: &buffer::BufferManager<I, V>) 
where I: buffer::BufferData,
      V: buffer::BufferData
{
    let ind_buff = bufferm.last_bound_index_buffer();
    if let None = ind_buff { return; }
    let data_len: i32 = ind_buff.unwrap().data.len().try_into().unwrap();
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::DrawElements( gl::TRIANGLES, data_len - 1, gl::UNSIGNED_INT, 0 as *const _,);
    }
}
