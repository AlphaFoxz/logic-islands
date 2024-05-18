extends Node2D

func _ready() -> void:
	print('游戏主菜单')

func _on_start_button_pressed() -> void:
	get_tree().change_scene_to_file('res://scenes/in_game/in_game_scenes.tscn')

func _on_settings_button_pressed() -> void:
	get_tree().change_scene_to_file('res://scenes/settings/settings.tscn')

func _on_exit_button_pressed() -> void:
	get_tree().quit(0)
