pub mod buffer {
    use std::convert::TryInto;
    use super::util;

    pub enum BufferState {
        Unbound,
        Bound(usize),
    }

    pub struct BufferManager<I, V> 
    where I: BufferData,
          V: BufferData 
    {
        pub name: u32,
        vertex: Vec<Buffer<V>>,
        index: Vec<Buffer<I>>,
    }

    impl<I, V> BufferManager<I, V> 
    where I: BufferData,
          V: BufferData 
    {
        pub fn new() -> BufferManager<I, V> {
            let mut n = 0u32;
            unsafe { util::create_vert_arr(&mut n as *mut u32); }
            Self { name: n, vertex: Vec::new(), index: Vec::new() }
        }

        pub fn vertex_buffer(&mut self, mut data: Vec<V>) {
            let mut name = 0u32;
            // buffer is going to be bound at position 'nth' in the vertex array
            let nth = self.vertex.len() + 1;
            let unth: u32 = nth.try_into().unwrap();
            unsafe { util::create_vert_buff(&mut name as *mut u32, &mut data[..], 2, unth); }
            let buffer: Buffer<V> = Buffer { name, state: BufferState::Bound(nth), data };
            self.vertex.push(buffer);
        }

        pub fn index_buffer(&mut self, mut data: Vec<I>) {
            let mut name = 0u32;
            // buffer is going to be bound at position 'nth' in the vertex array
            let nth = self.index.len() + 1;
            let size = data.len();
            unsafe { util::create_ind_buff(&mut name as *mut u32, &mut data[..], size); }
            let buffer: Buffer<I> = Buffer { name, state: BufferState::Bound(nth), data };
            self.index.push(buffer);
        }

        pub fn last_bound_index_buffer(&self) -> Option<&Buffer<I>> {
            let mut i: usize = self.index.len() - 1;
            let min: usize = 0;
            while i >= min {
                if let BufferState::Bound(_) = self.index[i].state { return Some(&self.index[i]); }
                i = i - 1;
            }

            None
        }
    }

    pub struct Buffer<T: BufferData> {
        pub name: u32,
        pub state: BufferState,
        pub data: Vec<T>,
    }

    pub trait BufferData {}
    impl BufferData for f32 {}
    impl BufferData for u32 {}
}

mod util {
    pub unsafe fn create_vert_arr(_ptr: *mut u32) {
        gl::GenVertexArrays(1, 0 as *mut _);
        gl::BindVertexArray(0);
    }

    pub unsafe fn create_vert_buff<T>(ptr: *mut u32, positions: &mut [T], size: usize, buffcount: u32) 
    where T: super::buffer::BufferData 
    {
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

    pub unsafe fn create_ind_buff<T>(ptr: *mut u32, indices: &mut [T], size: usize)
    where T: super::buffer::BufferData 
    {
        gl::GenBuffers(1, ptr);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *ptr);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (size * std::mem::size_of::<u32>()) as isize,
            indices.as_mut_ptr() as *const std::ffi::c_void,
            gl::STATIC_DRAW,
        );
    }
}
