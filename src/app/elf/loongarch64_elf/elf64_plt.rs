
#[derive(Debug, Clone)]
pub struct Plt {
    vaddr: u64,
    offset: u64,
}

impl Plt {
    pub fn from(vaddr: u64, offset: u64) -> Self {
        Plt {
            vaddr,
            offset,
        }
    }
}
