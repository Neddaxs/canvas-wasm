import fragment from './shaders/fragment.frag';
import vertex from './shaders/vertex.vert';
import path from 'path';

import * as glMatrix from 'gl-matrix';

class CubeProgram {
  private _worldMatrix: Float32Array;
  private _viewMatrix: Float32Array;
  private _projectionMatrix: Float32Array;
  private _identityMatrix: Float32Array;
  private _xRotationMatrix: Float32Array;
  private _yRotationMatrix: Float32Array;
  private _gl: WebGL2RenderingContext;
  private _program: WebGLProgram;
  private _worldLocation: WebGLUniformLocation;
  private _viewLocation: WebGLUniformLocation;
  private _projectionLocation: WebGLUniformLocation;
  private _boxIndices: number[];
  private _boxTexture: WebGLTexture;

  constructor(gl: WebGL2RenderingContext, aspect: number) {
    this._gl = gl;

    const image = new Image(0, 0);
    image.src = path.resolve('images/crate.png');

    const vertexShader = gl.createShader(gl.VERTEX_SHADER);
    const fragmentShader = gl.createShader(gl.FRAGMENT_SHADER);

    gl.shaderSource(fragmentShader, fragment);
    gl.compileShader(fragmentShader);
    if (!gl.getShaderParameter(fragmentShader, gl.COMPILE_STATUS)) {
      console.log(
        `Error compiling fragment shader, ${gl.getShaderInfoLog(
          fragmentShader
        )}`
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

    const boxVertices = [
      // X, Y, Z           U, V
      // Top
      -1.0, 1.0, -1.0, 0, 0, -1.0, 1.0, 1.0, 0, 1, 1.0, 1.0, 1.0, 1, 1, 1.0,
      1.0, -1.0, 1, 0,

      // Left
      -1.0, 1.0, 1.0, 0, 0, -1.0, -1.0, 1.0, 1, 0, -1.0, -1.0, -1.0, 1, 1, -1.0,
      1.0, -1.0, 0, 1,

      // Right
      1.0, 1.0, 1.0, 1, 1, 1.0, -1.0, 1.0, 0, 1, 1.0, -1.0, -1.0, 0, 0, 1.0,
      1.0, -1.0, 1, 0,

      // Front
      1.0, 1.0, 1.0, 1, 1, 1.0, -1.0, 1.0, 1, 0, -1.0, -1.0, 1.0, 0, 0, -1.0,
      1.0, 1.0, 0, 1,

      // Back
      1.0, 1.0, -1.0, 0, 0, 1.0, -1.0, -1.0, 0, 1, -1.0, -1.0, -1.0, 1, 1, -1.0,
      1.0, -1.0, 1, 0,

      // Bottom
      -1.0, -1.0, -1.0, 1, 1, -1.0, -1.0, 1.0, 1, 0, 1.0, -1.0, 1.0, 0, 0, 1.0,
      -1.0, -1.0, 0, 1,
    ];

    this._boxIndices = [
      // Top
      0, 1, 2, 0, 2, 3,

      // Left
      5, 4, 6, 6, 4, 7,

      // Right
      8, 9, 10, 8, 10, 11,

      // Front
      13, 12, 14, 15, 14, 12,

      // Back
      16, 17, 18, 16, 18, 19,

      // Bottom
      21, 20, 22, 22, 20, 23,
    ];

    const boxVertexBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, boxVertexBuffer);
    gl.bufferData(
      gl.ARRAY_BUFFER,
      new Float32Array(boxVertices),
      gl.STATIC_DRAW
    );

    const boxIndexBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, boxIndexBuffer);
    gl.bufferData(
      gl.ELEMENT_ARRAY_BUFFER,
      new Uint16Array(this._boxIndices),
      gl.STATIC_DRAW
    );

    const positionLocation = gl.getAttribLocation(program, 'vertPosition');
    const textCoordLocation = gl.getAttribLocation(program, 'vertTexCoord');

    gl.vertexAttribPointer(
      positionLocation,
      3,
      gl.FLOAT,
      false,
      5 * Float32Array.BYTES_PER_ELEMENT,
      0
    );

    gl.vertexAttribPointer(
      textCoordLocation,
      2,
      gl.FLOAT,
      false,
      5 * Float32Array.BYTES_PER_ELEMENT,
      3 * Float32Array.BYTES_PER_ELEMENT
    );

    gl.enableVertexAttribArray(positionLocation);
    gl.enableVertexAttribArray(textCoordLocation);

    // Textures
    this._boxTexture = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, this._boxTexture);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);

    image.onload = () => {
      gl.texImage2D(
        gl.TEXTURE_2D,
        0,
        gl.RGBA,
        gl.RGBA,
        gl.UNSIGNED_BYTE,
        image
      );
      gl.bindTexture(gl.TEXTURE_2D, null);
    };

    // Tell webgl what program we are going to be using to set this variables
    gl.useProgram(program);

    this._worldLocation = gl.getUniformLocation(program, 'mWorld');
    this._viewLocation = gl.getUniformLocation(program, 'mView');
    this._projectionLocation = gl.getUniformLocation(program, 'mProj');

    this._worldMatrix = new Float32Array(16);
    this._viewMatrix = new Float32Array(16);
    this._projectionMatrix = new Float32Array(16);
    this._identityMatrix = new Float32Array(16);
    this._xRotationMatrix = new Float32Array(16);
    this._yRotationMatrix = new Float32Array(16);

    glMatrix.mat4.identity(this._identityMatrix);
    glMatrix.mat4.identity(this._worldMatrix);
    glMatrix.mat4.lookAt(
      this._viewMatrix,
      [0.0, 0.0, -7.0],
      [0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0]
    );
    glMatrix.mat4.perspective(
      this._projectionMatrix,
      glMatrix.glMatrix.toRadian(45),
      aspect,
      0.1,
      1000.0
    );

    gl.uniformMatrix4fv(this._worldLocation, false, this._worldMatrix);
    gl.uniformMatrix4fv(this._viewLocation, false, this._viewMatrix);
    gl.uniformMatrix4fv(
      this._projectionLocation,
      false,
      this._projectionMatrix
    );

    this._program = program;
  }

  render(angle: number) {
    const gl = this._gl;

    gl.useProgram(this._program);
    gl.clearColor(0.75, 0.85, 0.8, 1.0);
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);

    glMatrix.mat4.rotate(
      this._worldMatrix,
      this._identityMatrix,
      angle,
      [0, 1, 0]
    );

    glMatrix.mat4.rotate(
      this._yRotationMatrix,
      this._identityMatrix,
      angle,
      [0, 1, 0]
    );
    glMatrix.mat4.rotate(
      this._xRotationMatrix,
      this._identityMatrix,
      angle / 4,
      [1, 0, 0]
    );
    glMatrix.mat4.mul(
      this._worldMatrix,
      this._yRotationMatrix,
      this._xRotationMatrix
    );

    gl.uniformMatrix4fv(this._worldLocation, false, this._worldMatrix);

    gl.drawElements(
      gl.TRIANGLES,
      this._boxIndices.length,
      gl.UNSIGNED_SHORT,
      0
    );
    gl.bindTexture(gl.TEXTURE_2D, this._boxTexture);
    gl.activeTexture(gl.TEXTURE0);

    gl.drawArrays(gl.TRIANGLES, 0, 3);
  }
}

export default CubeProgram;
