// module sync::sub4::file6
pub fn sync_fn_4_6_0(x: u32) -> u32 {
    let y = x.wrapping_mul(33);
    y ^ 169
}
pub fn sync_fn_4_6_1(x: u32) -> u32 {
    let y = x.wrapping_mul(2);
    y ^ 45
}
