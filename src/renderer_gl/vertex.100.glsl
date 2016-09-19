#version 100

//! Vertex shader source code for OpenGL ES 2.0 (GLSL ES 100)

attribute vec2 vertices;
attribute vec2 texcoords;
uniform ivec2 screen_size;
varying vec2 v_texcoords;

void main(void)
{
    mat2 view_matrix = mat2(2.0/float(screen_size.x),          0.0,
                                    0.0,           -2.0/float(screen_size.y));
    vec2 translation_vector = vec2(-1.0, 1.0);
    gl_Position = vec4(view_matrix * vertices + translation_vector, 0.0, 1.0);
    v_texcoords = texcoords;
}
