#version 330 core

in vec2 vTexCoord;
out vec4 fColor;

uniform sampler2D palette;

void main() {
    fColor = texture(palette, vTexCoord);

    // An attempt at debugging.
    if (fColor.w == 0.0) {
        fColor = vec4(0.0, 1.0, 0.0, 1.0);
    }
}