info("来自 lua 的 info")
warn("来自 lua 的 warn")
error("来自 lua 的 error")
Plugin("kissa_plugin_example") {
  foo = "Hello World"
}
local handle = Plugin("kissa_plugin_example") {
  foo = "Bye World"
}
Unplug(handle)
