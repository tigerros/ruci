This example shows how to:
- Start a UCI engine connection.
- Send it some initial commands.
- Analyze a custom position.
- Analyze a custom position and receive info messages on a separate thread.

Note that this will print out the (truncated) `Display` impls of the `Info` messages.
That is not a reading from the engine, those are parsed messages converted back into a string
because it is easier to read.

Requires that you have installed Stockfish.