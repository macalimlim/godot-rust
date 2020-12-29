# Client Server Demo

A very simple example to show peer to peer communication with rpc calls.


## Setup

1. Build the library:

    `cargo build`

2. Import the Godot project in the Godot editor.

3. Open the `Server.tscn` scene and start the current scene with `F6`.

4. Start a second instance of the Godot editor.

5. In the `Remote Port` in the Debug settings of the editor (e.g. 6012).

    Editor > Editor Settings > Network > Debug

5. Open the `Client.tscn` scene and start the current scene with `F6`


## References

* [Godot high-level multiplayer API](https://docs.godotengine.org/en/stable/tutorials/networking/high_level_multiplayer.html)
