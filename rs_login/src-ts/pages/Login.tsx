import koffi from "koffi"
import LibLoad from "../../LibLoad"

export default function Page() { 
  const lib = LibLoad.getLib();
  const get_htm_login = lib.func(
      "get_htm_login",
      "char*",
      []
  );      
  const resp = get_htm_login()    
  return resp;
}
