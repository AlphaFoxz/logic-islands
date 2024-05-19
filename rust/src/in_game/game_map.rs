use super::island::Island;
use godot::engine::Sprite2D;
use godot::prelude::*;
use rand::Rng;
use std::collections::HashSet;

/// 搭桥动作
#[derive(GodotConvert, Debug, Var, Export)]
#[godot(via = GString)]
pub enum BridgeAction {
    /// 拆桥
    Remove = 1,
    /// 无事发生
    Pass = 2,
    /// 单桥
    Single = 3,
    /// 双桥
    Double = 4,
}

/// 方向
#[derive(GodotConvert, Debug, Var, Export)]
#[godot(via = GString)]
pub enum Direction2D {
    /// 上
    Up = 1,
    /// 下
    Down = 2,
    /// 左
    Left = 3,
    /// 右
    Right = 4,
}

#[derive(Debug)]
struct RandInsertVec {
    pub value: Vec<Vector2i>,
    pub limit_rng: usize,
    rng: rand::rngs::ThreadRng,
}
impl RandInsertVec {
    fn new() -> Self {
        RandInsertVec {
            value: vec![],
            limit_rng: 0,
            rng: rand::thread_rng(),
        }
    }
    fn insert(&mut self, p: Vector2i) {
        if self.limit_rng < self.value.len() {
            self.value.push(p);
            return;
        }
        //随机插入
        let index = self.rng.gen_range(self.limit_rng..=self.value.len());
        self.value.insert(index, p);
        return;
    }
}

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
    #[init(default = vec![])]
    pub able_to_gen_islands: Vec<Vector2i>,
    #[init(default = vec![])]
    pub bridge_points: Vec<Vector2i>,
    #[init(default = HashSet::new())]
    pub user_bridge_points: HashSet<Vector2i>,
    #[init(default = 1)]
    #[export]
    pub game_mode: i32,
    #[init(default = rand::thread_rng())]
    rng: rand::rngs::ThreadRng,
    base: Base<Sprite2D>,
}

const BRIDGE_STATE: &'static str = "bridge_state";
const MAX_BRIDGE_COUNT: &'static str = "max_bridge_count";
const CURRENT_BRIDGE_COUNT: &'static str = "current_bridge_count";
const CHANGE_BRIDGE_COUNT: &'static str = "change_bridge_count";
const RENDER_BRIDGE: &'static str = "render_bridge";

