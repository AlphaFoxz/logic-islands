extends Node2D

func _ready() -> void:
	#var offset = Global.config.map_size * Global.config.map_item_scale / 2
	var offset = Vector2(50, 50)
	$SimpleZoomCamera2D.position -= offset

func _on_back_button_pressed() -> void:
	get_tree().change_scene_to_file('res://scenes/main/main.tscn')

func _on_renew_button_pressed() -> void:
	self.reset()

func reset():
	get_tree().change_scene_to_file('res://scenes/in_game/in_game_scenes.tscn')
