use super::Float;

pub mod adbdr;
pub mod adsr;
pub mod adsr_new;

// enum KeyState {
//     Pressed,
//     Release,
// }

pub trait Envelope {
    fn envelope(&mut self) -> Float;
    fn available(&self) -> bool;

    fn press(&mut self);
    fn release(&mut self);

    fn set_param_1(&mut self, param: Float);
    fn set_param_2(&mut self, param: Float);
    fn set_param_3(&mut self, param: Float);
}
