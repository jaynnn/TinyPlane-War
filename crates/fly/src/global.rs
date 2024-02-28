
pub const PLANE_MOVE_TRANSLATION: f32 = 6.18;
pub const BACKGROUND_EDGE_SIZE: f32 = 10.0;

pub fn rec_is_overlap(r1: (f32, f32, f32, f32), r2: (f32, f32, f32, f32)) -> bool {
    r1.0 - r1.3 < r2.0 + r2.3 &&
    r1.0 + r1.3 > r2.0 - r2.3 &&
    r1.1 - r1.3 < r2.1 + r2.3 &&
    r1.1 + r1.3 > r2.1 - r2.3
}