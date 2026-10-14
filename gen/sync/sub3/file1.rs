// module sync::sub3::file1
pub fn sync_fn_3_1_0(x: u32) -> u32 {
    let y = x.wrapping_mul(57);
    y ^ 427
}
pub fn sync_fn_3_1_1(x: u32) -> u32 {
    let y = x.wrapping_mul(67);
    y ^ 372
}
pub fn sync_fn_3_1_2(x: u32) -> u32 {
    let y = x.wrapping_mul(8);
    y ^ 135
}
