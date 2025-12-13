# bevy_pretty_nice_menus

[![crates.io](https://img.shields.io/crates/v/bevy_pretty_nice_menus)](https://crates.io/crates/bevy_pretty_nice_menus)

| bevy  | bevy_pretty_nice_menus |
|-------|------------------------|
| 0.17  | 0.1, 0.2               |

A menu stack abstraction for Bevy.

# Usage

A "menu" in this context is an arbitrary game state, usually one that accepts input.
Each menu goes on the menu stack, with the top menu being the "current" menu.
The basic gameplay state also counts as a menu, and should be the lowest menu on the stack.

Add the [`PrettyNiceMenusPlugin`] plugin to your app.

## [`MenuStack`]

[`push`](MenuStack::push) menu entities onto the stack when you want it to become active, and [`remove`](MenuStack::remove) it when you want it to deactivate.

The top entity on the menu stack triggers the [`ActivateMenu`] event, and the old top entity gets [`DeactivateMenu`].

## [`MenuHidesWhenClosed`]

Menus with this component will automatically have their visibility set to [`Visible`](bevy::prelude::Visibility::Visible) when activated, and [`Hidden`](bevy::prelude::Visibility::Hidden) when deactivated.

## [`MenuDespawnsWhenClosed`]

Menus with this component despawn when deactivated.

## [`MenuWithMouse`] and [`MenuWithoutMouse`]

Menus with these components will either have the mouse unlocked and visible or grabbed and invisible when activated.
There is no default behavior.

Only available with the `visibility` feature.

## [`show_menu_on_event`] and [`close_menu_on_event`]

These are two entity observers, where the entity target is either pushed or removed when the input is triggered.

# Usage with [bevy_pretty_nice_input]

This crate also has compatibility with BPNI if the `pretty_nice_input` feature is enabled.

## [`MenuWithInput`]

The highest menu on the stack that has this component will be the only input on the stack without [`InputDisabled`](bevy_pretty_nice_input::InputDisabled).
This doesn't have to be the top menu on the stack *period* and can be in the middle of the stack.

## [`MenuInputs`] and [`MenuInputOf`]

This is a relationship where, when the input of a menu with [`MenuWithInput`] is enabled or disabled,
all descendents using this relationship are also enabled or disabled.

## [`show_menu_on_action`]

This is an observer that takes a BPNI action type and a marker component type, and when the action is [`JustPressed`](bevy_pretty_nice_input::JustPressed),
the *single* entity with the given marker component is shown.

## [`close_menu_on_action`]

This is an observer that takes a BPNI action type, and when the action is [`JustPressed`](bevy_pretty_nice_input::JustPressed),
the [`input`](bevy_pretty_nice_input::JustPressed::input) of the action is hidden.