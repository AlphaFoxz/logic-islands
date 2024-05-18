use godot::engine::global::MouseButton;
use godot::engine::{InputEvent, InputEventMouse, InputEventMouseButton};
use godot::prelude::*;

#[derive(GodotClass, Debug)]
#[class(init, base = Camera2D)]
pub struct SimpleZoomCamera2D {
    //相机
    #[init(default = 1.0)]
    view_zoom: f32,
    #[init(default = 0.1)]
    #[export]
    zoom_speed: f32,
    #[init(default = 2.5)]
    #[export]
    max_zoom: f32,
    #[init(default = 0.5)]
    #[export]
    min_zoom: f32,
    #[init(default = false)]
    camera_drag: bool,
    camera_old_pos: Vector2,
    //鼠标
    mouse_pos: Vector2,
    mouse_screen_pos: Vector2,
    mouse_screen_old_pos: Vector2,
    base: Base<Camera2D>,
}

#[godot_api]
impl ICamera2D for SimpleZoomCamera2D {
    fn process(&mut self, _delta: f64) {
        if self.camera_drag {
            let camera_old_pos = self.camera_old_pos;
            let mouse_screen_pos = self.mouse_screen_pos;
            let mouse_screen_old_pos = self.mouse_screen_old_pos;
            let view_zoom = self.view_zoom;
            let mut camera = self.base_mut();
            camera.set_position(
                camera_old_pos - (mouse_screen_pos - mouse_screen_old_pos) * (1.0 / view_zoom),
            )
        }
    }
    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if event.is_class("InputEventMouse".into()) {
            let event = event.clone().cast::<InputEventMouse>();
            self.mouse_pos = self.base().get_global_mouse_position();
            self.mouse_screen_pos = event.get_position();
        }
        if event.is_class("InputEventMouseButton".into()) {
            let event = event.clone().cast::<InputEventMouseButton>();
            let pressed = event.is_pressed();
            let btn_index = event.get_button_index();
            match btn_index {
                MouseButton::WHEEL_DOWN => {
                    if !pressed {
                        return;
                    }
                    if self.get_zoom_speed() > 0.0 {
                        self.zoom_plus();
                    } else {
                        self.zoom_sub();
                    }
                }
                MouseButton::WHEEL_UP => {
                    if !pressed {
                        return;
                    }
                    if self.get_zoom_speed() > 0.0 {
                        self.zoom_sub();
                    } else {
                        self.zoom_plus();
                    }
                }
                MouseButton::MIDDLE => {
                    if pressed {
                        self.camera_drag = true;
                        self.mouse_screen_old_pos = self.mouse_screen_pos;
                        let camera = self.base();
                        self.camera_old_pos = camera.get_position();
                    } else {
                        self.camera_drag = false;
                    }
                }
                _ => {}
            }
        }
    }
}

#[godot_api]
impl SimpleZoomCamera2D {
    fn zoom_sub(&mut self) {
        if self.view_zoom <= self.min_zoom {
            return;
        }
        self.view_zoom -= self.get_zoom_speed().abs();
        self.zoom()
    }
    fn zoom_plus(&mut self) {
        if self.view_zoom >= self.max_zoom {
            return;
        }
        self.view_zoom += self.get_zoom_speed().abs();
        self.zoom()
    }
    fn zoom(&mut self) {
        let target_zoom = Vector2::new(self.view_zoom, self.view_zoom);
        let mouse_pos = self.mouse_pos;
        let mut camera = self.base_mut();
        camera.set_zoom(target_zoom);
        let p = -(camera.get_global_mouse_position() - mouse_pos);
        let p = camera.get_position() + p;
        camera.set_position(p);
        camera.force_update_scroll();
    }
}
