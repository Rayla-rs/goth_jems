extends Control

# Exit aplication
func exit() -> void:
	get_tree().quit()

# Play game
func play() -> void:
	get_tree().change_scene_to_file("res://scenes/main.tscn")

func credits() -> void:
	get_tree().change_scene_to_file("res://scenes/credits.tscn")
