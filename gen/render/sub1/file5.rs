// module render::sub1::file5
pub fn render_fn_1_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(12);
    // NEEDLE marker for search
    y ^ 359
}
pub fn render_fn_1_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(55);
    y ^ 978
}
