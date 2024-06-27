# bevy-fps-ui
Simple and fancy FPS counter for Bevy Game Engine
![image](https://i.imgur.com/9Xbl1q7.png)
### Instalation
Via terminal:
`cargo add bevy-fps-ui`

Or add this to your Cargo.toml dependencaies:
`bevy-fps-ui = "0.1.3"`

### Usage
Basically add `FpsCounterPlugin` to your app:
```rust
use bevy::prelude::*;
use bevy_fps_ui::*; 

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FpsCounterPlugin)
        .run();
}
```

### Example
You can run example with
`cargo run --example basic`
also see [`basic example`](examples/basic)

## Compatibility with Bevy
| `bevy-fps-ui`   | `bevy` |
| :--             | :--    |
| `0.1.*`         | `0.13` |
