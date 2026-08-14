import koffi from "koffi"
import LibLoad from "../../LibLoad"

export default function Page() { 
  const lib = LibLoad.getLib();
  const get_htm_chat = lib.func(
      "get_htm_chat",
      "char*",
      []
  );      
  const resp = get_htm_chat()    
  return resp;
}
