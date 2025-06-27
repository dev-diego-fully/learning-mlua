---@class LuaEventHandler
local LuaEventHandler = {};

---
---@return LuaEventHandler
function LuaEventHandler.new() end

---
---@param self LuaEventHandler
---@param eventName string
---@param datas table
function LuaEventHandler.trigger(self, eventName, datas) end

---
---@param self LuaEventHandler
---@param eventName string
---@param ... fun(event: table)
function LuaEventHandler.when(self, eventName, ...) end

return LuaEventHandler