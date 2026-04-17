struct VertexOutput {
    @builtin(position) clip_position:vec4<f32>,
    @location(0) uv:vec2<f32>
}


struct ShaderMaterial {
    color: vec4<f32>,
    scale: f32
};


@group(2) @binding(0)
var<uniform> material: ShaderMaterial;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    let slant1 = (1 - (uv.x + uv.y * 0.6)) * material.scale;
    let slant2 = (1 - (uv.x + uv.y * 0.9)) * material.scale;
    let slant3 = (1 - (uv.x + uv.y * 0.1)) * material.scale;
    let beam1 = smoothstep(1.0, 0.2,  slant1)  * smoothstep(-0.6, 0.2, slant1);
    let beam2 = smoothstep(0.4, -0.2,  slant2)  * smoothstep(-0.8, -0.2, slant2);
    let beam3 = smoothstep(1.0, 0.3,  slant3)  * smoothstep(-0.1, 0.3, slant3);

    let beam = (beam3 + beam2) * 0.05 + beam1 * 0.3 ;

    return vec4<f32>(material.color.rgb, material.color.a * beam);
}


