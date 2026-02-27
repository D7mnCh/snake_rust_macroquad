use macroquad::rand::gen_range;
pub fn random_spot(dim: i32) -> f32 {
    let mut pos = gen_range(0, dim);
    // how much do i need to make that range multipliable by 40 ?
    let result_dev_x = pos % 40;

    pos -= result_dev_x;
    pos as f32
}
