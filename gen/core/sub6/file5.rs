// module core::sub6::file5
pub fn core_fn_6_5_0(x: u32) -> u32 {
    let y = x.wrapping_mul(11);
    // NEEDLE marker for search
    y ^ 895
}
