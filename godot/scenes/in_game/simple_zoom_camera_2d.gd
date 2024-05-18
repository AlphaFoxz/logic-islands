extends SimpleZoomCamera2D

func _ready() -> void:
	self.zoom_speed = Global.config.zoom_speed
