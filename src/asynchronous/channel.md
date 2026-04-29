# Message Passing

When your program has multiple tasks running at the same time, they sometimes need to talk to each 
other. Channels are how you do that, one task sends a message, another receives it. 
Think of it like a pipe: you put something in one end, and it comes out the other.

{{#include channel/bounded.md}}
{{#include channel/unbounded.md}}

{{#include ../links.md}}
