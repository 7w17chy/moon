#version 300 ES core

layout(location = 1) out vec4 color;
uniform vec4 u_Color;

void main()
{
   color = u_Color;
}
