//! A single wakeup plus bounded, coalesced paths. The notify callback must not
//! block while a build runs. Overflow requests a whole-root rebuild instead of
//! silently dropping edits or retaining an unbounded event history.

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;

use notify::{Event, EventKind, event::Flag};

pub(super) const MAX_PATHS: usize = 4096;
pub(super) const MAX_PATH_BYTES: usize = 1024 * 1024;

#[derive(Default)]
struct Pending {
    paths: BTreeSet<PathBuf>,
    bytes: usize,
    rescan: bool,
    error: Option<notify::Error>,
}

pub(super) struct Sender {
    pending: Arc<Mutex<Pending>>,
    wake: mpsc::SyncSender<()>,
    ignored_out_dir: PathBuf,
}

pub(super) struct Inbox {
    pending: Arc<Mutex<Pending>>,
    wake: mpsc::Receiver<()>,
}

pub(super) fn channel(ignored_out_dir: PathBuf) -> (Sender, Inbox) {
    let pending = Arc::new(Mutex::new(Pending::default()));
    let (tx, rx) = mpsc::sync_channel(1);
    (
        Sender {
            pending: Arc::clone(&pending),
            wake: tx,
            ignored_out_dir,
        },
        Inbox { pending, wake: rx },
    )
}

impl Sender {
    pub(super) fn send(&self, result: notify::Result<Event>) {
        let mut pending = self
            .pending
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        match result {
            Err(error) => {
                pending.error = Some(error);
                pending.rescan = true;
            }
            Ok(event) => {
                if matches!(event.kind, EventKind::Access(_)) {
                    return;
                }
                pending.rescan |= event.need_rescan();
                if !pending.rescan {
                    for path in event.paths {
                        if super::is_ignored(&path, &self.ignored_out_dir) {
                            continue;
                        }
                        if pending.paths.insert(path.clone()) {
                            pending.bytes += path.as_os_str().len() + 64;
                        }
                        if pending.paths.len() > MAX_PATHS || pending.bytes > MAX_PATH_BYTES {
                            pending.rescan = true;
                            break;
                        }
                    }
                }
            }
        }
        if pending.rescan {
            pending.paths.clear();
            pending.bytes = 0;
        }
        if pending.rescan || !pending.paths.is_empty() {
            let _ = self.wake.try_send(());
        }
    }
}

impl Inbox {
    fn take(&self) -> Event {
        let pending = std::mem::take(
            &mut *self
                .pending
                .lock()
                .unwrap_or_else(|error| error.into_inner()),
        );
        if let Some(error) = pending.error {
            eprintln!("warning: watch event failed: {error}");
        }
        let mut event = Event::new(EventKind::Any);
        event.paths = pending.paths.into_iter().collect();
        if pending.rescan {
            event = event.set_flag(Flag::Rescan);
        }
        event
    }

    pub(super) fn recv(&self) -> Result<notify::Result<Event>, mpsc::RecvError> {
        self.wake.recv()?;
        Ok(Ok(self.take()))
    }
}

pub(super) trait EventReceiver {
    fn recv_timeout(
        &self,
        duration: Duration,
    ) -> Result<notify::Result<Event>, mpsc::RecvTimeoutError>;
}

impl EventReceiver for Inbox {
    fn recv_timeout(
        &self,
        duration: Duration,
    ) -> Result<notify::Result<Event>, mpsc::RecvTimeoutError> {
        self.wake.recv_timeout(duration)?;
        Ok(Ok(self.take()))
    }
}

#[cfg(test)]
impl EventReceiver for mpsc::Receiver<notify::Result<Event>> {
    fn recv_timeout(
        &self,
        duration: Duration,
    ) -> Result<notify::Result<Event>, mpsc::RecvTimeoutError> {
        mpsc::Receiver::recv_timeout(self, duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn repeated_events_coalesce_and_output_is_discarded_before_queueing() {
        let (tx, rx) = channel(PathBuf::from("/project/out"));
        for _ in 0..10000 {
            tx.send(Ok(
                Event::new(EventKind::Any).add_path("/project/main.tex".into())
            ));
            tx.send(Ok(
                Event::new(EventKind::Any).add_path("/project/out/main.log".into())
            ));
        }
        assert_eq!(
            rx.recv().unwrap().unwrap().paths,
            vec![PathBuf::from("/project/main.tex")]
        );
        assert!(matches!(
            rx.recv_timeout(Duration::ZERO),
            Err(mpsc::RecvTimeoutError::Timeout)
        ));
    }

    #[test]
    fn overflow_is_bounded_and_requests_a_full_rescan_without_blocking() {
        let (tx, rx) = channel(PathBuf::from("/project/out"));
        let started = Instant::now();
        for n in 0..10000 {
            tx.send(Ok(
                Event::new(EventKind::Any).add_path(format!("/project/{n}.tex").into())
            ));
        }
        let pending = tx.pending.lock().unwrap();
        assert!(pending.rescan);
        assert!(pending.paths.is_empty());
        assert_eq!(pending.bytes, 0);
        drop(pending);
        assert!(rx.recv().unwrap().unwrap().need_rescan());
        // The sender never waits for a consumer, even after the wakeup is full.
        assert!(started.elapsed() < Duration::from_secs(10));
    }
}
