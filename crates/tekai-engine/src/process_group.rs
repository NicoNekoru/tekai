//! Signal an already owned process group without losing its leader reservation.
//!
//! Darwin killpg filters zombies and returns EPERM when no eligible member is
//! found. Classify only that case with bounded, read-only group metadata. The
//! listed PIDs never become signalling authority and are never individually
//! signalled. Callers retain their unreaped leader until this function returns.

#![warn(clippy::all)]

use rustix::process::{kill_process_group, Pid, Signal};
use std::io;

/// The caller must still own this group's unreaped leader. This helper does not
/// acquire ownership from the numeric ID or from an OS membership listing.
pub(crate) fn signal_owned_group(group: Pid, signal: Signal) -> io::Result<()> {
    loop {
        match kill_process_group(group, signal) {
            Ok(()) | Err(rustix::io::Errno::SRCH) => return Ok(()),
            Err(rustix::io::Errno::INTR) => continue,
            #[cfg(target_os = "macos")]
            Err(rustix::io::Errno::PERM) if darwin::only_zombies(group) => return Ok(()),
            Err(error) => return Err(error.into()),
        }
    }
}

#[cfg(target_os = "macos")]
mod darwin {
    use super::*;
    use rustix::process::{waitid, WaitId, WaitIdOptions};
    use std::mem::{size_of, size_of_val, MaybeUninit};

    // sys/proc_info.h defines PROC_PGRP_ONLY as 2. libc supplies the functions
    // and actual proc_bsdinfo layout, but does not expose this selector.
    const PROC_PGRP_ONLY: u32 = 2;
    const MAX_MEMBERS: usize = 1024;

    #[derive(Clone, Copy)]
    struct Member {
        pid: u32,
        group: u32,
        status: u32,
    }

    fn member_is_zombie(group: Pid, pid: i32, member: Member) -> bool {
        pid > 0
            && member.pid == pid as u32
            && member.group == group.as_raw_pid() as u32
            && member.status == libc::SZOMB
    }

    fn list_count(bytes: i32, capacity: usize) -> Option<usize> {
        let bytes = usize::try_from(bytes).ok()?;
        // Zero is also libproc's error sentinel. A full buffer may have been
        // truncated, so neither case establishes a complete empty group.
        if bytes == 0
            || bytes >= capacity.checked_mul(size_of::<i32>())?
            || bytes % size_of::<i32>() != 0
        {
            return None;
        }
        Some(bytes / size_of::<i32>())
    }

    fn list(group: Pid, ids: &mut [i32; MAX_MEMBERS]) -> Option<usize> {
        // SAFETY The fixed initialized array is aligned and covers buffersize.
        // This selector lists only the owned group, not the global process tree.
        let bytes = unsafe {
            libc::proc_listpids(
                PROC_PGRP_ONLY,
                group.as_raw_pid() as u32,
                ids.as_mut_ptr().cast(),
                size_of_val(ids) as i32,
            )
        };
        let count = list_count(bytes, ids.len())?;
        ids[..count].sort_unstable();
        if ids[..count].iter().any(|&pid| pid <= 0)
            || ids[..count].windows(2).any(|pair| pair[0] == pair[1])
        {
            return None;
        }
        Some(count)
    }

    fn fresh_exit_code(leader: Pid) -> rustix::io::Result<Option<i32>> {
        loop {
            match waitid(
                WaitId::Pid(leader),
                WaitIdOptions::EXITED | WaitIdOptions::NOWAIT | WaitIdOptions::NOHANG,
            ) {
                Err(rustix::io::Errno::INTR) => continue,
                result => return result.map(|status| status.map(|status| status.raw_code())),
            }
        }
    }

    fn unavailable_metadata_is_exited_leader(
        group: Pid,
        pid: i32,
        observe: impl FnOnce(Pid) -> rustix::io::Result<Option<i32>>,
    ) -> bool {
        // P_PID selects this exact direct child. No PID from the membership
        // list may acquire either wait ownership or signalling authority.
        pid == group.as_raw_pid()
            && matches!(
                observe(group),
                Ok(Some(libc::CLD_EXITED | libc::CLD_KILLED | libc::CLD_DUMPED))
            )
    }

