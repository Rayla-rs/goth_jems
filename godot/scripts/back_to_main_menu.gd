extends Control

# Go back to main menu
func back() -> void:
	get_tree().change_scene_to_file("res://scenes/main_menu.tscn")
