import { memo, useEffect, useState } from "react";
import chess from "../../utils/chess/chess.func";

export default memo(function Canvas(): JSX.Element {
  const [root, setRoot] = useState<HTMLDivElement>(null);

  useEffect(() => {
    if (root) {
      const cleanup = chess(root)
      return function onunmount(): void {
        cleanup();
      };
    }
  }, [root]);

  return (
    <div
      ref={(el) => setRoot(el)}
      style={{
        width: "100%",
        height: "100%",
        position: "relative",
        border: "1px solid black",
      }}
    />
  );
});
