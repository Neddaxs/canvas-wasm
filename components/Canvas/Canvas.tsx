import { memo, useEffect, useState } from "react";
import snake from "../../utils/snake/snake.func";

export default memo(function Canvas(): JSX.Element {
  const [root, setRoot] = useState<HTMLDivElement>(null);

  useEffect(() => {
    if (root) {
      const cleanup = snake(root);
      return function onunmount(): void {
        cleanup();
      };
    }
  }, [root]);

  return (
    <div style={{ flex: "1 1 100%" }}>
      <div
        ref={(el) => setRoot(el)}
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
