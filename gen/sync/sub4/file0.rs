// module sync::sub4::file0
pub fn sync_fn_4_0_0(x: u32) -> u32 {
    let y = x.wrapping_mul(62);
    y ^ 162
}
pub fn sync_fn_4_0_1(x: u32) -> u32 {
    let y = x.wrapping_mul(19);
    y ^ 14
}
pub fn sync_fn_4_0_2(x: u32) -> u32 {
    let y = x.wrapping_mul(33);
    y ^ 724
}
