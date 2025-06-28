--- Represents an event handler that dispatches events to registered listeners.
--- Listeners are stored in a hash set, meaning duplicate functions are not
--- registered and no specific call order is guaranteed for listeners of the same event.
---@class LuaEventHandler
local LuaEventHandler = {};

--- Creates a new `LuaEventHandler` instance.
---@return LuaEventHandler
function LuaEventHandler.new() end

--- Triggers an event, calling all registered listeners for the given event name.
--- Listeners receive a single `event` table as an argument, which contains a `values` field.
---@param self LuaEventHandler
---@param eventName string The name of the event to trigger.
---@param datas? table Optional table of data to be included in the `event.values` field.
function LuaEventHandler.trigger_event(self, eventName, datas) end

--- Registers one or more functions as listeners for a specific event.
--- Duplicate listener functions for the same event are ignored.
---@param self LuaEventHandler
---@param eventName string The name of the event to listen for.
---@param ... fun(event: table) One or more functions to be added as listeners.
---                       The `event` table passed to the listener contains a `values` field.
function LuaEventHandler.add_event_listeners(self, eventName, ...) end

--- Removes one or more functions as listeners for a specific event.
--- If a function was not registered, this operation has no effect.
---@param self LuaEventHandler
---@param eventName string The name of the event from which to remove listeners.
---@param ... fun(event: table) One or more functions to be removed as listeners.
function LuaEventHandler.remove_event_listeners(self, eventName, ...) end

return LuaEventHandler