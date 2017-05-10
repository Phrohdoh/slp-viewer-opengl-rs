#version 330 core

layout (location = 0) in vec2 pos;
layout (location = 1) in vec2 uv;

uniform mat4 mOrtho;

out vec2 vTexCoord;

void main() {
    vTexCoord = uv;
    gl_Position = mOrtho * vec4(pos, 0.0, 1.0);
}