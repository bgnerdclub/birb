# This is using old documentation/planning of the project by @jack


# App
The `App` struct in Birb is what manages everything, its split into 5 parts:
- Entities
- Systems
- Modules
- Events
- Requests
From these 5 parts you can build any game/application, or at least that's the goal

# Entities
Entities are what Unity would call a GameObject, its a thing in the game world. This could be a camera, a player, an enemy or a signpost. Entities are instances of an archetype, an archetype is just a type of entity, the examples before are actually examples of archetype, which are then instantiated as entities. Entities are stored and queried by archetype which is what makes this different from a standard ECS. Here's some example code with entities:
```rust
const birb = @import("birb");

const Player = struct { position: @Vector(f32, 3) };

pub fn main() !void {
    comptime var tracker = birb.TypeTracker{};
    var app = birb.App.init(allocator);
    
    app.add_entity(Player{ .position = .{ 0.0, 0.0, 0.0 } }, &tracker);
}
```
# Systems
Systems do something to an entity, currently the only method called on a System is the run method, which is called once per tick, but eventually there will be others that are called on game start, on entity add, on entity removal, etc. Systems operate on a specific archetype of entities, which is specified in the `Target` field. Here's an example that makes a player move left constantly:

```rust
const birb = @import("birb");
const MultiArrayList = @import("std").MultiArrayList;

const Player = struct { position: @Vector(f32, 3) };
const PlayerSystem = struct { 
    pub const Target = Player;

    pub fn run(_: *PlayerSystem, players: *MultiArrayList(Player)) void {
        for (players.items(.position)) |*position| {
            position[0].* += 1.0;
        }
    }
}

pub fn main() !void {
    comptime var tracker = birb.TypeTracker{};
    var app = birb.App.init(allocator);

    app.add_system(PlayerSystem{}, &tracker);
    app.start();
    app.add_entity(Player{ .position = .{ 0.0, 0.0, 0.0 } }, &tracker);
    app.run();
}
```

The `MultiArrayList` used in all systems is a really nice way of implementing struct-of-arrays, meaning iterating over a specific field is *blazingly fast*.
# Modules
Modules should never have to be written by the end user, they are extensions to the engine that add new features, some examples of modules are the GLFW windowing module and eventually the graphics implementations, renderers, UI, etc. Basically everything is a module, and all the base engine should be is the stuff listed here, this means the engine should be really modular and versatile. Modules currently have a lot more special functions that systems, currently there's:
- run which is called every tick
- init which is called to create the module
- deinit which is called to destroy the module
- start which is called when `app.start()` is called, we need this as well as init because all modules will have been created by now, which is not true for when init is called.

Modules communicate with events and requests, which I'll come onto in a second. I'm not writing an example as the end user should never be writing these, but go look at https://github.com/bgnerdclub/birb/tree/main/birb/src/glfw.zig for an example.

# Events
Events are messages emitted by modules which are then sent to all other modules, current examples would be `Resize` and `SetTitle`, which resizes the window and sets the window title respectively. events are emitted like:
```rust
try app.events.submit(WindowEvent.SetTitle { .title="new title" });
```

Handlers are just functions that can be registered like:
```rust
try app.events.register(handler_function, thing_you_want_to_be_passed_to_handler_function);
```

This is difficult to explain without a context, go look at the file I linked for modules.
# Requests
Requests are like events but they can have a response, which is just returned by the request handler, an example at the moment would be `GetWindow` which can be used to get a `*const Window` which stores information about the window, for example:

```rust
const responses = try app.requests.submit(WindowEvent.GetWindow{});
const window = responses.items[0];
std.debug.print("Width: {d}, Height: {d}", .{ window.size[0], window.size[1] });
```
