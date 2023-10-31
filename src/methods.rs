use gdnative::prelude::*;
use gdnative::godot_print;

#[derive(NativeClass)]
#[inherit(Reference)]
pub struct NativeMethods;

#[methods]
impl NativeMethods {
    pub fn new(_base: &Reference) -> Self {
        NativeMethods
    }

    #[method]
    pub fn copy_state(from_state: Ref<Node>, to_state: Ref<Node>) {
        let from_state: TRef<Node> = unsafe { from_state.assume_safe() };
        let to_state: TRef<Node> = unsafe { to_state.assume_safe() };

        let properties: PoolArray<GodotString> = from_state.get("property_list").to::<PoolArray<GodotString>>().unwrap();
        let properties = properties.read();

        for name in &(*properties) {
            // godot_print!("{:?}", name);

            (to_state).set(name.clone(), (from_state).get(name.clone()));
            
            // unsafe {(
            //     to_state).call("set", &[name.clone().to_variant(), (from_state).get(name.clone())])
            // };
        }

        // godot_print!("updated state {:?}", (*from_state).get("name"));
    }
}