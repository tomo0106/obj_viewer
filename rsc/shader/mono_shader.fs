#version 140

in vec3 Normal;
const vec3 LIGHT = vec3(5.0, 5.0, 5.0);

void main() {
    float lum = max(dot(normalize(Normal), normalize(LIGHT)), 0.0);
    vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
    gl_FragColor = vec4(color, 1.0);
}