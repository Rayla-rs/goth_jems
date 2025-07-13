extends Control

# Interum
var score = 0
var streak = 0
var hits = 0

# Totals
var total_score = 0
var total_streak = 0
var total_hits = 0

# Other
@export var duration = 0.25;

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	update_score_label()
	update_interum_label()
	pass # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta: float) -> void:
	update_score_label()
	update_interum_label()
	pass
	
func update_score(amount: int) -> void:
	total_score += amount
	var tween = get_tree().create_tween()
	tween.tween_property(self, "score", total_score, duration)

func update_score_label() -> void:
	$PanelContainer/HBoxContainer/ScoreLabel.text = "[wave amp=50.0 freq=5.0 connected=1]Score:" + str(score) + "[/wave]"

func update_streak(amount: int) -> void:
	total_streak += amount
	var tween = get_tree().create_tween()
	tween.tween_property(self, "streak", total_streak, duration)

func update_hits(amount: int) -> void:
	total_hits += amount
	var tween = get_tree().create_tween()
	tween.tween_property(self, "hits", total_hits, duration)

func update_interum_label() -> void:
	if hits == 0:
		$PanelContainer/HBoxContainer/InterumLabel.text = ""
	else:
		$PanelContainer/HBoxContainer/InterumLabel.text = "(" + str(hits) + ")" + " (x" + str(streak) + ")"

# Updates score using interum values
func resolve_interum() -> void:
	total_score += total_hits * total_streak
	total_hits = 0
	total_streak = 0
	update_streak(0)
	update_hits(0)
	update_score(0)
	
