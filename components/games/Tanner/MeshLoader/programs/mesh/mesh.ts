import fragment from './shaders/fragment.frag';
import vertex from './shaders/vertex.vert';
import path from 'path';

import * as glMatrix from 'gl-matrix';
import susanModel from './susan.json';

class MeshProgram {
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
  private _susanIndeces: number[];
  private _susanTexture: WebGLTexture;

  constructor(gl: WebGL2RenderingContext, aspect: number) {
    this._gl = gl;

    const image = new Image(0, 0);
    image.src = path.resolve('images/susan.png');

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

    const susanVertices = susanModel.meshes[0].vertices;

    this._susanIndeces = susanModel.meshes[0].faces.flat();
    const susanTexCoords = susanModel.meshes[0].texturecoords[0];
    const susanNormals = susanModel.meshes[0].normals;

    const susanNormalBuffers = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, susanNormalBuffers);
    gl.bufferData(
      gl.ARRAY_BUFFER,
      new Float32Array(susanNormals),
      gl.STATIC_DRAW
    );

    const susanBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, susanBuffer);
    gl.bufferData(
      gl.ARRAY_BUFFER,
      new Float32Array(susanVertices),
      gl.STATIC_DRAW
    );

    const susanTexBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, susanTexBuffer);
    gl.bufferData(
      gl.ARRAY_BUFFER,
      new Float32Array(susanTexCoords),
      gl.STATIC_DRAW
    );

    const susanIndexBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, susanIndexBuffer);
    gl.bufferData(
      gl.ELEMENT_ARRAY_BUFFER,
      new Uint16Array(this._susanIndeces),
      gl.STATIC_DRAW
    );

    gl.bindBuffer(gl.ARRAY_BUFFER, susanBuffer);
    const positionLocation = gl.getAttribLocation(program, 'vertPosition');
    gl.vertexAttribPointer(
      positionLocation,
      3,
      gl.FLOAT,
      false,
      3 * Float32Array.BYTES_PER_ELEMENT,
      0
    );
    gl.enableVertexAttribArray(positionLocation);

    gl.bindBuffer(gl.ARRAY_BUFFER, susanTexBuffer);
    const textCoordLocation = gl.getAttribLocation(program, 'vertTexCoord');
    gl.vertexAttribPointer(
      textCoordLocation,
      2,
      gl.FLOAT,
      false,
      2 * Float32Array.BYTES_PER_ELEMENT,
      0
    );
    gl.enableVertexAttribArray(textCoordLocation);

    // Lighting
    gl.bindBuffer(gl.ARRAY_BUFFER, susanNormalBuffers);
    const normalLocation = gl.getAttribLocation(program, 'vertNormal');
    gl.vertexAttribPointer(
      normalLocation,
      3,
      gl.FLOAT,
      true,
      3 * Float32Array.BYTES_PER_ELEMENT,
      0
    );
    gl.enableVertexAttribArray(normalLocation);

    // Textures
    this._susanTexture = gl.createTexture();
    gl.bindTexture(gl.TEXTURE_2D, this._susanTexture);
    gl.pixelStorei(gl.UNPACK_FLIP_Y_WEBGL, true);
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

    const ambientLightIntensityLocation = gl.getUniformLocation(
      program,
      'ambientLightIntensity'
    );
    const sunlightIntensityLocation = gl.getUniformLocation(
      program,
      'sun.color'
    );
    const sunlightDirectionLocation = gl.getUniformLocation(
      program,
      'sun.direction'
    );

    gl.uniform3f(ambientLightIntensityLocation, 0.2, 0.2, 0.2);
    gl.uniform3f(sunlightIntensityLocation, 0.9, 0.9, 0.9);
    gl.uniform3f(sunlightDirectionLocation, 3.0, 4.0, -2.0);

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
      this._susanIndeces.length,
      gl.UNSIGNED_SHORT,
      0
    );
    gl.bindTexture(gl.TEXTURE_2D, this._susanTexture);
    gl.activeTexture(gl.TEXTURE0);

    gl.drawArrays(gl.TRIANGLES, 0, 3);
  }
}

export default MeshProgram;
