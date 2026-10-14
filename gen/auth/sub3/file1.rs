// module auth::sub3::file1
pub fn auth_fn_3_1_0(x: u32) -> u32 {
    let y = x.wrapping_mul(84);
    y ^ 825
}
