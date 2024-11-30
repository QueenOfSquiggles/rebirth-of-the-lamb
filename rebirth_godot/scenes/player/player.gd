extends CharacterBody3D

@export var mouse_motion_relay_event: EventResource
var mouse_delta := Vector2.ZERO

func _ready() -> void:
	EventDepot.add_listener(mouse_motion_relay_event, mouse_motion_event)

func mouse_motion_event(relative: Vector2):
	mouse_delta += relative
