#version 330 core
layout (location = 0) in vec3 vertexPosition;

uniform mat4 transform;

void main()
{
    gl_Position = vec4(vertexPosition, 1.0) * transform;
}
