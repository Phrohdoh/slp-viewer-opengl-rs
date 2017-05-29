#version 330 core

in vec2 vTexCoord;

uniform usampler2D spriteData;

out vec4 color;

void main() {
    uvec4 rgba = texture(spriteData, vTexCoord);

    if (rgba.a == uint(0)) {
        discard;
    } else {
        float r = float(rgba.r) / 255.0;
        float g = float(rgba.g) / 255.0;
        float b = float(rgba.b) / 255.0;
        float a = float(rgba.a) / 255.0;
        color = vec4(r, g, b, a);
    }
}