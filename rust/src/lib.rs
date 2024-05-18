mod common;
mod in_game;
#[macro_use]
mod macros;

use godot::prelude::*;

struct LoginIslandExtension;

#[gdextension]
unsafe impl ExtensionLibrary for LoginIslandExtension {}
