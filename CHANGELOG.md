# Changelog

All notable changes to this project are documented in this file.

## Overview

- [upcomming](#upcomming)
- [`0.1.0-alpha0.3`](#0.1.0-alpha0.4)
- [`0.1.0-alpha0.3`](#0.1.0-alpha0.3)
- [`0.1.0-alpha0.2`](#0.1.0-alpha0.2)
- [`0.1.0-alpha0.1`](#0.1.0-alpha0.1)
- [`0.0.5`](#005)
- [`0.0.4`](#004)
- [`0.0.3`](#003)

## upcomming

- [BREAKING] dropping the LayerId does not cause the layer to be removed. Only if the layer is also empty does that happen now.
- [BREAKING] LayerId is now !Send.
- [BREAKING] Context::new() stopped taking the mouse location. There is no way (yet) to get it at the location that you want to create a context.
- [BREAKING] Reworked how concealers work and are configured. Short description: It now controls a layer instead of list of widgets.
- [BREAKING] Replaced the iter/iter_mut methods from ConcealerManager. There are no channels to loop over. Instead, they give references to the layers.
- break up the examples so all.rs isn't the only example.
- Add a get_layer/get_layer_mut for ConcealerManager. Gets the layer from a specific Concealer that it manages.
- Add a SingularLayerId, which is similar to a normal layer, except that it has some restrictions on what users can do with it. It is used for Concealers.
- Remove unesesary lifetime parameter in Context.
- Implement clone for LayerId.

## 0.1.0-alpha0.4

- Improve the `all` example
- [BREAKING] Remove WidgetId from public interface. There was no reason for it to be public

## 0.1.0-alpha0.3

- Fix dropdown being totally broken

## 0.1.0-alpha0.2

- Update to quicksilver 0.4-alpha0.4
- Replace all Arc<Mutex<T>> with Rc<RefCell<T>>
- [BREAKING] remove DropDown::values()

## 0.1.0-alpha0.1

- Update to quicksilver 0.4-alpha0.3
- Add text input field
- Add an example to show all widgets

## 0.0.5

- feat: Add Text widget ([`92c9995`])
- feat: Add dropdown widget ([`40c145f`])
- chore: Update changelog ([`0c0ba84`])

## 0.0.4

_2019.12.14_

### Contributions

This release is made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖

- lenscas (<lenscas@gmail.com>)

### Changes

#### Miscellaneous Tasks

- **Add changelog** ([`80da6b6`])

- **Increase version number** ([`d16d7c9`])

#### Bug Fixes

- **layer events not being processed in render** ([`bd8fc46`])

## 0.0.3

_2019.12.12_

### Changes

<!-- [releases] -->

[unreleased]: #/compare/v0.0.4...HEAD
[0.0.4]: #/releases/tag/v0.0.4
[0.0.3]: #/releases/tag/v0.0.3

<!-- [commits] -->

[`92c9995`]: #/commit/92c999521ef453cd8bce57e6ec1ccffa1e934115
[`40c145f`]: #/commit/40c145f47c0fc5eb173c6cb153ab4541ea20d117
[`0c0ba84`]: #/commit/0c0ba84d87e2c5d594dd22a655777435f65bb91f
[`80da6b6`]: #/commit/80da6b6557a7b3ad4b7544f54a29f7c362cf8c12
[`d16d7c9`]: #/commit/d16d7c97e65c7518d0eaa4578b5cbbb37073a36f
[`bd8fc46`]: #/commit/bd8fc4695d95bfdaceb830197236603fe067ce35
