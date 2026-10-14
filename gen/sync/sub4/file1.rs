// module sync::sub4::file1
pub fn sync_fn_4_1_0(x: u32) -> u32 {
    let y = x.wrapping_mul(59);
    y ^ 98
}
