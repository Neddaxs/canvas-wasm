import { memo, useEffect } from 'react';
import useWasm from 'hooks/useWasm';

function Snake() {
  const wasm = useWasm();

  useEffect(() => {
    if (wasm) {
      wasm.snake('snake-canvas-parent');
    }
  }, [wasm]);

  return (
    <div className="h-full w-full">
      <div
        id="snake-canvas-parent"
        style={{
          width: '100%',
          height: '100%',
          position: 'relative',
          border: '1px solid black',
        }}
      />
    </div>
  );
}

export default memo(Snake);
