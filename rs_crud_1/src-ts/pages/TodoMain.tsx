import koffi from "koffi"
import LibLoad from "../../LibLoad"

export default function Page() { 
  const lib = LibLoad.getLib();
  const get_htm_todo = lib.func(
      "get_htm_todo",
      "char*",
      []
  );      
  const resp = get_htm_todo()    
  return resp;
}
