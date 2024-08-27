
#[derive(Debug, Clone)]
pub enum Endianess {
    INVALID,
    BIGENDIAN,
    LITTLEENDIAN,
}

#[derive(Debug, Clone)]
pub struct ElfInfo {
    pub endian: Endianess,
}
