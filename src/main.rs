#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb_derive;

mod webgl_rendering_context;

use stdweb::web::html_element::CanvasElement;

use stdweb::unstable::TryInto;
use stdweb::web::{
    IEventTarget,
    IHtmlElement,
    IParentNode,
    document,
    window,
    TypedArray,
};

use webgl_rendering_context::{
    WebGLRenderingContext as gl,
    WebGLUniformLocation,
    WebGLBuffer,
    WebGLShader,
    WebGLProgram
};

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

fn main() {
    stdweb::initialize();

    let canvas: CanvasElement = document().query_selector("#canvas").unwrap();
    let context: gl = canvas.get_context().unwrap();

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);

    window().add_event_listener( enclose!( (canvas) move |_: ResizeEvent| {
        canvas.set_width(canvas.offset_width() as u32);
        canvas.set_height(canvas.offset_height() as u32);
    }));

    let shader_program = init_shader_program(&mut context);
}

fn init_shader_program(ctx: &mut gl) -> WebGLProgram {
    context.clear_color(1.0, 0.0, 0.0, 1.0);
    context.clear(gl::COLOR_BUFFER_BIT);

    let vertices = TypedArray::<f32>::from(&[
        0.0,  1.0,  0.0,
        -1.0, -1.0,  0.0,
        1.0, -1.0,  0.0
    ][..]).buffer();

    let triangle_vert = gl.createBuffer().unwrap();
    context.bind_buffer(gl::ARRAY_BUFFER, Some(&triangle_vert));
    context.buffer_data_1(gl::ARRAY_BUFFER, Some(&vertices), gl::STATIC_DRAW);

    get_program(ctx, get_vertex_shader(ctx), get_fragment_shader(ctx))
}

fn get_vertex_shader(ctx: &mut gl) -> WebGLShader {
    let vert_code = r#"
        attribute vec3 aVertexPosition;
        uniform mat4 uMVMatrix;
        uniform mat4 uPMatrix;
        void main(void) {
            gl_Position = uPMatrix * uMVMatrix * vec4(aVertexPosition, 1.0);
        }
    "#;

    let vert_shader = context.create_shader(gl::VERTEX_SHADER).unwrap();
    ctx.shader_source(&vert_shader, vert_code);
    ctx.compile_shader(&vert_shader);
    vert_shader
}

fn get_fragment_shader(ctx: &mut gl) -> WebGLShader {
    let frag_code = r#"
        precision mediump float;
        void main(void) {
            gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#;

    let frag_shader = ctx.create_shader(gl::FRAGMENT_SHADER).unwrap();
    ctx.shader_source(&frag_shader, frag_code);
    ctx.compile_shader(&frag_shader);
    frag_shader
}

fn get_program(ctx: &mut gl, vertex_shader: WebGLShader, frag_shader: WebGLShader) -> WebGLProgram {
    let shader_program = ctx.create_program().unwrap();
    ctx.attach_shader(&shader_program, &vert_shader);
    ctx.attach_shader(&shader_program, &frag_shader);
    ctx.link_program(&shader_program);
    shader_program
}
