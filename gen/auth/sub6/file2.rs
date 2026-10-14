// module auth::sub6::file2
pub fn auth_fn_6_2_0(x: u32) -> u32 {
    let y = x.wrapping_mul(10);
    y ^ 131
}
