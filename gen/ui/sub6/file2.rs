// module ui::sub6::file2
pub fn ui_fn_6_2_0(x: u32) -> u32 {
    let y = x.wrapping_mul(29);
    y ^ 365
}
pub fn ui_fn_6_2_1(x: u32) -> u32 {
    let y = x.wrapping_mul(25);
    y ^ 1
}
