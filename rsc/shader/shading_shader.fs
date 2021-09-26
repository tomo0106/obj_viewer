#version 140

struct Material {
    vec3 specular;  // 鏡面反射の強さ
    float shininess;// 発光の強さ
};

struct Light {
    vec3 direction; // 照明の光が指すベクトル
    vec3 ambient;   // 環境光の強さ
    vec3 diffuse;   // 拡散光の強さ
    vec3 specular;  // 鏡面反射の強さ
};

// in float Alpha;
in vec3 FragPosition;
in vec3 Normal;
//in vec2 TexCoords;

// uniform sampler2D uScreenTexture;
// uniform vec3 uViewPosition;
// uniform Material uMaterial;
// uniform Light uLight;

const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

void main()
{
    // ambient
    vec3 ambient = uLight.ambient * texture(uScreenTexture, TexCoords).rgb;

    // diffuse
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(-uLight.direction);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = uLight.diffuse * diff * texture(uScreenTexture, TexCoords).rgb;

    // specular
    vec3 viewDir = normalize(uViewPosition - FragPosition);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), uMaterial.shininess);
    vec3 specular = uLight.specular * spec * uMaterial.specular;

    vec3 result = ambient + diffuse + specular;

    gl_FragColor = vec4(result, Alpha);
}