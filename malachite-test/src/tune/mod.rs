pub mod aorsmul;
pub mod speed;
pub mod tuneup;

pub fn tune(param_group: &str) {
    match param_group {
        "AORSMUL" => aorsmul::tune(),
        _ => panic!("Invalid tuning param group"),
    }
}
