use wasm_bindgen::JsValue;
use web_sys::WebGlRenderingContext;

pub const VERTEX_SHADER_SOURCE: &'static str = include_str!("sample.vert");
pub const FRAGMENT_SHADER_SOURCE: &'static str = include_str!("sample.frag");

enum ShaderType {
    VERTEX,
    FRAGMENT,
}

pub fn create_shader(
    gl: &WebGlRenderingContext,
    shader_type: &ShaderType,
) -> Result<web_sys::WebGlShader, JsValue> {
    let (type_, source) = match shader_type {
        ShaderType::VERTEX => (WebGlRenderingContext::VERTEX_SHADER, VERTEX_SHADER_SOURCE),
        ShaderType::FRAGMENT => (
            WebGlRenderingContext::FRAGMENT_SHADER,
            FRAGMENT_SHADER_SOURCE,
        ),
    };

    let shader = gl.create_shader(type_).unwrap();

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .is_truthy()
    {
        return Ok(shader);
    }

    let error = gl.get_shader_info_log(&shader).unwrap();

    gl.delete_shader(Some(&shader));
    Err(JsValue::from_str(&error))
}
