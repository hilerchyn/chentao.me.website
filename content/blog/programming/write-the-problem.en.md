+++
title = "Write the problem before the code"
date = "2026-11-03"
summary = "Jumping straight to the implementation leaves only the how in the history. Pin the problem down in a few sentences first."
+++

The easiest thing to skip while programming is writing the problem down. The editor opens, and the hands are already filling in types and branches. Months later the commits show how it was done, not what it was for.

I keep a small order: a few sentences for the problem, then what failure looks like, and only then the implementation. Those sentences can live in a README or above a function. Short is fine. Later I have to be able to read them.

The DAG in Go started the same way: a runnable example of how nodes connect, then the definition. An example is harder to fool yourself with than a comment.

Code goes stale. A problem written clearly still works.
