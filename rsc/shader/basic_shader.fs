#version 140

in vec3 FragPosition;

void main()
{
    // red
    // gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);

    // Gradation
    gl_FragColor[0] = gl_FragCoord.x/640.0;
    gl_FragColor[1] = gl_FragCoord.y/480.0;
    gl_FragColor[2] = 0.5;
}