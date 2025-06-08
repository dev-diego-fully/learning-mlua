local lib_path = "../target/release/?.so;../target/release/lib?.so"

package.cpath = package.cpath .. ";" .. lib_path

return require("hello_world")
