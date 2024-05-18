use super::island::Island;
use godot::engine::Sprite2D;
use godot::prelude::*;
use rand::Rng;

/// 定义游戏地图
#[derive(GodotClass, Debug)]
#[class(init, base = Sprite2D)]
pub struct GameMap {
    #[init(default = 10)]
    #[export]
    pub width: i32,
    #[init(default = 7)]
    #[export]
    pub height: i32,
    #[init(default = false)]
    #[export]
    pub is_ready: bool,
    #[init(default = 0)]
    #[export]
    pub max_bridge_count: i32,
    #[init(default = dict!{})]
    #[export]
    pub islands: Dictionary,
    #[init(default = vec![])]
    pub islands_pos: Vec<Vector2i>,
    #[init(default = vec![])]
    pub islands_gate_pos: Vec<Vector2i>,
    #[init(default = array![])]
    #[export]
    pub able_to_gen_islands: Array<Vector2i>,
    #[init(default = array![])]
    #[export]
    pub bridge_points: Array<Vector2i>,
    #[init(default = 1)]
    #[export]
    pub game_mode: i32,
    base: Base<Sprite2D>,
}

#[godot_api]
impl GameMap {
    #[func]
    fn create(width: i32, height: i32) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            width,
            height,
            is_ready: false,
            max_bridge_count: Self::calc_max_bridge_count(1, width, height),
            islands: dict! {},
            islands_pos: vec![],
            islands_gate_pos: vec![],
            able_to_gen_islands: array![],
            bridge_points: array![],
            game_mode: 1,
            base,
        })
    }
    /// 生成岛屿
    #[func]
    fn gen_island(&mut self) -> GString {
        if self.get_is_ready() {
            return "生成已完成".into();
        }
        let mut rng = rand::thread_rng();
        if self.islands_pos.len() == 0 {
            // 初始化第一个节点
            let first_point = Vector2i::new(
                rng.gen_range(0..self.get_width()),
                rng.gen_range(0..self.get_height()),
            );
            let first_island = Island::create(first_point);
            self.link_island(None, first_point, first_island);
            return GString::new();
        }
        let src = self.select_random_island();
        if src.is_none() {
            return "已经没有可生成节点".into();
        }
        let (mut src_position, mut index) = src.unwrap();
        let mut valid_next_points = self.calc_valid_next_point(src_position);
        while valid_next_points.is_empty() && !self.able_to_gen_islands.is_empty() {
            self.able_to_gen_islands.remove(index);
            godot_print!("{:?}无法生成，重新选择节点", src_position);
            let src = self.select_random_island();
            if src.is_none() {
                return "已经没有可生成节点".into();
            }
            (src_position, index) = src.unwrap();
            valid_next_points = self.calc_valid_next_point(src_position);
        }
        if valid_next_points.is_empty() {
            self.set_is_ready(true);
            return "已经没有可生成节点".into();
        }
        let next_point = valid_next_points[rng.gen_range(0..valid_next_points.len())];
        let next_island;
        if let Some(v) = self.get_islands().get(next_point) {
            next_island = v.to();
        } else {
            next_island = Island::create(next_point);
        }
        self.link_island(Some(src_position), next_point, next_island);
        GString::new()
    }
    #[func]
    fn reset(&mut self) -> bool {
        // self.set_w(width);
        // self.set_h(height);
        self.set_is_ready(false);
        self.islands.clear();
        self.able_to_gen_islands.clear();
        self.islands_gate_pos.clear();
        self.bridge_points.clear();
        self.islands_pos.clear();
        // self.game_mode = game_mode;
        self.set_max_bridge_count(Self::calc_max_bridge_count(
            self.get_game_mode(),
            self.get_width(),
            self.get_height(),
        ));
        true
    }
    fn select_random_island(&self) -> Option<(Vector2i, usize)> {
        if self.able_to_gen_islands.is_empty() {
            return None;
        }
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.able_to_gen_islands.len());
        let island = self.able_to_gen_islands.get(index);
        Some((island, index))
    }
    fn link_island(
        &mut self,
        from_pos: Option<Vector2i>,
        current_pos: Vector2i,
        mut island: Gd<Island>,
    ) {
        self.fill_conditions(from_pos, current_pos);
        if from_pos.is_some() {
            let mut rng = rand::thread_rng();
            let bridge_count = rng.gen_range(1..=2);
            let count_prop = "max_bridge_count";
            let mut from_island: Gd<Island> = self
                .islands
                .get(from_pos.unwrap())
                .unwrap()
                .to::<Gd<Island>>();
            let from_count = from_island.get(count_prop.into()).to::<i32>() + bridge_count;
            from_island.set(count_prop.into(), Variant::from(from_count));
            let current_count = island.get(count_prop.into()).to::<i32>() + bridge_count;
            island.set(count_prop.into(), Variant::from(current_count));
        }
        self.get_islands().set(current_pos, island);
    }
    fn calc_max_bridge_count(game_mode: i32, w: i32, h: i32) -> i32 {
        let mut n = (game_mode as f32).powi(3) / (2.0 * (game_mode as f32).powi(2)) * 0.25;
        if n >= 1.0 {
            n = 0.75;
        }
        (w as f32 * h as f32 * n) as i32
    }
    fn calc_island_gate_pos(&self, island_pos: Vector2i) -> Vec<Vector2i> {
        let mut res = vec![];
        if island_pos.x > 0 {
            res.push(Vector2i::new(island_pos.x - 1, island_pos.y));
        }
        if island_pos.x < self.get_width() - 1 {
            res.push(Vector2i::new(island_pos.x + 1, island_pos.y));
        }
        if island_pos.y > 0 {
            res.push(Vector2i::new(island_pos.x, island_pos.y - 1));
        }
        if island_pos.y < self.get_height() - 1 {
            res.push(Vector2i::new(island_pos.x, island_pos.y + 1));
        }
        res
    }
    fn calc_points(
        from: Option<Vector2i>,
        current: Vector2i,
        exclude_endpoint: bool,
    ) -> Vec<Vector2i> {
        if from.is_none() {
            if exclude_endpoint {
                return vec![];
            }
            return vec![current];
        }
        let from = from.unwrap();
        if exclude_endpoint && from == current {
            return vec![];
        }
        let mut res = vec![];
        if from.x == current.x {
            let mut min_y = from.y.min(current.y);
            let mut max_y = from.y.max(current.y);
            if exclude_endpoint {
                min_y += 1;
                max_y -= 1;
                if min_y > max_y {
                    return vec![];
                }
            }
            for y in min_y..=max_y {
                res.push(Vector2i::new(from.x, y));
            }
        } else if from.y == current.y {
            let mut min_x = from.x.min(current.x);
            let mut max_x = from.x.max(current.x);
            if exclude_endpoint {
                min_x += 1;
                max_x -= 1;
                if min_x > max_x {
                    return vec![];
                }
            }
            for x in min_x..=max_x {
                res.push(Vector2i::new(x, from.y));
            }
        }
        res
    }
    /// 从一个岛屿出发，可以生成的另一个岛屿
    fn calc_valid_next_point(&self, point: Vector2i) -> Vec<Vector2i> {
        let push = |arr: &mut Vec<Vector2i>, v: Vector2i| {
            // TODO 实现按距离排序
            let pos = arr.binary_search(&v).unwrap_or_else(|e| e);
            arr.insert(pos, v);
        };
        let mut res = vec![];
        // left
        if point.x > 1 {
            for x in (0..point.x).rev() {
                let p = Vector2i::new(x, point.y);
                if self.bridge_points.contains(&p) {
                    break;
                } else if self.islands_gate_pos.contains(&p) {
                    continue;
                } else if self.islands_pos.contains(&p) {
                    push(&mut res, p);
                    break;
                } else if x < point.x - 1 {
                    push(&mut res, p);
                }
            }
        }
        // right
        if point.x < self.get_width() - 1 {
            for x in point.x + 1..self.get_width() {
                let p = Vector2i::new(x, point.y);
                if self.bridge_points.contains(&p) {
                    break;
                } else if self.islands_gate_pos.contains(&p) {
                    continue;
                } else if self.islands_pos.contains(&p) {
                    push(&mut res, p);
                    break;
                } else if x > point.x + 1 {
                    push(&mut res, p);
                }
            }
        }
        // top
        if point.y > 1 {
            for y in (0..point.y).rev() {
                let p = Vector2i::new(point.x, y);
                if self.bridge_points.contains(&p) {
                    break;
                } else if self.islands_gate_pos.contains(&p) {
                    continue;
                } else if self.islands_pos.contains(&p) {
                    res.push(p);
                    break;
                } else if y < point.y - 1 {
                    res.push(p);
                }
            }
        }
        // bottom
        if point.y < self.get_height() - 1 {
            for y in point.y + 1..self.get_height() {
                let p = Vector2i::new(point.x, y);
                if self.bridge_points.contains(&p) {
                    break;
                } else if self.islands_gate_pos.contains(&p) {
                    continue;
                } else if self.islands_pos.contains(&p) {
                    res.push(p);
                    break;
                } else if y > point.y + 1 {
                    res.push(p);
                }
            }
        }
        res
    }
    fn fill_conditions(&mut self, from: Option<Vector2i>, current: Vector2i) {
        if !self.islands_pos.contains(&current) {
            self.islands_pos.push(current);
        }
        if !self.able_to_gen_islands.contains(&current) {
            self.able_to_gen_islands.push(current);
        }
        for bridge_point in Self::calc_points(from, current, true).iter() {
            if !self.bridge_points.contains(bridge_point) {
                self.bridge_points.push(bridge_point.clone());
            }
        }
        for gate in self.calc_island_gate_pos(current).iter() {
            if !self.islands_gate_pos.contains(gate) {
                self.islands_gate_pos.push(gate.clone());
            }
        }
        if self.islands_pos.len() >= self.get_max_bridge_count() as usize {
            self.set_is_ready(true);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32tuple_eq() {
        assert!((0, 0) == (0, 0));
    }
    #[test]
    fn test_vec2i_eq() {
        assert!(Vector2i::new(0, 0) == Vector2i::new(0, 0));
        assert!(Vector2i::new(0, 0).eq(&Vector2i::new(0, 0)));
        assert!(vec![Vector2i::new(0, 0), Vector2i::new(0, 1)].contains(&Vector2i::new(0, 0)));
    }
    #[test]
    fn test_f64_to_i32() {
        assert_eq!(1, (2 as f64 * 0.99) as i32)
    }
    #[test]
    fn test_calc_points() {
        assert_eq!(
            GameMap::calc_points(Some(Vector2i::new(0, 0)), Vector2i::new(0, 2), false),
            vec![
                Vector2i::new(0, 0),
                Vector2i::new(0, 1),
                Vector2i::new(0, 2)
            ]
        );
        assert_eq!(
            GameMap::calc_points(Some(Vector2i::new(0, 0)), Vector2i::new(0, 2), true),
            vec![Vector2i::new(0, 1),]
        );
    }
    #[test]
    fn test_range() {
        let r = 0..10;
        let mut count = 0;
        let mut t = 10;
        for i in r.rev() {
            assert!(t == i + 1);
            t = i;
            count += 1;
        }
        assert_eq!(count, 10);
        let mut rng = rand::thread_rng();
        assert_eq!(rng.gen_range(0..1), 0);
    }
}
