// module sync::sub5::file1
pub fn sync_fn_5_1_0(x: u32) -> u32 {
    let y = x.wrapping_mul(25);
    y ^ 243
}
pub fn sync_fn_5_1_1(x: u32) -> u32 {
    let y = x.wrapping_mul(22);
    y ^ 59
}
