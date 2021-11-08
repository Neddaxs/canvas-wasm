import { memo, useEffect, useState } from "react";

export default memo(function Canvas(): JSX.Element {
  const [root, setRoot] = useState<HTMLDivElement>(null);

  useEffect(() => {
    if (root) {
      return function onunmount(): void {
        console.log("unmounting");
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
