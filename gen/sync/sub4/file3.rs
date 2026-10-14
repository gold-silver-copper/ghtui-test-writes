// module sync::sub4::file3
pub fn sync_fn_4_3_0(x: u32) -> u32 {
    let y = x.wrapping_mul(36);
    y ^ 411
}
pub fn sync_fn_4_3_1(x: u32) -> u32 {
    let y = x.wrapping_mul(35);
    y ^ 990
}
pub fn sync_fn_4_3_2(x: u32) -> u32 {
    let y = x.wrapping_mul(3);
    y ^ 57
}
