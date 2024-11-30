extends RigidBody3D


func _on_interaction_component_on_deselect() -> void:
	print("deselect")


func _on_interaction_component_on_interact() -> void:
	print("interact")


func _on_interaction_component_on_select() -> void:
	print("select")
