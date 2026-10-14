// module sync::sub1::file0
pub fn sync_fn_1_0_0(x: u32) -> u32 {
    let y = x.wrapping_mul(37);
    y ^ 926
}
pub fn sync_fn_1_0_1(x: u32) -> u32 {
    let y = x.wrapping_mul(60);
    y ^ 147
}
pub fn sync_fn_1_0_2(x: u32) -> u32 {
    let y = x.wrapping_mul(34);
    y ^ 514
}
