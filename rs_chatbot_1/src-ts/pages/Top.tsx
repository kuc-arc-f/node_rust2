import koffi from "koffi"
import LibLoad from "../../LibLoad"

export default function Page() { 
  const lib = LibLoad.getLib();
  const ssr_htm_top = lib.func(
      "ssr_htm_top",
      "char*",
      []
  );      
  const resp = ssr_htm_top()    
  return resp;
}
