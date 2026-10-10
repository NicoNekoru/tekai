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
    use proptest::prelude::*;
    use proptest::test_runner::TestCaseResult;

    #[derive(Clone, Debug)]
    struct GeneratedEvent {
        kind: u8,
        paths: Vec<(u8, u8)>,
        rescan: bool,
        drain: bool,
    }

    fn generated_events() -> impl Strategy<Value = Vec<GeneratedEvent>> {
        prop::collection::vec(
            (
                0..5u8,
                prop::collection::vec((0..8u8, 0..16u8), 0..9),
                any::<bool>(),
                any::<bool>(),
            )
                .prop_map(|(kind, paths, rescan, drain)| GeneratedEvent {
                    kind,
                    paths,
                    rescan,
                    drain,
                }),
            0..81,
        )
    }

    // The boolean is part of the generated input's contract, rather than a
    // call to the production ignore filter. Sibling and lookalike names matter.
    fn generated_path(kind: u8, id: u8) -> (PathBuf, bool) {
        let (directory, ignored) = match kind {
            0 => ("/project", false),
            1 => ("/project/out", true),
            2 => ("/project/outside", false),
            3 => ("/project/.git", true),
            4 => ("/project/nested/target", true),
            5 => ("/project/.tekai/deep", true),
            6 => ("/project/.gitish", false),
            _ => ("/project/src", false),
        };
        (PathBuf::from(format!("{directory}/{id}.tex")), ignored)
    }

    #[derive(Default)]
    struct ReferencePending {
        paths: Vec<PathBuf>,
        rescan: bool,
        error: bool,
    }

    impl ReferencePending {
        fn send(&mut self, input: &GeneratedEvent) {
            if input.kind == 4 {
                self.error = true;
                self.rescan = true;
            } else if input.kind != 3 {
                self.rescan |= input.rescan;
                if !self.rescan {
                    for &(kind, id) in &input.paths {
                        let (path, ignored) = generated_path(kind, id);
                        if !ignored && !self.paths.contains(&path) {
                            self.paths.push(path);
                        }
                    }
                }
            }
            if self.rescan {
                self.paths.clear();
            }
        }

        fn check_pending(&self, tx: &Sender) -> TestCaseResult {
            let actual = tx.pending.lock().unwrap();
            let mut expected = self.paths.clone();
            expected.sort();
            prop_assert_eq!(actual.paths.iter().cloned().collect::<Vec<_>>(), expected);
            prop_assert_eq!(actual.rescan, self.rescan);
            prop_assert_eq!(actual.error.is_some(), self.error);
            prop_assert_eq!(
                actual.bytes,
                self.paths
                    .iter()
                    .map(|path| path.as_os_str().len() + 64)
                    .sum::<usize>()
            );
            prop_assert!(actual.paths.len() <= MAX_PATHS);
            prop_assert!(actual.bytes <= MAX_PATH_BYTES);
            Ok(())
        }

        fn drain(&mut self, rx: &Inbox) -> TestCaseResult {
            let result = rx.recv_timeout(Duration::ZERO);
            if self.rescan || !self.paths.is_empty() {
                let event = result.unwrap().unwrap();
                self.paths.sort();
                prop_assert_eq!(&event.paths, &self.paths);
                prop_assert_eq!(event.need_rescan(), self.rescan);
            } else {
                prop_assert!(matches!(result, Err(mpsc::RecvTimeoutError::Timeout)));
            }
            *self = Self::default();
            // Coalescing must leave at most one wakeup, not one per send.
            prop_assert!(matches!(
                rx.recv_timeout(Duration::ZERO),
                Err(mpsc::RecvTimeoutError::Timeout)
            ));
            Ok(())
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 64,
            max_shrink_iters: 4096,
            ..ProptestConfig::default()
        })]

        #[test]
        fn generated_batches_match_the_reference_across_drains(inputs in generated_events()) {
            let (tx, rx) = channel(PathBuf::from("/project/out"));
            let mut model = ReferencePending::default();
            for input in inputs {
                let result = if input.kind == 4 {
                    Err(notify::Error::generic("generated watcher failure"))
                } else {
                    let kind = match input.kind {
                        0 => EventKind::Any,
                        1 => EventKind::Create(notify::event::CreateKind::File),
                        2 => EventKind::Modify(notify::event::ModifyKind::Any),
                        _ => EventKind::Access(notify::event::AccessKind::Read),
                    };
                    let mut event = Event::new(kind);
                    event.paths = input.paths.iter().map(|&(kind, id)| generated_path(kind, id).0).collect();
                    if input.rescan {
                        event = event.set_flag(Flag::Rescan);
                    }
                    Ok(event)
                };
                tx.send(result);
                model.send(&input);
                model.check_pending(&tx)?;
                if input.drain {
                    model.drain(&rx)?;
                    model.check_pending(&tx)?;
                }
            }
            model.drain(&rx)?;
            model.check_pending(&tx)?;
        }

        #[test]
        fn generated_limit_boundaries_rescan_exactly_when_a_cap_is_exceeded(
            by_count in any::<bool>(),
            short_name in 0..97usize,
            long_name in 256..2049usize,
            offset in -2i8..3,
            late_paths in prop::collection::vec((0..8u8, 0..16u8), 0..33),
        ) {
            let (tx, rx) = channel(PathBuf::from("/project/out"));
            let name_length = if by_count { short_name } else { long_name };
            let padding = "x".repeat(name_length);
            let path_bytes = format!("/project/00000-{padding}").len() + 64;
            let capacity = if by_count { MAX_PATHS } else { MAX_PATH_BYTES / path_bytes };
            let count = capacity.saturating_add_signed(isize::from(offset));
            for id in 0..count {
                let path = PathBuf::from(format!("/project/{id:05}-{padding}"));
                // A duplicate must not spend another count or byte allowance.
                tx.send(Ok(Event::new(EventKind::Any).add_path(path.clone()).add_path(path)));
            }
            let overflow = count > MAX_PATHS || count * path_bytes > MAX_PATH_BYTES;
            if overflow {
                // A full-root rescan stays latched as more edits arrive, even
                // after the single wakeup has filled the channel.
                for (kind, id) in late_paths {
                    tx.send(Ok(Event::new(EventKind::Any).add_path(generated_path(kind, id).0)));
                    let pending = tx.pending.lock().unwrap();
                    prop_assert!(pending.rescan);
                    prop_assert!(pending.paths.is_empty());
                    prop_assert_eq!(pending.bytes, 0);
                }
            }
            let pending = tx.pending.lock().unwrap();
            prop_assert_eq!(pending.rescan, overflow);
            prop_assert_eq!(pending.paths.len(), if overflow { 0 } else { count });
            prop_assert_eq!(pending.bytes, if overflow { 0 } else { count * path_bytes });
            drop(pending);
            let event = rx.recv_timeout(Duration::ZERO).unwrap().unwrap();
            prop_assert_eq!(event.need_rescan(), overflow);
            prop_assert_eq!(event.paths.len(), if overflow { 0 } else { count });
            prop_assert!(event.paths.windows(2).all(|pair| pair[0] < pair[1]));
            prop_assert!(matches!(rx.recv_timeout(Duration::ZERO), Err(mpsc::RecvTimeoutError::Timeout)));

            // Taking a rescan restores an empty inbox that accepts precise paths.
            let next = PathBuf::from("/project/next.tex");
            tx.send(Ok(Event::new(EventKind::Any).add_path(next.clone())));
            let event = rx.recv_timeout(Duration::ZERO).unwrap().unwrap();
            prop_assert!(!event.need_rescan());
            prop_assert_eq!(event.paths, vec![next]);
        }
    }
}
