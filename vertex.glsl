#version 330 core

layout(location = 1) in vec4 postion;

void main()
{
   gl_Position = postion;
}
