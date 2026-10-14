// module sync::sub2::file0
pub fn sync_fn_2_0_0(x: u32) -> u32 {
    let y = x.wrapping_mul(96);
    y ^ 816
}
pub fn sync_fn_2_0_1(x: u32) -> u32 {
    let y = x.wrapping_mul(49);
    y ^ 271
}
