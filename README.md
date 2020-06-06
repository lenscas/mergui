# WARNING!

Mergui follows quicksilver 0.4, which at point of writing is in an alpha stage.

# Mergui

A simple gui system for the quicksilver engine, originally developed for [Arena keeper](https://github.com/lenscas/arena_keeper_quick) and later ported to quicksilver 0.4 to be used in my yet to be named [card game](https://github.com/lenscas/card_game_client) .

However, I try my best to not let those 2 games shape Mergui and instead focus on how to make Mergui work the best for every game written in Quicksilver.


## Why

When I first started with Quicksilver I couldn't find a gui system that worked with it. The result being predictable, every game made their own GUI system which often didn't go further than a button.

After I did the same for Arena keeper and discovered problems with mine I decided to rewrite it and publish it as its own crate so others won't have to.

## Widgets

There are multiple widgets. The 3 most important ones are
 - Button.
 - Dropdown
 - Text input field.

For a complete list and how to use them, see the [examples](examples). Examples/all.rs has most listed, but doesn't show how to read the state of them.
You can also see the examples in action [here](https://lenscas.github.io/mergui/)

