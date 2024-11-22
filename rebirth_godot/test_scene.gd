extends Node2D

@export var push_event: EventArgs

func _ready() -> void:
	EventDepot.add_listener(push_event.event, call_empty)
	EventDepot.add_listener(push_event.event, call_args)
	EventDepot.add_listener(push_event.event, call_wrong)
	EventDepot.add_listener(push_event.event, call_too_many)
	EventDepot.add_listener(push_event.event, call_bound.bind("Extra from the bindings"))


func _process(_delta: float) -> void:
	if Input.is_action_just_pressed("ui_accept"):
		EventDepot.trigger(push_event)

func call_empty() -> void:
	print("Called empty") # does not get called

func call_args(flag: bool, num: int, vec: Vector2) -> void:
	print("Called with args: [%s, %s, %s]" % [str(flag), str(num), str(vec)])

func call_wrong(_num: int, _flag: bool, _mat: Vector4i) -> void:
	print("This shouldn't get called") # does not get called

func call_too_many(_a : bool, _b : int, _c : Vector2, _d : int, _e : String):
	print("This should break godot") # does not get called

func call_bound(flag: bool, num: int, vec: Vector2, extra: String) -> void:
	print("Called with args: [%s, %s, %s, %s]" % [str(flag), str(num), str(vec), str(extra)])
	
