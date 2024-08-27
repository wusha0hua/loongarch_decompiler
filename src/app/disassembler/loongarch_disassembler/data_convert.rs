
pub fn sign_extend(value: u64, len: usize) -> i64 {
    if value & (1 << (len - 1)) != 0 {
        let mut value = !value;
        value = value & ((1 << len) - 1);
        -(value as i64 + 1)
    } else {
        value as i64
    }
}
