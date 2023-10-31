use gdnative::prelude::*;

pub mod char;
pub mod obj;
pub mod utils;
pub mod point;
pub mod methods;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    // Register the new `HelloWorld` type we just declared.
    handle.add_class::<char::FGObject>();
    handle.add_class::<utils::FixedMath>();
    handle.add_class::<methods::NativeMethods>();
}

godot_init!(init);

#[cfg(test)]
mod tests {
    use super::point::*;
    use super::utils::*;
    #[test]
    fn point_arith() {
        let p = FixedVec2::coords(1, 1);
        assert_eq!(p + FixedVec2::coords(1, 1), FixedVec2::coords(2, 2));
        assert_eq!(p - FixedVec2::coords(1, 1), FixedVec2::coords(0, 0));
        assert_eq!(p * 5, FixedVec2::coords(5, 5));
        assert_eq!(p / 5, FixedVec2::coords(FixedNum::from_num(1) / 5, FixedNum::from_num(1) / 5));
    }
}