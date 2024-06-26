extends GameMap

var pos_scale: float = Global.config.map_item_scale
var base_offset = Vector2(pos_scale/2, pos_scale/2)
const PREVIEW_NAME: String = 'preview'

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	self.connect('render_bridge', self.on_render_bridge)
	self.width = Global.config.map_size.x
	self.height = Global.config.map_size.y
	self.game_mode = Global.config.game_mode
	self.reset()
	while !self.is_ready:
		var exit_str: String = self.gen_island()
		if exit_str.length() > 0:
			push_error('生成结束，信息：' + exit_str)
			break
		#print('able_to_gen_islands', self.able_to_gen_islands)
		#print('bridge_points', self.bridge_points)
	print('island计数' + String.num(self.islands.size()))
	for child: Island in self.islands.values():
		child.connect('finish_preview_bridge', self.on_finish_preview_bridge)
		child.connect('preview_bridge', self.on_preview_bridge)
		child.connect('change_bridge_count', self.on_change_bridge_count)
		child.text = String.num(child.max_bridge_count)
		#child.text = String.num(child.max_bridge_count) + "(" + String.num(child.pos.x) + "," + String.num(child.pos.y) + ")"
		var size = Vector2(pos_scale, pos_scale)
		child.set_size(size)
		child.pivot_offset = base_offset
		child.set_position(child.pos * pos_scale)
		add_child(child)

func _draw() -> void:
	var sc = base_offset * 2
	var map_size: Vector2i = Global.config.map_size
	var color: Color = Color(1, 1, 1, 0.1)
	for x in range(0, map_size.x):
		draw_line(Vector2(x, 0) * sc + base_offset, Vector2(x, map_size.y - 1) * sc + base_offset, color, 5)
	for y in range(0, map_size.y):
		draw_line(Vector2(0, y) * sc + base_offset, Vector2(map_size.x - 1, y) * sc + base_offset, color, 5)

func on_preview_bridge(island: Island, rel_pos: Vector2):
	var abs_pos = rel_pos + island.position
	var line = self.find_child(PREVIEW_NAME, false, false)
	if line == null:
		line = Line2D.new()
		line.name = PREVIEW_NAME
		line.width = 3.0
		line.add_point(island.position + base_offset)
		line.add_point(abs_pos)
		self.add_child(line)
	line = line as Line2D
	if island.get_rect().has_point(abs_pos):
		self.remove_child(line)
		line.queue_free()
		return
	var offset_x: float = 0.0
	var offset_y: float = 0.0
	rel_pos -= base_offset
	if abs(rel_pos.x) > pos_scale:
		offset_x = rel_pos.x
	if abs(rel_pos.y) > pos_scale:
		offset_y = rel_pos.y
	if abs(offset_x) >= abs(offset_y):
		offset_y = 0.0
		var n = offset_x / abs(offset_x)
		line.set_point_position(0, island.position + base_offset + Vector2(n * pos_scale / 2, 0))
	else:
		offset_x = 0.0
		var n = offset_y / abs(offset_y)
		line.set_point_position(0, island.position + base_offset + Vector2(0, n * pos_scale / 2))
	line.set_point_position(1, island.position + base_offset + Vector2(offset_x, offset_y))

func on_finish_preview_bridge(island: Island, rel_pos: Vector2):
	var p = self.find_child(PREVIEW_NAME, false, false)
	if p == null:
		print('取消搭桥')
		return
	print('开始搭桥')
	self.remove_child(p)
	p.queue_free()
	print('gd端创建桥梁', island, rel_pos)
	var offset_x: float = 0.0
	var offset_y: float = 0.0
	rel_pos -= base_offset
	if abs(rel_pos.x) > pos_scale:
		offset_x = rel_pos.x
	if abs(rel_pos.y) > pos_scale:
		offset_y = rel_pos.y
	if abs(offset_x) >= abs(offset_y):
		var n = offset_x / abs(offset_x)
		if n > 0:
			self.user_gen_bridge(island.pos, 'Right')
		else:
			self.user_gen_bridge(island.pos, 'Left')
	else:
		var n = offset_y / abs(offset_y)
		if n > 0:
			self.user_gen_bridge(island.pos, 'Down')
		else:
			self.user_gen_bridge(island.pos, 'Up')

func on_change_bridge_count(island: Island):
	if island.max_bridge_count == island.current_bridge_count:
		var color = Color(0, 0.7, 0)
		island.add_theme_color_override('font_focus_color', color)
		island.add_theme_color_override('font_hover_color', color)
		island.add_theme_color_override('font_color', color)
		#island.text = 'y'
	else:
		island.remove_theme_color_override('font_focus_color')
		island.remove_theme_color_override('font_hover_color')
		island.remove_theme_color_override('font_color')
		#island.text = String.num(island.max_bridge_count)

func on_render_bridge(src_island: Island, target_island: Island, action: String):
	var i_arr: Array[Island] = Bridge.order_island(src_island, target_island)
	src_island = i_arr[0]
	target_island = i_arr[1]
	var p1 = src_island.pos
	var p2 = target_island.pos
	var bridge_name: String = Bridge.calc_name(p1, p2)
	var bridge_node: Bridge = self.find_child(bridge_name, false, false)
	if action == 'Remove':
		if bridge_node != null:
			self.remove_child(bridge_node)
			bridge_node.queue_free()
		return
	if bridge_node == null:
		# action == 'Single'
		bridge_node = Bridge.create(p1, p2, pos_scale, 1)
		bridge_node.position = src_island.position
		self.add_child(bridge_node)
	if action == 'Double':
		print('双桥')
		(bridge_node as Bridge).change_bridge_count(2)
