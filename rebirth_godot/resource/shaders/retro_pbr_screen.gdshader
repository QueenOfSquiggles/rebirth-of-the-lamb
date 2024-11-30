shader_type canvas_item;

uniform sampler2D noise_texture : hint_default_white, filter_nearest, repeat_enable;
uniform float noise_scale = 1.0;
uniform float noise_factor :hint_range(0.0, 1.0) = 1.0;
uniform float levels = 10.0;
uniform float gamma = 1.0;


float brightness(vec3 colour) {
	return colour.r * 0.2126 + colour.g * 0.7152 + colour.b * 0.0722;
}

float posterize(float brightness) {
	float factor = round(brightness * levels) / levels;
	return factor;
}


void fragment() {
	vec2 screen_size = vec2(textureSize(TEXTURE, 0));
	vec2 dither_size = vec2(textureSize(noise_texture, 0));
	float dither = texture(noise_texture, UV * (screen_size / dither_size) * noise_scale).r;
	
	vec3 screen = texture(TEXTURE, UV).rgb;
	screen *= posterize(brightness(screen) + dither * noise_factor);
	screen = pow(screen, vec3(gamma));
	COLOR = vec4(screen, 1.0);
}