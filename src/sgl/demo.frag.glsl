//#ifdef GL_ES
//#version 100
//#endif

#ifdef GL_FRAGMENT_PRECISION_HIGH
precision highp float;
#else
precision mediump float;
#endif

varying vec3 color;

void main() {
   gl_FragColor = vec4(color, 1.0);
}
