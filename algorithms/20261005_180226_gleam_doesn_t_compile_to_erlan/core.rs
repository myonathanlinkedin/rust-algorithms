use std::fmt;
use std::hash::{Hash, Hasher};

/// Represents the compilation target for the Gleam compiler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Target {
    Erlang,
    JavaScript,
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::Erlang => write!(f, "Erlang"),
            Target::JavaScript => write!(f, "JavaScript"),
        }
    }
}

/// Represents the state of the Gleam compiler's Erlang backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErlangBackendState {
    Active,
    Deprecated,
    Removed,
}

impl fmt::Display for ErlangBackendState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErlangBackendState::Active => write!(f, "Active"),
            ErlangBackendState::Deprecated => write!(f, "Deprecated"),
            ErlangBackendState::Removed => write!(f, "Removed"),
        }
    }
}

/// Represents a specific version of the Gleam compiler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GleamVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl GleamVersion {
    pub const fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    pub fn is_at_least(&self, other: &GleamVersion) -> bool {
        (self.major, self.minor, self.patch) >= (other.major, other.minor, other.patch)
    }
}

impl fmt::Display for GleamVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// Represents a compilation event in the Gleam compiler.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompilationEvent {
    pub version: GleamVersion,
    pub target: Target,
    pub backend_state: ErlangBackendState,
    pub description: String,
}

impl CompilationEvent {
    pub fn new(
        version: GleamVersion,
        target: Target,
        backend_state: ErlangBackendState,
        description: &str,
    ) -> Self {
        Self {
            version,
            target,
            backend_state,
            description: description.to_string(),
        }
    }
}

/// The Gleam Compiler Architecture model.
///
/// This struct encapsulates the historical and current state of the Gleam compiler,
/// specifically tracking the deprecation and removal of the Erlang source code backend.
#[derive(Debug, Clone)]
pub struct GleamCompiler {
    pub current_version: GleamVersion,
    pub erlang_backend_state: ErlangBackendState,
    pub supported_targets: Vec<Target>,
    pub event_history: Vec<CompilationEvent>,
}

impl GleamCompiler {
    /// Creates a new GleamCompiler instance with the current known state.
    ///
    /// As of Gleam v1.0, the Erlang backend was removed. The compiler now
    /// compiles directly to BEAM bytecode (via Erlang/OTP) or JavaScript.
    pub fn new() -> Self {
        let current_version = GleamVersion::new(1, 0, 0);
        let erlang_backend_state = ErlangBackendState::Removed;
        let supported_targets = vec![Target::Erlang, Target::JavaScript];

        let event_history = vec![
            CompilationEvent::new(
                GleamVersion::new(0, 1, 0),
                Target::Erlang,
                ErlangBackendState::Active,
                "Initial release with Erlang source code backend",
            ),
            CompilationEvent::new(
                GleamVersion::new(0, 2, 0),
                Target::JavaScript,
                ErlangBackendState::Active,
                "Added JavaScript backend",
            ),
            CompilationEvent::new(
                GleamVersion::new(0, 3, 0),
                Target::Erlang,
                ErlangBackendState::Deprecated,
                "Erlang source code backend deprecated in favor of direct BEAM compilation",
            ),
            CompilationEvent::new(
                current_version,
                Target::Erlang,
                ErlangBackendState::Removed,
                "Erlang source code backend removed; compiler now emits BEAM bytecode directly",
            ),
        ];

        Self {
            current_version,
            erlang_backend_state,
            supported_targets,
            event_history,
        }
    }

    /// Returns the current state of the Erlang backend.
    pub fn erlang_backend_state(&self) -> ErlangBackendState {
        self.erlang_backend_state
    }

    /// Returns true if the Erlang source code backend is no longer active.
    pub fn erlang_source_backend_removed(&self) -> bool {
        self.erlang_backend_state == ErlangBackendState::Removed
    }

    /// Returns the list of supported compilation targets.
    pub fn supported_targets(&self) -> &[Target] {
        &self.supported_targets
    }

    /// Returns the full event history of backend state changes.
    pub fn event_history(&self) -> &[CompilationEvent] {
        &self.event_history
    }

    /// Returns the version at which the Erlang source backend was removed.
    pub fn erlang_backend_removal_version(&self) -> Option<GleamVersion> {
        self.event_history
            .iter()
            .rev()
            .find(|e| e.backend_state == ErlangBackendState::Removed)
            .map(|e| e.version)
    }

    /// Returns the version at which the Erlang source backend was deprecated.
    pub fn erlang_backend_deprecation_version(&self) -> Option<GleamVersion> {
        self.event_history
            .iter()
            .rev()
            .find(|e| e.backend_state == ErlangBackendState::Deprecated)
            .map(|e| e.version)
    }

    /// Generates a human-readable summary of the compiler's Erlang backend status.
    pub fn backend_status_summary(&self) -> String {
        let removal_version = self
            .erlang_backend_removal_version()
            .map(|v| v.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        format!(
            "Gleam v{}: Erlang source code backend is {}. \
             Removed in v{}. Supported targets: {}.",
            self.current_version,
            self.erlang_backend_state,
            removal_version,
            self
                .supported_targets
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    /// Validates that the event history is chronologically consistent.
    pub fn validate_history(&self) -> bool {
        for window in self.event_history.windows(2) {
            if window[0].version > window[1].version {
                return false;
            }
        }
        true
    }
}

impl Default for GleamCompiler {
    fn default() -> Self {
        Self::new()
    }
}
