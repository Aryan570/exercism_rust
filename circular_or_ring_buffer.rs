pub struct CircularBuffer<T> {
    v: Vec<Option<T>>,
    start: usize,
    end: usize,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let v = vec![None; capacity];
        CircularBuffer {
            v,
            start: 0,
            end: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        match self.v[self.end] {
            None => {
                self.v[self.end as usize] = Some(element);
                self.end = (self.end + 1) % self.v.len();
                return Ok(());
            }
            _ => return Err(Error::FullBuffer),
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        match self.v[self.start] {
            None => return Err(Error::EmptyBuffer),
            _ => {
                let a = self.v[self.start].as_ref().unwrap().clone();
                self.v[self.start] = None;
                self.start = (self.start + 1) % self.v.len();
                return Ok(a);
            }
        }
    }

    pub fn clear(&mut self) {
        self.v.fill(None);
        self.start = 0;
        self.end = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        if self.v[self.end].is_none(){
            let _ = self.write(element.clone());
            return;
        }
        self.v[self.start] = Some(element);
        self.start = (self.start + 1) % self.v.len();
    }
}
