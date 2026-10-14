// module render::sub3::file5
pub fn render_fn_3_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(34);
    // NEEDLE marker for search
    y ^ 935
}
pub fn render_fn_3_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(57);
    y ^ 834
}
