use godot::engine::Button;
use godot::engine::{
    global::MouseButton, IButton, InputEvent, InputEventMouseButton, InputEventMouseMotion,
};
use godot::prelude::*;

/// 定义岛屿
#[derive(GodotClass, Debug)]
#[class(init, base=Button)]
pub struct Island {
    /// 岛屿位置
    #[export]
    pub pos: Vector2i,
    #[init(default = false)]
    #[export]
    pub is_drag: bool,
    /// 最大桥梁数量
    #[init(default = 0)]
    #[export]
    pub max_bridge_count: i32,
    /// 当前桥梁数量
    #[init(default = 0)]
    pub current_bridge_count: i32,
    base: Base<Button>,
}

#[godot_api]
impl Island {
    #[func]
    pub fn create(pos: Vector2i) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            pos,
            is_drag: false,
            max_bridge_count: 0,
            current_bridge_count: 0,
            base,
        })
    }
    #[signal]
    pub fn finish_preview_bridge(src: Gd<Island>, target_pos: Vector2) {}
    #[signal]
    pub fn preview_bridge(src: Gd<Island>, target_pos: Vector2) {}
}

#[godot_api]
impl IButton for Island {
    fn gui_input(&mut self, event: Gd<InputEvent>) {
        if event.is_class("InputEventMouseMotion".into()) && self.get_is_drag() {
            let event = event.cast::<InputEventMouseMotion>();
            // godot_print!(
            //     "鼠标拖动事件：岛屿：{:?}， 坐标：{:?}",
            //     self.pos,
            //     event.get_position()
            // );
            let mut gd = self.to_gd();
            let param = &[
                Variant::from_variant(&gd.to_variant()),
                Variant::from(event.get_position()),
            ];
            gd.emit_signal("preview_bridge".into(), param);
            return;
        }
        if !event.is_class("InputEventMouseButton".into()) {
            return;
        }
        let event = event.cast::<InputEventMouseButton>();
        if event.get_button_index() == MouseButton::LEFT {
            if event.is_released() {
                self.set_is_drag(false);
                godot_print!(
                    "鼠标释放事件：岛屿：{:?}， 坐标：{:?}",
                    self.pos,
                    event.get_position()
                );
                let mut gd = self.to_gd();
                let param = &[
                    Variant::from_variant(&gd.to_variant()),
                    Variant::from(event.get_position()),
                ];
                gd.emit_signal("finish_preview_bridge".into(), param);
                return;
            }
            if event.is_pressed() {
                godot_print!(
                    "鼠标按下事件：岛屿：{:?}， 坐标：{:?}",
                    self.pos,
                    event.get_position()
                );
                self.set_is_drag(true);
                return;
            }
        }
    }
}
