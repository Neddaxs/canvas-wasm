import { memo, useEffect, useRef } from 'react';
import useWasm from 'hooks/useWasm';

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

      gl.enable(gl.BLEND);
      gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

      const client = wasm.TannerClient.new();
      const initialTime = Date.now();

      let lastDrawTime = -1; // In Milliseconds

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

          const elapsedTime = currTime - initialTime;
          client.update(elapsedTime, window.innerHeight, window.innerWidth);
          client.render();
        }
      }

      render();

      wasm.snake('tanner-canvas-parent');
    }
  }, [wasm, canvasRef]);

  return (
    <div className="flex-1 mx-4 my-8 flex">
      <div id="tanner-canvas-parent" className="flex-1 relative">
        <canvas id="tannerCanvas" ref={canvasRef} />
      </div>
    </div>
  );
}

export default memo(Snake);
