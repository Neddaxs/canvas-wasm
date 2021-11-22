import fragment from './shaders/fragment.frag';
import vertex from './shaders/vertex.vert';

export const program = (gl: WebGL2RenderingContext) => {
  gl.clearColor(0.75, 0.85, 0.8, 1.0);
  gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

  const vertexShader = gl.createShader(gl.VERTEX_SHADER);
  const fragmentShader = gl.createShader(gl.FRAGMENT_SHADER);

  gl.shaderSource(fragmentShader, fragment);
  gl.compileShader(fragmentShader);
  if (!gl.getShaderParameter(fragmentShader, gl.COMPILE_STATUS)) {
    console.log(
      `Error compiling fragment shader, ${gl.getShaderInfoLog(fragmentShader)}`
    );
    return;
  }

  gl.shaderSource(vertexShader, vertex);
  gl.compileShader(vertexShader);
  if (!gl.getShaderParameter(vertexShader, gl.COMPILE_STATUS)) {
    console.log(
      `Error compiling fragment shader, ${gl.getShaderInfoLog(vertexShader)}`
    );
    return;
  }

  const program = gl.createProgram();

  gl.attachShader(program, vertexShader);
  gl.attachShader(program, fragmentShader);

  gl.linkProgram(program);
  gl.validateProgram(program);

  if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
    console.log(`Error validating program, ${gl.getProgramInfoLog(program)}`);
    return;
  }

  // X, Y,       R, G, B
  const triangleVertices = [
    0.0, 0.5, 1.0, 1.0, 0.0, -0.5, -0.5, 0.7, 0.0, 1.0, 0.5, -0.5, 0.1, 1.0,
    0.6,
  ];

  const triangleVertexBuffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, triangleVertexBuffer);
  gl.bufferData(
    gl.ARRAY_BUFFER,
    new Float32Array(triangleVertices),
    gl.STATIC_DRAW
  );

  const positionLocation = gl.getAttribLocation(program, 'vertPosition');
  const colorPosition = gl.getAttribLocation(program, 'vertColor');

  gl.vertexAttribPointer(
    positionLocation,
    2,
    gl.FLOAT,
    false,
    5 * Float32Array.BYTES_PER_ELEMENT,
    0
  );

  gl.vertexAttribPointer(
    colorPosition,
    3,
    gl.FLOAT,
    false,
    5 * Float32Array.BYTES_PER_ELEMENT,
    2 * Float32Array.BYTES_PER_ELEMENT
  );

  gl.enableVertexAttribArray(positionLocation);
  gl.enableVertexAttribArray(colorPosition);

  return program;
};

export const renderProgram = (
  gl: WebGL2RenderingContext,
  program: WebGLProgram
) => {
  gl.useProgram(program);
  gl.drawArrays(gl.TRIANGLES, 0, 3);
};

export default {
  create: program,
  render: renderProgram,
};
