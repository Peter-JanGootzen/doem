in vec3 vertex;
in vec3 color;

out vec3 v_color;

uniform mat3 transform;

void main() {
  gl_Position = vec4(transform * vertex, 1.);
  v_color = color;
}
