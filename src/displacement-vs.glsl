in vec2 vertex;
in vec3 color;

out vec3 v_color;

uniform mat4 transform;

void main() {
  vec4 transformed_vertex = vec4(vertex, 1.0f, 1.0f) * transform;
  gl_Position = vec4(transformed_vertex.x, transformed_vertex.y, 0.0f, 1.0f);
  v_color = color;
}
