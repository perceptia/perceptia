#version 300 es

//! Fragment shader source code for OpenGL ES 3.0 (GLSL ES 300)

in highp vec2 v_texcoords;
uniform sampler2D texture;
out highp vec4 color;

void main(void)
{
    color = texture2D(texture, v_texcoords);
}
