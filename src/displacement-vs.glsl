in vec3 pos;
in vec3 color;

out vec3 v_color;

uniform mat4 transform;
uniform mat4 projection;
uniform mat4 view;

void main() {
  gl_Position = projection * view * transform * vec4(pos, 1.0f);
  v_color = color;
}
