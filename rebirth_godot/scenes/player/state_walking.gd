extends StateMachineState

@onready var player: CharacterBody3D = $"../.."
@onready var camera: Camera3D = $"../../Camera3D"
@onready var interactor: Interactor = $"../../Camera3D/Interactor"

@export var SPEED := 5.0
@export var JUMP_VELOCITY := 4.5
@export var look_angle_limit := 75.0

var mouse_delta := Vector2.ZERO

func _state_enter() -> void:
	Input.mouse_mode = Input.MOUSE_MODE_CAPTURED

func _state_exit() -> void:
	pass

func _state_physics(delta: float) -> void:
	_move_cam(delta)
	
	if not player.is_on_floor():
		player.velocity += player.get_gravity() * delta

	if Input.is_action_just_pressed("jump") and player.is_on_floor():
		player.velocity.y = JUMP_VELOCITY

	# Get the input direction and handle the movement/deceleration.
	# As good practice, you should replace UI actions with custom gameplay actions.
	var input_dir := Input.get_vector("move_left", "move_right", "move_forwards", "move_back")
	var direction := ( \
		( camera.global_basis * \
		Vector3(input_dir.x, 0, input_dir.y) ) * \
		Vector3(1, 0, 1) \
		).normalized()
	if direction:
		player.velocity.x = direction.x * SPEED
		player.velocity.z = direction.z * SPEED
	else:
		player.velocity.x = move_toward(player.velocity.x, 0, SPEED)
		player.velocity.z = move_toward(player.velocity.z, 0, SPEED)

	player.move_and_slide()


func _state_input(event: InputEvent) -> bool:
	if event is InputEventMouseMotion:
		var mouse = event as InputEventMouseMotion
		mouse_delta += mouse.relative
		return true
	if event.is_action_pressed("interact"):
		interactor.do_interact()
	return false


func _move_cam(delta: float):
	var look_vec := Input.get_vector("look_left", "look_right", "look_down", "look_up")
	if look_vec.length_squared() < 0.3:
		look_vec = mouse_delta # mul mouse sensitivity
		mouse_delta = Vector2.ZERO
	else:
		look_vec *= 10.0 # gamepad sensitivity
	look_vec *= -1
	player.rotate_y(look_vec.x * delta)
	camera.rotate_x(look_vec.y * delta)
	camera.rotation.x = clamp(camera.rotation.x, -deg_to_rad(look_angle_limit), deg_to_rad(look_angle_limit))