/// 生成阶段
#[godot_api]
impl GameMap {
    #[signal]
    pub fn render_bridge(src_island: Gd<Island>, target_island: Gd<Island>, action: BridgeAction) {}
    #[func]
    fn user_gen_bridge(&mut self, src_pos: Vector2i, direction: Direction2D) -> BridgeAction {
        let mut target_pos = src_pos.clone();
        let mut src_island = self.islands.get(src_pos).unwrap().to::<Gd<Island>>();
        let mut src_island_bridge_state = src_island.get(BRIDGE_STATE.into()).to::<Vector4i>();
        let src_island_max_bridge_count = src_island.get(MAX_BRIDGE_COUNT.into()).to::<i32>();
        let mut src_island_current_bridge_count =
            src_island.get(CURRENT_BRIDGE_COUNT.into()).to::<i32>();
        let mut src_has_bridge;
        loop {
            match direction {
                Direction2D::Up => {
                    target_pos += Vector2i::UP;
                    src_has_bridge = src_island_bridge_state.x;
                }
                Direction2D::Down => {
                    target_pos += Vector2i::DOWN;
                    src_has_bridge = src_island_bridge_state.z;
                }
                Direction2D::Left => {
                    target_pos += Vector2i::LEFT;
                    src_has_bridge = src_island_bridge_state.w;
                }
                Direction2D::Right => {
                    target_pos += Vector2i::RIGHT;
                    src_has_bridge = src_island_bridge_state.y;
                }
            }
            if self.user_bridge_points.contains(&target_pos) && src_has_bridge == 0
                || !self.is_in_map(&target_pos)
            {
                // 已经被别的岛屿搭桥，或者不在地图范围内
                return BridgeAction::Pass;
            }
            if !self.islands.contains_key(target_pos) {
                // 没有岛屿
                continue;
            }
            let mut target_island = self.islands.get(target_pos).unwrap().to::<Gd<Island>>();
            let mut target_island_bridge_state =
                target_island.get(BRIDGE_STATE.into()).to::<Vector4i>();
            let target_island_max_bridge_count =
                target_island.get(MAX_BRIDGE_COUNT.into()).to::<i32>();
            let mut target_island_current_bridge_count =
                target_island.get(CURRENT_BRIDGE_COUNT.into()).to::<i32>();
            if src_island_max_bridge_count == src_island_current_bridge_count
                || target_island_max_bridge_count == target_island_current_bridge_count
                || src_has_bridge == 2
            {
                if src_has_bridge == 0 {
                    return BridgeAction::Pass;
                }
                match direction {
                    Direction2D::Up => {
                        src_island_bridge_state.x = 0;
                        target_island_bridge_state.z = 0;
                    }
                    Direction2D::Down => {
                        src_island_bridge_state.z = 0;
                        target_island_bridge_state.x = 0;
                    }
                    Direction2D::Left => {
                        src_island_bridge_state.w = 0;
                        target_island_bridge_state.y = 0;
                    }
                    Direction2D::Right => {
                        src_island_bridge_state.y = 0;
                        target_island_bridge_state.w = 0;
                    }
                }
                // 拆桥
                for p in Self::calc_points(Some(src_pos), target_pos, true).iter() {
                    self.user_bridge_points.remove(p);
                }
                // 更新状态
                src_island_current_bridge_count -= src_has_bridge;
                target_island_current_bridge_count -= src_has_bridge;
                src_island.set(
                    CURRENT_BRIDGE_COUNT.into(),
                    Variant::from(src_island_current_bridge_count),
                );
                src_island.set(BRIDGE_STATE.into(), Variant::from(src_island_bridge_state));
                target_island.set(
                    CURRENT_BRIDGE_COUNT.into(),
                    Variant::from(target_island_current_bridge_count),
                );
                target_island.set(
                    BRIDGE_STATE.into(),
                    Variant::from(target_island_bridge_state),
                );
                // 触发signal
                let s_arg = &[src_island.to_variant()];
                src_island.emit_signal(CHANGE_BRIDGE_COUNT.into(), s_arg);
                let t_arg = &[target_island.to_variant()];
                target_island.emit_signal(CHANGE_BRIDGE_COUNT.into(), t_arg);
                self.base_mut().emit_signal(
                    RENDER_BRIDGE.into(),
                    &[
                        Variant::from(src_island),
                        Variant::from(target_island),
                        Variant::from(BridgeAction::Remove),
                    ],
                );
                return BridgeAction::Remove;
            }
            match direction {
                Direction2D::Up => {
                    src_island_bridge_state.x += 1;
                    target_island_bridge_state.z += 1;
                }
                Direction2D::Down => {
                    src_island_bridge_state.z += 1;
                    target_island_bridge_state.x += 1;
                }
                Direction2D::Left => {
                    src_island_bridge_state.w += 1;
                    target_island_bridge_state.y += 1;
                }
                Direction2D::Right => {
                    src_island_bridge_state.y += 1;
                    target_island_bridge_state.w += 1;
                }
            }
            // 搭桥
            for p in Self::calc_points(Some(src_pos), target_pos, true).iter() {
                self.user_bridge_points.insert(*p);
            }
            // 更新状态
            src_island_current_bridge_count += 1;
            target_island_current_bridge_count += 1;
            src_island.set(
                CURRENT_BRIDGE_COUNT.into(),
                Variant::from(src_island_current_bridge_count),
            );
            src_island.set(BRIDGE_STATE.into(), Variant::from(src_island_bridge_state));
            target_island.set(
                CURRENT_BRIDGE_COUNT.into(),
                Variant::from(target_island_current_bridge_count),
            );
            target_island.set(
                BRIDGE_STATE.into(),
                Variant::from(target_island_bridge_state),
            );
            // 触发signal
            let s_arg = &[src_island.to_variant()];
            src_island.emit_signal(CHANGE_BRIDGE_COUNT.into(), s_arg);
            let t_arg = &[target_island.to_variant()];
            target_island.emit_signal(CHANGE_BRIDGE_COUNT.into(), t_arg);
            let action;
            let render_action;
            if src_has_bridge == 0 {
                action = BridgeAction::Single;
                render_action = BridgeAction::Single;
            } else {
                action = BridgeAction::Double;
                render_action = BridgeAction::Double;
            }
            self.base_mut().emit_signal(
                RENDER_BRIDGE.into(),
                &[
                    Variant::from(src_island),
                    Variant::from(target_island),
                    Variant::from(render_action),
                ],
            );
            return action;
        }
    }
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
            able_to_gen_islands: vec![],
            bridge_points: vec![],
            user_bridge_points: HashSet::new(),
            game_mode: 1,
            rng: rand::thread_rng(),
            base,
        })
    }
    /// 生成岛屿
    #[func]
    fn gen_island(&mut self) -> GString {
        if self.get_is_ready() {
            return "生成已完成".into();
        }
        if self.islands_pos.len() == 0 {
            // 初始化第一个节点
            let first_point = Vector2i::new(
                self.rng.gen_range(0..self.get_width()),
                self.rng.gen_range(0..self.get_height()),
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
            // godot_print!("{:?}无法生成，重新选择节点", src_position);
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
        let rindex = {
            let gen_per = self.islands_pos.len() as f32 / self.max_bridge_count as f32;
            if gen_per < 0.95 {
                let b = self.rng.gen_bool(0.05 * self.game_mode as f64);
                self.weighted_random_index(valid_next_points.len(), b)
            } else {
                self.rng.gen_range(0..valid_next_points.len())
            }
        };
        let next_point = valid_next_points[rindex];
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
        self.set_is_ready(false);
        self.islands.clear();
        self.able_to_gen_islands.clear();
        self.islands_gate_pos.clear();
        self.bridge_points.clear();
        self.islands_pos.clear();
        self.user_bridge_points.clear();
        // self.game_mode = game_mode;
        self.set_max_bridge_count(Self::calc_max_bridge_count(
            self.get_game_mode(),
            self.get_width(),
            self.get_height(),
        ));
        true
    }
    fn weighted_random_index(&mut self, n: usize, more_weight: bool) -> usize {
        let weights: Vec<f32>;
        if more_weight {
            // 指数权重
            weights = (0..n).map(|i| (-(i as f32)).exp()).collect();
        } else {
            // 线性权重
            weights = (0..n).map(|i| 1.0 / (i as f32 * 0.8 + 1.0)).collect();
        }

        let total_weight = weights.iter().sum::<f32>();
        let rand_v = self.rng.gen_range(0.0..total_weight);
        let mut cumulative_weight = 0.0;
        for (i, &weight) in weights.iter().enumerate() {
            cumulative_weight += weight;
            if rand_v < cumulative_weight {
                return i;
            }
        }
        n
    }
    fn select_random_island(&mut self) -> Option<(Vector2i, usize)> {
        if self.able_to_gen_islands.is_empty() {
            return None;
        }
        let index = self.rng.gen_range(0..self.able_to_gen_islands.len());
        let island = self.able_to_gen_islands.get(index).unwrap();
        Some((island.clone(), index))
    }
    fn link_island(
        &mut self,
        from_pos: Option<Vector2i>,
        current_pos: Vector2i,
        mut island: Gd<Island>,
    ) {
        self.fill_conditions(from_pos, current_pos);
        if from_pos.is_some() {
            let bridge_count = {
                if self.rng.gen_bool(0.55 - 0.02 * self.game_mode as f64) {
                    2
                } else {
                    1
                }
            };
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
        let n = {
            if game_mode > 2 {
                0.25
            } else {
                0.18
            }
        };
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
        } else {
            godot_error!("不可能的情况");
            return vec![];
        }
        res
    }
    /// 从一个岛屿出发，可以生成的另一个岛屿
    fn calc_valid_next_point(&mut self, point: Vector2i) -> Vec<Vector2i> {
        let mut result = RandInsertVec::new();
        let mut has_up = true;
        let mut has_down = true;
        let mut has_left = true;
        let mut has_right = true;
        let mut offset = 1;
        loop {
            if !has_up && !has_down && !has_left && !has_right {
                return result.value;
            }
            if has_up {
                let p = Vector2i::new(point.x, point.y - offset);
                if !self.is_in_map(&p) || self.bridge_points.contains(&p) {
                    has_up = false;
                } else if self.islands_gate_pos.contains(&p) || offset == 1 {
                    // continue;
                } else if self.islands_pos.contains(&p) {
                    result.insert(p);
                    has_up = false
                } else {
                    result.insert(p);
                }
            }
            if has_down {
                let p = Vector2i::new(point.x, point.y + offset);
                if !self.is_in_map(&p) || self.bridge_points.contains(&p) {
                    has_down = false;
                } else if self.islands_gate_pos.contains(&p) || offset == 1 {
                    // continue;
                } else if self.islands_pos.contains(&p) {
                    result.insert(p);
                    has_down = false
                } else {
                    result.insert(p);
                }
            }
            if has_left {
                let p = Vector2i::new(point.x - offset, point.y);
                if !self.is_in_map(&p) || self.bridge_points.contains(&p) {
                    has_left = false;
                } else if self.islands_gate_pos.contains(&p) || offset == 1 {
                    // continue;
                } else if self.islands_pos.contains(&p) {
                    result.insert(p);
                    has_left = false;
                } else {
                    result.insert(p);
                }
            }
            if has_right {
                let p = Vector2i::new(point.x + offset, point.y);
                if !self.is_in_map(&p) || self.bridge_points.contains(&p) {
                    has_right = false;
                } else if self.islands_gate_pos.contains(&p) || offset == 1 {
                    // continue;
                } else if self.islands_pos.contains(&p) {
                    result.insert(p);
                    has_right = false;
                } else {
                    result.insert(p);
                }
            }
            result.limit_rng = result.value.len();
            offset += 1;
        }
    }
    fn is_in_map(&self, point: &Vector2i) -> bool {
        if point.x < 0 || point.y < 0 {
            return false;
        }
        if point.x >= self.get_width() || point.y >= self.get_height() {
            return false;
        }
        true
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

    #[test]
    fn test_list() {
        let mut v = vec![0];
        v.insert(1, 1);
        assert_eq!(v, vec![0, 1])
        // v.insert(3, 3);
    }
}
