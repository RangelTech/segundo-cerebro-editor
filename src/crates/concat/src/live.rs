//! Live mode (Segundo Cérebro): the Automation server's requests about the
//! project on screen run here, on the window's own session, so the person
//! watches each edit land - in the timeline, in the undo history they use -
//! and can keep editing alongside. Upstream refuses those requests "until
//! live mode lands"; this is that mode, for this fork.

use std::sync::mpsc;
use std::time::Duration;

use concat_api::{LiveOp, LiveReply, WindowLink};

/// How long a request waits for the window. A window busy with a modal
/// dialog answers once it closes; past this the request runs as before
/// (refused, the window holding the project).
const WAIT: Duration = Duration::from_secs(60);

/// The window's side of [`WindowLink`]: hands each request to the event
/// loop and waits for its answer.
pub struct Link;

impl WindowLink for Link {
    fn run(&self, path: &str, op: LiveOp) -> Option<Result<LiveReply, String>> {
        let (answer, wait) = mpsc::channel();
        let path = path.to_owned();
        crate::host::on_ui(move |studio, _app, _models| {
            let _ = answer.send(studio.run_live(&path, op));
        });
        wait.recv_timeout(WAIT).ok().flatten()
    }
}

/// Whether two folder paths are the same folder.
pub fn same_folder(a: &str, b: &str) -> bool {
    let canonical = |path: &str| {
        std::fs::canonicalize(path)
            .map(|path| path.to_string_lossy().into_owned())
            .unwrap_or_else(|_| path.to_owned())
    };
    canonical(a) == canonical(b)
}
