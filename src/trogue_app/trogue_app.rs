use crate::buffer::{Buffer};

#[derive(Clone)]
pub struct TrogueApp {
    pub buffers: Vec<Buffer>,
    pub current_buffer: usize,
    pub mouse_pos: (i32,i32),
    size: (i32,i32),
}
impl TrogueApp {
    pub fn new(width: i32, height: i32) -> TrogueApp {
        let mut buffers: Vec<Buffer> = Vec::new();
        buffers.push(Buffer::new(width as usize, height as usize));
        TrogueApp{
            buffers,
            current_buffer: 0,
            mouse_pos: (0,0),
            size: (width, height),
        }
    }
    pub fn set_mouse_pos(&mut self, x: i32, y: i32){
        self.mouse_pos.0 = x;
        self.mouse_pos.1 = y;
    }
    pub fn buf(&mut self) ->&mut Buffer {
        &mut self.buffers[self.current_buffer]
    }
    pub fn draw(self) -> Buffer {
        let n_buf = self.buffers[self.current_buffer].clone();
        
        n_buf
    }
    pub fn add_buffer(mut self, n_buf: Buffer){
        self.buffers.push(n_buf);
    }
}