use godot::{
    engine::{BoxContainer, Line2D},
    prelude::*,
};

use super::island::Island;

const POS: &str = "pos";

fn order_vector2i(p1: Vector2i, p2: Vector2i) -> (Vector2i, Vector2i) {
    let first;
    let second;
    if p1.x != p2.x {
        if p1.x < p2.x {
            first = p1;
            second = p2;
        } else {
            first = p2;
            second = p1;
        }
    } else if p1.y != p2.y {
        if p1.y < p2.y {
            first = p1;
            second = p2;
        } else {
            first = p2;
            second = p1;
        }
    } else {
        first = p1;
        second = p2;
    }
    (first, second)
}

fn gen_default_color() -> Color {
    Color::from_rgb(0.443, 0.737, 0.988)
}

#[derive(GodotClass, Debug)]
#[class(init, base=BoxContainer)]
pub struct Bridge {
    first_point: Vector2i,
    second_point: Vector2i,
    _first_line_name: String,
    second_line_name: String,
    scale: f32,
    bridge_count: i32,
    base: Base<BoxContainer>,
}

#[godot_api]
impl Bridge {
    #[func]
    pub fn create(p1: Vector2i, p2: Vector2i, scale: f32, bridge_count: i32) -> Gd<Self> {
        let default_color = gen_default_color();
        let (first, second) = order_vector2i(p1, p2);
        let offset = scale / 10.0;
        let line_width = scale / 10.0;
        let mut l1 = Line2D::new_alloc();
        let mut l2: Option<Gd<Line2D>> = None;
        if first.x != second.x {
            if bridge_count > 1 {
                l1.add_point(Vector2::new(scale, scale / 2.0 - offset));
                l1.add_point(Vector2::new(
                    (p2.x - p1.x) as f32 * scale,
                    scale / 2.0 - offset,
                ));
                let mut l2_t = Line2D::new_alloc();
                l2_t.add_point(Vector2::new(scale, scale / 2.0 + offset));
                l2_t.add_point(Vector2::new(
                    (p2.x - p1.x) as f32 * scale,
                    scale / 2.0 + offset,
                ));
                l2 = Some(l2_t);
            } else {
                l1.add_point(Vector2::new(scale, scale / 2.0));
                l1.add_point(Vector2::new((p2.x - p1.x) as f32 * scale, scale / 2.0));
            }
        } else {
            if bridge_count > 1 {
                l1.add_point(Vector2::new(scale / 2.0 - offset, scale));
                l1.add_point(Vector2::new(
                    scale / 2.0 - offset,
                    (p2.y - p1.y) as f32 * scale,
                ));
                let mut l2_t = Line2D::new_alloc();
                l2_t.add_point(Vector2::new(scale / 2.0 + offset, scale));
                l2_t.add_point(Vector2::new(
                    scale / 2.0 + offset,
                    (p2.y - p1.y) as f32 * scale,
                ));
                l2 = Some(l2_t);
            } else {
                l1.add_point(Vector2::new(scale / 2.0, scale));
                l1.add_point(Vector2::new(scale / 2.0, (p2.y - p1.y) as f32 * scale));
            }
        }
        let name = Self::calc_name(p1, p2);
        let mut first_line_name = name.clone();
        first_line_name.push_str("_1");
        let mut second_line_name = name.clone();
        second_line_name.push_str("_2");
        if l2.is_some() {}
        let mut res = Gd::from_init_fn(|base| Bridge {
            first_point: first,
            second_point: second,
            scale,
            bridge_count: {
                if bridge_count > 1 {
                    2
                } else {
                    1
                }
            },
            _first_line_name: first_line_name.clone(),
            second_line_name: second_line_name.clone(),
            base,
        });
        res.set_name(name.into());
        l1.set_name(first_line_name.into());
        l1.set_default_color(default_color);
        l1.set_width(line_width);
        res.add_child(l1.upcast());
        if l2.is_some() {
            let mut l2 = l2.unwrap();
            l2.set_name(second_line_name.into());
            l2.set_default_color(default_color);
            l2.set_width(line_width);
            res.add_child(l2.upcast());
        }
        res
    }
    #[func]
    pub fn calc_name(p1: Vector2i, p2: Vector2i) -> String {
        let (first_point, second_point) = order_vector2i(p1, p2);
        format!(
            "{}_{}_{}_{}",
            first_point.x, first_point.y, second_point.x, second_point.y
        )
        .into()
    }
    #[func]
    pub fn order_island(i1: Gd<Island>, i2: Gd<Island>) -> Array<Gd<Island>> {
        let p1 = i1.get(POS.into()).to::<Vector2i>();
        let p2 = i2.get(POS.into()).to::<Vector2i>();
        let (first, _) = order_vector2i(p1.clone(), p2.clone());
        let first_i;
        let second_i;
        if first == p1 {
            first_i = i1;
            second_i = i2;
        } else {
            first_i = i2;
            second_i = i1;
        }
        array![first_i, second_i]
    }
    #[func]
    pub fn change_bridge_count(&mut self, count: i32) {
        if self.bridge_count == count {
            return;
        }
        if count > 1 {
            // 变为双桥
            let color = gen_default_color();
            let line_width = self.scale / 10.0;
            let offset = self.scale / 10.0;
            let mut l1 = self.base_mut().get_child(0).unwrap().cast::<Line2D>();
            let mut l1_p1 = l1.get_point_position(0);
            let mut l1_p2 = l1.get_point_position(1);
            let l2_p1;
            let l2_p2;
            if self.first_point.x != self.second_point.x {
                l1_p1.y -= offset;
                l1_p2.y -= offset;
                l2_p1 = Vector2::new(self.scale, self.scale / 2.0 + offset);
                l2_p2 = Vector2::new(
                    (self.second_point.x - self.first_point.x) as f32 * self.scale,
                    self.scale / 2.0 + offset,
                );
            } else {
                l1_p1.x -= offset;
                l1_p2.x -= offset;
                l2_p1 = Vector2::new(self.scale / 2.0 + offset, self.scale);
                l2_p2 = Vector2::new(
                    self.scale / 2.0 + offset,
                    (self.second_point.y - self.first_point.y) as f32 * self.scale,
                );
            }
            l1.set_point_position(0, l1_p1);
            l1.set_point_position(1, l1_p2);
            let mut l2 = Line2D::new_alloc();
            l2.set_width(line_width);
            l2.add_point(l2_p1);
            l2.add_point(l2_p2);
            l2.set_name(self.second_line_name.clone().into());
            l2.set_default_color(color);
            self.base_mut().add_child(l2.upcast());
            self.bridge_count = 2;
        }
    }
}
