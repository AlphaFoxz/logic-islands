extends Node2D

@onready var _height_edit = $GridContainer/HeightEdit as TextEdit
@onready var _width_edit = $GridContainer/WidthEdit as TextEdit
@onready var _game_mode = $GridContainer/GameModeEdit as TextEdit
var blank_reg = RegEx.new()

func _ready() -> void:
	_width_edit.text = String.num(Global.config.map_size.x)
	_height_edit.text = String.num(Global.config.map_size.y)
	_game_mode.text = String.num(Global.config.game_mode)
	blank_reg.compile('\\s')
	print('游戏主菜单')

func _on_start_button_pressed() -> void:
	get_tree().change_scene_to_file('res://scenes/in_game/in_game_scenes.tscn')

func _on_settings_button_pressed() -> void:
	get_tree().change_scene_to_file('res://scenes/settings/settings.tscn')

func _on_exit_button_pressed() -> void:
	get_tree().quit(0)

func _on_width_edit_focus_exited() -> void:
	var t: String = _width_edit.text
	var w: int
	if t == null or t.is_empty() or !t.is_valid_int():
		w = 10
	else:
		w = t.to_int()
		if w < 5:
			w = 5
	Global.config.map_size.x = w
	_width_edit.text = String.num(w)

func _on_width_edit_text_changed() -> void:
	var t: String = _width_edit.text
	var n = t
	if blank_reg.search(t) != null:
		n = blank_reg.sub(t, '', true)
	if !n.is_valid_int():
		n = '10'
	if n != t:
		_width_edit.text = n

func _on_height_edit_focus_exited() -> void:
	var t: String = _height_edit.text
	var h: int
	if t == null or t.is_empty() or !t.is_valid_int():
		h = 7
	else:
		h = t.to_int()
		if h < 5:
			h = 5
	Global.config.map_size.y = h
	_height_edit.text = String.num(h)

func _on_height_edit_text_changed() -> void:
	var t: String = _height_edit.text
	var n = t
	if blank_reg.search(t) != null:
		n = blank_reg.sub(t, '', true)
	if !n.is_valid_int():
		n = '7'
	if n != t:
		_height_edit.text = n

func _on_game_mode_edit_focus_exited() -> void:
	var t: String = _game_mode.text
	var v: int
	if t == null or t.is_empty() or !t.is_valid_int():
		v = 1
	else:
		v = t.to_int()
		if v < 1:
			v = 5
		elif v > 20:
			v = 20
	Global.config.game_mode = v
	_game_mode.text = String.num(v)

func _on_game_mode_edit_text_changed() -> void:
	var t: String = _game_mode.text
	var n = t
	if blank_reg.search(t) != null:
		n = blank_reg.sub(t, '', true)
	if !n.is_valid_int():
		n = '1'
	if n != t:
		_game_mode.text = n
