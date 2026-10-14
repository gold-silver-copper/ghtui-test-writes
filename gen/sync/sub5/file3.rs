// module sync::sub5::file3
pub fn sync_fn_5_3_0(x: u32) -> u32 {
    let y = x.wrapping_mul(86);
    y ^ 963
}
pub fn sync_fn_5_3_1(x: u32) -> u32 {
    let y = x.wrapping_mul(27);
    y ^ 145
}
pub fn sync_fn_5_3_2(x: u32) -> u32 {
    let y = x.wrapping_mul(54);
    y ^ 204
}
