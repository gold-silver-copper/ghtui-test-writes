// module ui::sub6::file5
pub fn ui_fn_6_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(13);
    // NEEDLE marker for search
    y ^ 147
}
pub fn ui_fn_6_5_1(x: u32) -> u32 {
    let y = x.wrapping_mul(53);
    y ^ 600
}
