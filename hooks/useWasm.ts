import { useEffect, useState } from "react";

import * as wasmPackage from "rust/pkg";

const useWasm = () => {
  const [state, setState] = useState<typeof wasmPackage>(null);
  useEffect(() => {
    const fetchWasm = async () => {
      const wasm = wasmPackage;
      setState(wasm);
    };
    fetchWasm();
  }, []);
  return state;
};

export default useWasm;
