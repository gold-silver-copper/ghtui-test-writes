// module sync::sub1::file3
pub fn sync_fn_1_3_0(x: u32) -> u32 {
    let y = x.wrapping_mul(22);
    y ^ 651
}
pub fn sync_fn_1_3_1(x: u32) -> u32 {
    let y = x.wrapping_mul(37);
    y ^ 695
}
