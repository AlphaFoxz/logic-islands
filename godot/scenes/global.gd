extends Node

class_name Global

static var config: Cfg = Cfg.new()
class Cfg:
	var map_size: Vector2i = Vector2i(10, 7)
	var game_mode: int = 1
	var zoom_speed: float = -0.1
	var map_item_scale: float = 30.0
