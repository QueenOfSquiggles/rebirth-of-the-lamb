extends Node

@export var mouse_motion_event: EventResource

func _unhandled_input(event: InputEvent) -> void:
	if Input.mouse_mode != Input.MOUSE_MODE_CAPTURED:
		return
	if event is InputEventMouseMotion:
		var mouse := event as InputEventMouseMotion
		var relay = EventArgs.new()
		relay.event = mouse_motion_event
		relay.args = [mouse.relative]
		EventDepot.trigger(relay)
		get_viewport().set_input_as_handled()
