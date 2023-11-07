#import bevy_render::view::View
#import bevy_ui::ui_vertex_output::UiVertexOutput

struct CircleMaterial {
    @location(0) color: vec4<f32>
}

@group(0) @binding(0)
var<uniform> view: View;

@group(1) @binding(0)
var<uniform> input: CircleMaterial;

// @fragment
// fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
//     let uv = in.uv * 2.0 - 1.0;
//     let alpha = 1.0 - pow(sqrt(dot(uv, uv)), 100.0);
//     return vec4<f32>(input.color.rgb, alpha);
// }

@vertex
fn vertex(
    @location(0) vertex_position: vec3<f32>,
    @location(1) vertex_uv: vec2<f32>,
    @location(2) border_widths: vec4<f32>,
) -> UiVertexOutput {
    var out: UiVertexOutput;
    out.uv = vertex_uv;
    // let stick_pos = border_widths.xy;
    // let base_offset = border_widths.zw;
    let base_offset = vec2<f32>(0., 10.);
    out.position = view.view_proj * vec4<f32>(vertex_position + vec3<f32>(base_offset, 0.0), 1.0);
    out.border_widths = border_widths;
    return out;
}

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.5);
}