    pub(super) fn only_zombies(group: Pid) -> bool {
        let mut first = [0i32; MAX_MEMBERS];
        let Some(count) = list(group, &mut first) else {
            return false;
        };
        for &pid in &first[..count] {
            let mut info = MaybeUninit::<libc::proc_bsdinfo>::zeroed();
            // XNU proc_info.c permits BSDINFO zombie lookup when arg != 0.
            // arg=0 would make a genuine zombie indistinguishable from a query
            // failure. The SDK layout and exact return length are required.
            // SAFETY info is aligned, initialized and has exactly buffersize.
            let bytes = unsafe {
                libc::proc_pidinfo(
                    pid,
                    libc::PROC_PIDTBSDINFO,
                    1,
                    info.as_mut_ptr().cast(),
                    size_of::<libc::proc_bsdinfo>() as i32,
                )
            };
            if bytes as usize != size_of::<libc::proc_bsdinfo>() {
                // Protected system executables can deny BSDINFO even for the
                // parent's reserved zombie. A fresh exit-only WNOWAIT query
                // proves only this exact leader is dead and leaves it unreaped.
                // Every other member still requires its complete SDK record.
                if unavailable_metadata_is_exited_leader(group, pid, fresh_exit_code) {
                    continue;
                }
                return false;
            }
            // SAFETY The record began initialized and the successful call
            // returned its complete size. All fields have integer/byte types.
            let info = unsafe { info.assume_init() };
            if !member_is_zombie(
                group,
                pid,
                Member {
                    pid: info.pbi_pid,
                    group: info.pbi_pgid,
                    status: info.pbi_status,
                },
            ) {
                return false;
            }
        }
        // A parent can fork and exit between the first listing and its status
        // query. Require a second complete identical membership list, instead
        // of overlooking a newly created live descendant. Never use the list
        // to signal members, even if their IDs and status were just validated.
        let mut second = [0i32; MAX_MEMBERS];
        list(group, &mut second).is_some_and(|n| n == count && first[..count] == second[..n])
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn unavailable_metadata_requires_fresh_exact_leader_exit() {
            let group = Pid::from_raw(7123).unwrap();
            for code in [libc::CLD_EXITED, libc::CLD_KILLED, libc::CLD_DUMPED] {
                assert!(unavailable_metadata_is_exited_leader(
                    group,
                    7123,
                    |selected| {
                        assert_eq!(selected, group);
                        Ok(Some(code))
                    }
                ));
                assert!(!unavailable_metadata_is_exited_leader(group, 9123, |_| {
                    panic!("a nonleader must never acquire exit-proof authority")
                }));
            }
            for code in [
                libc::CLD_STOPPED,
                libc::CLD_TRAPPED,
                libc::CLD_CONTINUED,
                0,
                -1,
            ] {
                assert!(!unavailable_metadata_is_exited_leader(group, 7123, |_| {
                    Ok(Some(code))
                }));
            }
            assert!(!unavailable_metadata_is_exited_leader(group, 7123, |_| Ok(
                None
            )));
            for error in [
                rustix::io::Errno::CHILD,
                rustix::io::Errno::PERM,
                rustix::io::Errno::IO,
            ] {
                assert!(!unavailable_metadata_is_exited_leader(
                    group,
                    7123,
                    |_| Err(error)
                ));
            }
            let current = rustix::process::getpid();
            assert_eq!(fresh_exit_code(current), Err(rustix::io::Errno::CHILD));
            assert!(!unavailable_metadata_is_exited_leader(
                current,
                current.as_raw_pid(),
                fresh_exit_code,
            ));
        }

        #[test]
        fn membership_counts_reject_errors_malformed_and_full_lists() {
            assert_eq!(list_count(4, 1024), Some(1));
            for bytes in [-1, 0, 1, 3, 4096, 4100] {
                assert_eq!(list_count(bytes, 1024), None, "bytes {bytes}");
            }
            assert_eq!(list_count(4, usize::MAX), None);
        }

        #[test]
        fn zombie_predicate_requires_exact_pid_group_and_known_status() {
            let group = Pid::from_raw(7123).unwrap();
            assert!(member_is_zombie(
                group,
                9123,
                Member {
                    pid: 9123,
                    group: 7123,
                    status: libc::SZOMB
                }
            ));
            for member in [
                Member {
                    pid: 9124,
                    group: 7123,
                    status: libc::SZOMB,
                },
                Member {
                    pid: 9123,
                    group: 7124,
                    status: libc::SZOMB,
                },
                Member {
                    pid: 9123,
                    group: 7123,
                    status: 2,
                },
                Member {
                    pid: 9123,
                    group: 7123,
                    status: 0,
                },
            ] {
                assert!(!member_is_zombie(group, 9123, member));
            }
            assert!(!member_is_zombie(
                group,
                -1,
                Member {
                    pid: u32::MAX,
                    group: 7123,
                    status: libc::SZOMB
                }
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustix::process::{waitid, WaitId, WaitIdOptions};
    use std::io::{BufRead, BufReader};
    use std::os::unix::process::CommandExt;
    use std::process::{Child, Command, ExitStatus, Stdio};
    #[cfg(target_os = "macos")]
    use std::time::{Duration, Instant};

    struct OwnedGroup {
        child: Option<Child>,
        group: Pid,
    }
    impl OwnedGroup {
        fn spawn(script: &str) -> Self {
            let child = Command::new("/bin/sh")
                .args(["-c", script])
                .process_group(0)
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            let group = Pid::from_raw(child.id() as i32).unwrap();
            Self {
                child: Some(child),
                group,
            }
        }
        fn observe_exit(&self) {
            loop {
                match waitid(
                    WaitId::Pid(self.group),
                    WaitIdOptions::EXITED | WaitIdOptions::NOWAIT,
                ) {
                    Ok(Some(_)) => return,
                    Ok(None) | Err(rustix::io::Errno::INTR) => continue,
                    Err(error) => panic!("cannot observe owned group {error}"),
                }
            }
        }
        fn reap(&mut self) -> ExitStatus {
            let status = self.child.as_mut().unwrap().wait().unwrap();
            // Reaping releases the reserved group ID. No later guard action
            // may signal it, and the Child itself remains normally drop-owned.
            self.child.take();
            status
        }
    }
    impl Drop for OwnedGroup {
        fn drop(&mut self) {
            if let Some(child) = &mut self.child {
                let _ = signal_owned_group(self.group, Signal::KILL);
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }

    #[test]
    fn completed_leader_keeps_status_through_empty_group_cleanup() {
        let mut worker = OwnedGroup::spawn("exit 7");
        worker.observe_exit();
        #[cfg(target_os = "macos")]
        assert!(darwin::only_zombies(worker.group));
        signal_owned_group(worker.group, Signal::KILL).unwrap();
        assert_eq!(worker.reap().code(), Some(7));
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn completed_protected_system_leader_keeps_status_through_group_cleanup() {
        let mut worker = OwnedGroup::spawn("exec /bin/ps -p $$ -o pid=");
        worker.observe_exit();
        assert!(darwin::only_zombies(worker.group));
        signal_owned_group(worker.group, Signal::KILL).unwrap();
        assert_eq!(worker.reap().code(), Some(0));
    }

    #[test]
    fn living_descendant_is_not_misclassified_as_empty_or_zombie_only() {
        let mut worker = OwnedGroup::spawn("sleep 60 & printf '%s\\n' \"$!\"; exit 7");
        let mut line = String::new();
        BufReader::new(worker.child.as_mut().unwrap().stdout.take().unwrap())
            .read_line(&mut line)
            .unwrap();
        let descendant = Pid::from_raw(line.trim().parse().unwrap()).unwrap();
        worker.observe_exit();
        #[cfg(target_os = "macos")]
        assert!(!darwin::only_zombies(worker.group));
        assert_eq!(
            rustix::process::getpgid(Some(descendant)).unwrap(),
            worker.group
        );
        signal_owned_group(worker.group, Signal::KILL).unwrap();
        // Keep leader authority until every remaining member is gone or dead.
        // This observes group metadata only, never signals a listed PID.
        #[cfg(target_os = "macos")]
        {
            let deadline = Instant::now() + Duration::from_secs(5);
            while !darwin::only_zombies(worker.group) {
                assert!(Instant::now() < deadline, "group survived owned KILL");
                std::thread::sleep(Duration::from_millis(10));
            }
        }
        assert_eq!(worker.reap().code(), Some(7));
    }
}
