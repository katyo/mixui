//#ifdef GL_ES
//#version 100
//#endif

attribute vec2 position;
uniform vec2 offset;

varying vec3 color;

void main() {
  color = vec3(position + 0.5, 0.0);
  gl_Position = vec4(position + offset, 0.0, 1.0);
}
