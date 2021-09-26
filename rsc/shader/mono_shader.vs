#version 140

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;

in vec3 iPosition;
in vec3 iNormal;

out vec3 FragPosition;
out vec3 Normal;

void main()
{
    // flagment position
    // FragPosition=iPosition;
    FragPosition = vec3(uModel * vec4(iPosition, 1.0));
    
    // calc normal
    // Normal = iNormal;
    Normal = mat3(transpose(inverse(uModel))) * iNormal;
    
    // gl position
    gl_Position = uProjection * uView * vec4(FragPosition, 1.0);
}