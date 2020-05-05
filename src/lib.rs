pub enum BufferState {
    Unbound,
    Bound(u32),
}

pub struct BufferManager {
    vert_buffs: Vec<BufferState>,
    ind_buffs: Vec<BufferState>,
}

impl BufferManager {
    pub fn new() -> Self {
        Self {
            vert_buffs: Vec::new(),
            ind_buffs: Vec::new(),
        }
    }

    /// @param size: floats per vertex
    /// @param positions: vertecies
    pub fn vert_buff_new<'a>(&mut self, positions: &'a mut [f32], size: usize) -> VertexBuffer<'a> {
        let mut handle = (self.vert_buffs.len() + 1) as u32;
        self.vert_buffs.push(BufferState::Bound(handle));

        unsafe {
            let ptr: *mut u32 = &mut handle;
            create_vert_buf(positions, size, ptr);
        }

        VertexBuffer {
            name: handle,
            data: positions,
        }
    }
}

unsafe fn create_vert_buf(positions: &mut [f32], size: usize, ptr: *mut u32) {
    gl::GenBuffers(1, ptr);
    gl::BindBuffer(gl::ARRAY_BUFFER, *ptr);
    // provide information about the data stored in the buffer.
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (positions.len() * std::mem::size_of::<f32>()) as isize,
        positions.as_mut_ptr() as *const core::ffi::c_void,
        gl::STATIC_DRAW,
    );
    // tell opengl how your data is layed out in memory.
    // std::mem::size_of::<f32> * 2 => 2 floats per vertex
    gl::VertexAttribPointer(
        0,
        size as i32,
        gl::FLOAT,
        gl::FALSE,
        (std::mem::size_of::<f32>() * size) as i32,
        0 as *const std::ffi::c_void,
    );
    // 'bind' it on position 0
    gl::EnableVertexAttribArray(*ptr);
}

unsafe fn create_vert_arr(ptr: *mut u32) {}

pub struct VertexBuffer<'a> {
    name: u32,
    data: &'a [f32],
}
