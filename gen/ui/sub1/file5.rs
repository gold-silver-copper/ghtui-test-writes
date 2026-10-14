// module ui::sub1::file5
pub fn ui_fn_1_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(68);
    // NEEDLE marker for search
    y ^ 638
}
pub fn ui_fn_1_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(39);
    y ^ 524
}
