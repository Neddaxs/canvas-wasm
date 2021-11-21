import { memo, useEffect, useRef } from 'react';
import useWasm from 'hooks/useWasm';

import { triangleProgram, renderProgram } from './Tanner.utils';

const FPS_THROTTLE = 1000.0 / 60.0; // milliseconds / frames;

function Snake() {
  const wasm = useWasm();
  const canvasRef = useRef<HTMLCanvasElement>();

  useEffect(() => {
    if (wasm && canvasRef) {
      const canvas = canvasRef.current;
      const gl = canvas.getContext('webgl2');
      if (!gl) {
        alert('Failed to initialize webgl2');
        return;
      }

      let lastDrawTime = -1; // In Milliseconds
      const program = triangleProgram(gl);

      function render() {
        window.requestAnimationFrame(render);
        const currTime = Date.now();
        if (currTime >= lastDrawTime + FPS_THROTTLE) {
          lastDrawTime = currTime;

          if (
            window.innerHeight != canvas.height ||
            window.innerWidth != canvas.width
          ) {
            canvas.height = window.innerHeight;
            canvas.width = window.innerWidth;

            gl.viewport(0, 0, window.innerWidth, window.innerHeight);
          }

          renderProgram(gl, program);
        }
      }

      render();
    }
  }, [wasm, canvasRef]);

  return (
    <div className="flex-1 mx-4 my-8 flex">
      <div className="flex-1 relative">
        <canvas id="tannerCanvas" ref={canvasRef} />
      </div>
    </div>
  );
}

export default memo(Snake);
