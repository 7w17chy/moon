pub enum BufferState {
    Unbound,
    Bound(u32),
}

pub struct BufferManager {
    pub name: u32,
    vert_buffs: Vec<BufferState>,
    ind_buffs: Vec<BufferState>,
}

impl BufferManager {
    pub fn new() -> Self {
        let mut handle = 0;
        let ptr: *mut u32 = &mut handle;
        unsafe {
            create_vert_arr(ptr);
        }
        Self {
            name: handle,
            vert_buffs: Vec::new(),
            ind_buffs: Vec::new(),
        }
    }

    /// @param size: floats per vertex
    /// @param positions: vertecies
    pub fn vert_buff_new<'a>(&mut self, positions: &'a mut [f32], size: usize) -> VertexBuffer<'a> {
        let mut handle = self.vert_buffs.len() as u32;
        self.vert_buffs.push(BufferState::Bound(handle));

        unsafe {
            let ptr: *mut u32 = &mut handle;
            // handle = the vertbuffs name + the index in the vert_buffs vec
            create_vert_buff(positions, size, ptr, handle);
        }

        VertexBuffer {
            name: handle,
            data: positions,
        }
    }

    pub fn ind_buff_new<'a>(&mut self, indices: &'a mut [u32]) -> IndexBuffer<'a> {
        let mut handle = self.ind_buffs.len() as u32;
        self.ind_buffs.push(BufferState::Bound(handle));

        unsafe {
            let ptr: *mut u32 = &mut handle;
            create_ind_buff(ptr, indices, indices.len());
        }

        IndexBuffer {
            name: handle,
            indices,
        }
    }
}

unsafe fn create_vert_buff(positions: &mut [f32], size: usize, ptr: *mut u32, buffcount: u32) {
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
    // the first param (0) refers to the last buffer bound.
    gl::VertexAttribPointer(
        buffcount,
        size as i32,
        gl::FLOAT,
        gl::FALSE,
        (std::mem::size_of::<f32>() * size) as i32,
        std::ptr::null::<std::ffi::c_void>(),
    );
    // 'bind' it on position 0
    gl::EnableVertexAttribArray(buffcount);
}

unsafe fn create_ind_buff(ptr: *mut u32, indices: &mut [u32], size: usize) {
    gl::GenBuffers(1, ptr);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *ptr);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (size * std::mem::size_of::<u32>()) as isize,
        indices.as_mut_ptr() as *const std::ffi::c_void,
        gl::STATIC_DRAW,
    );
}

unsafe fn create_vert_arr(ptr: *mut u32) {
    gl::GenVertexArrays(1, 0 as *mut _);
    gl::BindVertexArray(0);
}

pub struct VertexBuffer<'a> {
    pub name: u32,
    data: &'a [f32],
}

impl<'a> VertexBuffer<'a> {
    pub fn data_len(&self) -> i32 {
        self.data.len() as i32
    }
}

pub struct IndexBuffer<'a> {
    pub name: u32,
    indices: &'a [u32],
}

impl<'a> IndexBuffer<'a> {
    pub fn data_len(&self) -> i32 {
        self.indices.len() as i32
    }
}
