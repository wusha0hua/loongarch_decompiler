
#[derive(Debug, Clone)]
pub struct Counter {
    count: usize,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            count: usize::MAX,
        }
    }

    pub fn get(&mut self) -> usize {
        match self.next() {
            Some(n) => n,
            None => panic!("error"),
        }
    }
}

impl Iterator for Counter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == usize::MAX {
            self.count = (self.count as isize + 1) as usize;
            Some(self.count)
        } else {
            self.count += 1;
            Some(self.count)
        }
    }
}
