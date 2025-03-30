This example shows how to start a UCI connection, send it some initial commands,
start calculating a position, but interrupt it after a couple of seconds.

Note that this will print out the `Display` impls of the `Info` messages.
That is not a reading from the engine, those are parsed messages converted back into a string
because it is easier to read.

This example requires that you have installed Stockfish.