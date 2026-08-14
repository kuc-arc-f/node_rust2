import koffi from "koffi"

const LibLoad = {
  getLib: function(){
    const lib = koffi.load("./target/release/libsample1.so");
    return lib;
  },
}
export default LibLoad;
