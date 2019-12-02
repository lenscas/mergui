# Mergui
A simple gui system for the quicksilver engine, developed for use in [Arena keeper](https://github.com/lenscas/arena_keeper_quick)

## Why

Simple, right now there is no GUI system at all for quicksilver and worse the few libraries that compile to WASM are based on wasm-bindgen and as a result not compatible with quicksilver.

This project is created to fill in the gap until a better compatible library shows up.

## Widgets

Right now there are only 3 types of widgets: Buttons, Concealers and images.

There are 3 types of buttons. Ones that can be used to render an image, one for text and one that does both.

A concealer gets a list of other Widgets and a button. The button toggles the visibility and interactivity of the other widgets.
