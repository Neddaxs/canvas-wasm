import { useEffect, useState } from "react";

const useWasm = () => {
  const [state, setState] = useState(null);
  useEffect(() => {
    const fetchWasm = async () => {
      const wasm = await import("rust/pkg");
      setState(wasm);
    };
    fetchWasm();
  }, []);
  return state;
};

export default useWasm;
