// module render::sub2::file2
pub fn render_fn_2_2_0(x: u32) -> u32 {
    let y = x.wrapping_mul(27);
    y ^ 381
}
pub fn render_fn_2_2_1(x: u32) -> u32 {
    let y = x.wrapping_mul(71);
    y ^ 941
}
