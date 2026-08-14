import koffi from "koffi"
import LibLoad from "../../LibLoad"

export default function Page() { 
  const lib = LibLoad.getLib();
  const get_htm_about = lib.func(
      "get_htm_about",
      "char*",
      []
  );      
  const resp = get_htm_about()    
  return resp;
}
