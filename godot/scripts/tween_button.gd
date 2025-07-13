extends Button

@export var hover_scale_mult = 1.0
@export var duration = 0.25

func _ready() -> void:
	mouse_entered.connect(enter)
	mouse_exited.connect(exit)
	
	# Setup pivot
	pivot_offset = size / 2
	pass

# When mouse enters
func enter() -> void:
	var tween = create_tween().set_trans(Tween.TRANS_SINE)
	tween.tween_property(self, "scale", Vector2.ONE * hover_scale_mult, duration)
	pass

# When mouse exits
func exit() -> void:
	var tween = create_tween().set_trans(Tween.TRANS_SINE)
	tween.tween_property(self, "scale", Vector2.ONE, duration)
	pass
