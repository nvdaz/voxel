mod chunks;
mod diagnostics;

use crate::prelude::*;

use self::{chunks::ChunksMenuPlugin, diagnostics::DiagnosticsMenuPlugin};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ChunksMenuPlugin, DiagnosticsMenuPlugin));
    }
}
