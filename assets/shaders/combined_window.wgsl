struct VertexOutput {
    @builtin(position) clip_position:vec4<f32>,
    @location(0) uv:vec2<f32>
};

struct Globals {
    time: f32,
    delta_time: f32,
    frame_count: u32,
}

struct ShaderMaterial {
    color: vec4<f32>,
    scale: f32,
    shader_type: u32,
    light_shaders: u32,
    dust_particles: u32,
    dust_amount: f32,
};


@group(0) @binding(1)
var<uniform> globals: Globals;

@group(2) @binding(0)
var<uniform> material: ShaderMaterial;



@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    if material.light_shaders == 0 { return vec4<f32>(0.0);}

    let uv = in.uv;

    var beam = 0.0;

    let slant1 = (1 - (uv.x + uv.y * 0.6)) * material.scale;
    let slant2 = (1 - (uv.x + uv.y * 0.9)) * material.scale;
    let slant3 = (1 - (uv.x + uv.y * 0.1)) * material.scale;


    if (material.shader_type == 0) {
        let beam1 = smoothstep(1.0, 0.2,  slant1)  * smoothstep(-0.6, 0.2, slant1);
        let beam2 = smoothstep(0.4, -0.2,  slant2)  * smoothstep(-0.8, -0.2, slant2);
        let beam3 = smoothstep(1.0, 0.3,  slant3)  * smoothstep(-0.1, 0.3, slant3);

        beam = (beam3 + beam2) * 0.05 + beam1 * 0.3 ;

    } else if (material.shader_type == 1) {

        let beam1 = smoothstep(1.0, 0.8,  slant1)  * smoothstep(-0.8, -0.2, slant1);
        let beam2 = smoothstep(0.4, -0.2,  slant2)  * smoothstep(-0.8, -0.2, slant2);
        let beam3 = smoothstep(1.0, 0.3,  slant3)  * smoothstep(-0.1, 0.3, slant3);

        let win_beam1 = (beam3 + beam2) * 0.5 + beam1 * 0.2 ;
        
        let slant4 = (uv.x - uv.y * 0.6) * material.scale;
        let slant5 = (uv.x - uv.y * 0.9) * material.scale;
        let slant6 = (uv.x - uv.y * 0.1) * material.scale;

        let beam4 = smoothstep(0.0, -0.2,  slant4)  * smoothstep(-1.0, -0.2, slant4);
        let beam5 = smoothstep(0.6, -0.2,  slant5)  * smoothstep(-1.0, -0.2, slant5);
        let beam6 = smoothstep(1.2, 0.3,  slant6)  * smoothstep(-0.3, 0.3, slant6);

        let win_beam2 = (beam5 + beam6) * 0.5 + beam4 * 0.075 ;

        beam = (win_beam1 + win_beam2 * 1.8) * 0.2;

    }

    var final_beam =beam;

    if material.dust_particles == 1 {
        final_beam = generation_dust(uv, beam, material.dust_amount);
    };
    
    return vec4<f32>(material.color.rgb, material.color.a * final_beam );
}

fn generation_dust(
    uv: vec2<f32>,
    beam: f32,
    dust_amount: f32,
) -> f32 {
    let grid_uv = floor(uv * 100.0);

    let cell_hash = hash(grid_uv);

    let threshold  = 1.0 - dust_amount * 0.05;

    let is_dust = step(threshold, cell_hash);

    let seed = floor(globals.time * 0.01 + cell_hash * 1.0);

    let n = hash(floor(grid_uv + seed));

    var dust = pow(n, 30.0);

    dust *= sin(globals.time * 3.0 + cell_hash * 50.0) * 0.5 + 0.5;
            
    let dust_beam = dust * beam * 12.0;

    return beam + (dust_beam * is_dust);
}

fn hash(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3<f32>(p.xyx) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}
    