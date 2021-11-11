import { memo, useEffect } from "react";
import useWasm from "hooks/useWasm";

export default memo(function Canvas(): JSX.Element {
  const wasm = useWasm();

  useEffect(() => {
    if (wasm) {
      const cleanup = wasm.snake("snake-canvas-parent");
      return () => cleanup;
    }
  }, [wasm]);

  return (
    <div style={{ flex: "1 1 100%" }}>
      <div
        id="snake-canvas-parent"
        style={{
          width: "100%",
          height: "100%",
          position: "relative",
          border: "1px solid black",
        }}
      />
    </div>
  );
});
