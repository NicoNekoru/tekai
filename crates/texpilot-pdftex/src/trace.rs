use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceEvent {
    pub kind: &'static str,
    pub detail: String,
}

impl TraceEvent {
    pub fn new(kind: &'static str, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }
}

#[derive(Debug)]
pub struct TraceWriter {
    path: PathBuf,
    events: Vec<TraceEvent>,
}

impl TraceWriter {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            events: Vec::new(),
        }
    }

    pub fn push(&mut self, event: TraceEvent) {
        self.events.push(event);
    }

    pub fn write(&self) -> io::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(&self.path)?;
        for event in &self.events {
            writeln!(file, "{}\t{}", event.kind, sanitize(&event.detail))?;
        }
        Ok(())
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

fn sanitize(value: &str) -> String {
    value.replace(['\n', '\r', '\t'], " ")
}
