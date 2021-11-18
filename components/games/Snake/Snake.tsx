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
    <div className="flex-1 mx-4 my-8 flex">
      <div id="snake-canvas-parent" className="flex-1 relative" />
    </div>
  );
}

export default memo(Snake);
