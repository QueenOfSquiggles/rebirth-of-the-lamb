extends Player

@export var emit_hit : EventArgs


func _ready() -> void:
	EventDepot.connect(emit_hit.event, on_receive)

func _process(_delta: float) -> void:
	if Input.is_action_just_pressed("ui_accept"):
		EventDepot.trigger(emit_hit)

func on_receive():
	print("heyo")
