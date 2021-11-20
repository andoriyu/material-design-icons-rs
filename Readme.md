# Material Design Icons for Rust

This is a collection of [open-source icons](https://github.com/google/material-design-icons).

Currently it includes just Yew-0.18 components, but this might change in the future. Each style is gated behind feature flag, but all enabled by default. It's Thiccc. Zero promises on how often this is going to get updated.


### Usage

Module layout: `style/group/icon`. Example of styles: `materialicons`, `materialiconsoutlined`. All icons prefixed with `Icon` and follow CamelCase naming. Use just like any other Yew component:

```rust
use material_design_icons::materialicons::action::icon_123::Icon123;

...somewhere in html!{} block
<Icon123 />

```
