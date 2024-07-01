pub const BASE_VERTEX_SHADER: &str = r#"
#version 140

in vec3 position;
in vec4 color;
in vec2 uv;
in float style;

uniform mat4 matrix;

out vec4 v_color;
out vec2 v_uv;
out float v_style;

void main() {
    v_color = color;
    v_uv = uv;
    v_style = style;
    gl_Position = matrix * vec4(position.xy, 1 / position.z, 1.0);
}
"#;
pub const BASE_FRAG_SHADER: &str = r#"
#version 140

in vec4 v_color;
in vec2 v_uv;
in float v_style;

out vec4 color;

void main() {
    color = v_color;

    if (v_style == 1.0) {
        if (distance(v_uv, vec2(0.5)) > 0.5) {
            color = vec4(0.0,0.0,0.0,0.0);
        }
    }
}
"#;

pub const TEX_VERTEX_SHADER: &str = r#"
#version 140

in vec3 position;
in vec4 color;
in vec2 uv;
in float style;
out vec2 v_tex_coord;
out vec4 v_color;
out float v_style;

uniform mat4 matrix;

void main() {
    v_color = color;
    v_tex_coord = uv;
    v_style = style;
    gl_Position = matrix * vec4(position.xy, 1 / position.z, 1.0);
}
"#;
pub const TEX_FRAG_SHADER: &str = r#"
#version 140

in vec2 v_tex_coord;
in vec4 v_color;
in float v_style;
out vec4 color;

uniform sampler2D tex;

// Converts a color from linear light gamma to sRGB gamma
vec4 fromLinear(vec4 linearRGB)
{
    bvec3 cutoff = lessThan(linearRGB.rgb, vec3(0.0031308));
    vec3 higher = vec3(1.055)*pow(linearRGB.rgb, vec3(1.0/2.4)) - vec3(0.055);
    vec3 lower = linearRGB.rgb * vec3(12.92);

    return vec4(mix(higher, lower, cutoff), linearRGB.a);
}

void main() {
    if (v_style == 0) {
        color = v_color;
    }
    else if (v_style == 1) {
        if (distance(v_tex_coord, vec2(0.5)) > 0.5) {
            color = vec4(0.0,0.0,0.0,0.0);
        }
        else {
            color = v_color;
        }
    }
    else {
        color = fromLinear(texture(tex, v_tex_coord) * v_color);
    }
}
"#;