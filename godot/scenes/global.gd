extends Node

class_name Global

static var config: CfgFile = CfgFile.new()
const CFG_PATH: String = 'res://config.ini'
class CfgFile:
	var _file = ConfigFile.new()
	var _is_ready = false
	var map_size: Vector2i = Vector2i(100, 100):
		get:
			self.load()
			return map_size
		set(v):
			map_size = v
			_file.set('map_size', v)
			self.save()
	var game_mode: int = 2:
		get:
			self.load()
			return game_mode
		set(v):
			game_mode = v
			_file.set('game_mode', v)
			self.save()
	var zoom_speed: float = -0.1:
		get:
			self.load()
			return zoom_speed
		set(v):
			zoom_speed = v
			_file.set('zoom_speed', v)
			self.save()
	func load():
		if _is_ready:
			return
		var err = self._file.load(CFG_PATH)
		if err != OK:
			printerr('加载配置文件异常', err)
			return
		self._is_ready = true
	func save():
		self._file.save(CFG_PATH)
