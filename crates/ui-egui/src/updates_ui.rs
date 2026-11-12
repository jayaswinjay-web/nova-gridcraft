//! Software updates: this copy of NOVA Grid never goes online.
//!
//! The NOVA Workspace suite is shipped as one complete bundle, so there is
//! nothing to fetch over the network. The commands that used to start a check
//! still work; they only say that this copy does not check for updates, and
//! no address is ever contacted.

use crate::SheetApp;

/// The update runtime. There is no updater behind it: the suite never connects out.
pub(crate) struct Runtime {
    /// True once a manual check was asked for and the local note is pending.
    pub(crate) manual: bool,
}

impl Default for Runtime {
    fn default() -> Self {
        Self { manual: false }
    }
}

/// Ask for a check. A manual one answers locally; the start-up one is ignored.
pub(crate) fn start(app: &mut SheetApp, manual: bool) {
    if manual {
        app.updates_rt.manual = true;
    }
}

/// There is no background work to do: the pending note is all that is left.
pub(crate) fn tick(app: &mut SheetApp, _ctx: &egui::Context) {
    if std::mem::take(&mut app.updates_rt.manual) {
        app.message = Some((
            "NOVA Grid".into(),
            "This copy of NOVA Grid does not check for updates. The NOVA Workspace suite is installed as one bundle.".into(),
        ));
    }
}

/// Nothing to show: the note goes through the app's own message area.
pub(crate) fn show(_app: &mut SheetApp, _ctx: &egui::Context) {}
