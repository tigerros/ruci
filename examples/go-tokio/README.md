This example shows how to:
- Start a UCI engine connection.
- Send it some initial commands.
- Analyze a custom position.
- Interrupt the analysis (async-exclusive feature).

Note that this will print out the (truncated) `Display` impls of the `Info` messages.
That is not a reading from the engine, those are parsed messages converted back into a string
because it is easier to read.

This example requires that you have installed Stockfish.