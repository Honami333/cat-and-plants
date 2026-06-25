struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>
};

struct Globals {
    time: f32,
    delta_time: f32,
    frame_count: u32
};

struct BreezeShaderMaterial{
    wind_speed: f32,
    wind_strength: f32,
    sprite_rect: vec4<f32>,
    soil_line: f32,

    breeze_shaders: u32,
};


@group(0) @binding(1)
var<uniform> globals: Globals;

@group(2) @binding(0)
var<uniform> material: BreezeShaderMaterial;

@group(2) @binding(1)
var texture_color: texture_2d<f32>;
@group(2) @binding(2)
var texture_sampler: sampler;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let textur_uv = mix(material.sprite_rect.xy, material.sprite_rect.zw, in.uv);

    let uv = in.uv;

    let original_color = textureSample(texture_color, texture_sampler, textur_uv);

    if (material .breeze_shaders == 0) { return original_color; };

    if (original_color.a == 0.0) { return original_color; };

    let is_green = (original_color.g > original_color.r * 1.1) && (original_color.g > original_color.b * 1.1); 

    if (is_green) {
        let un_uv = 1.0 - textur_uv.y;

        let sway_factor = (un_uv - material.soil_line) / (1.0 - material.soil_line );
        let safe_sway  = clamp(sway_factor, 0.0, 1.0);

        let wave_x = sin(globals.time * material.wind_speed + textur_uv.y * 3.0) * material.wind_strength * safe_sway * 0.7;
        let wave_y = cos(globals.time * material.wind_speed * 0.5 + textur_uv.x * 2.0) * material.wind_strength * safe_sway * 0.3;

        let offset_uv  = textur_uv + vec2<f32>(wave_x, wave_y);

        var swayed_color  = textureSample(texture_color, texture_sampler, offset_uv);

        let is_green = (swayed_color.g > swayed_color.r * 1.1) && (swayed_color.g > swayed_color.b * 1.1); 

        let is_return = (!is_green) || (swayed_color.a == 0);

        if (is_return) { return original_color; };

        return swayed_color;
    };

    return original_color;
}