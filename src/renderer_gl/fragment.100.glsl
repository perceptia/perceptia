#version 100

//! Fragment shader source code for OpenGL ES 2.0 (GLSL ES 100)

varying highp vec2 v_texcoords;
uniform sampler2D texture;
mediump vec4 color;

void main(void)
{
    color = texture2D(texture, v_texcoords);
    gl_FragColor = vec4(color.b, color.g, color.r, color.a);
}
