#version 100

//! Fragment shader source code for OpenGL ES 2.0 (GLSL ES 100)

varying highp vec2 v_texcoords;
uniform sampler2D texture;

void main(void)
{
    gl_FragColor = texture2D(texture, v_texcoords);
}
