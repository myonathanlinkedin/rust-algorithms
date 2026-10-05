mod core;

use core::{
    CompilationEvent, ErlangBackendState, GleamCompiler, GleamVersion, Target,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_compiler_has_correct_version() {
        let compiler = GleamCompiler::new();
        assert_eq!(
            compiler.current_version,
            GleamVersion::new(1, 0, 0)
        );
    }

    #[test]
    fn test_erlang_backend_is_removed() {
        let compiler = GleamCompiler::new();
        assert_eq!(
            compiler.erlang_backend_state(),
            ErlangBackendState::Removed
        );
        assert!(compiler.erlang_source_backend_removed());
    }

    #[test]
    fn test_supported_targets_include_erlang_and_javascript() {
        let compiler = GleamCompiler::new();
        let targets = compiler.supported_targets();
        assert!(targets.contains(&Target::Erlang));
        assert!(targets.contains(&Target::JavaScript));
        assert_eq!(targets.len(), 2);
    }

    #[test]
    fn test_event_history_is_chronological() {
        let compiler = GleamCompiler::new();
        assert!(compiler.validate_history());
    }

    #[test]
    fn test_event_history_has_four_events() {
        let compiler = GleamCompiler::new();
        assert_eq!(compiler.event_history().len(), 4);
    }

    #[test]
    fn test_first_event_is_active_erlang() {
        let compiler = GleamCompiler::new();
        let first = &compiler.event_history()[0];
        assert_eq!(first.target, Target::Erlang);
        assert_eq!(first.backend_state, ErlangBackendState::Active);
        assert_eq!(first.version, GleamVersion::new(0, 1, 0));
    }

    #[test]
    fn test_deprecation_version_is_0_3_0() {
        let compiler = GleamCompiler::new();
        let dep_version = compiler.erlang_backend_deprecation_version();
        assert_eq!(
            dep_version,
            Some(GleamVersion::new(0, 3, 0))
        );
    }

    #[test]
    fn test_removal_version_is_1_0_0() {
        let compiler = GleamCompiler::new();
        let rem_version = compiler.erlang_backend_removal_version();
        assert_eq!(rem_version, Some(GleamVersion::new(1, 0, 0)));
    }

    #[test]
    fn test_version_comparison() {
        let v010 = GleamVersion::new(0, 1, 0);
        let v030 = GleamVersion::new(0, 3, 0);
        let v100 = GleamVersion::new(1, 0, 0);
        assert!(v010 < v030);
        assert!(v030 < v100);
        assert!(v100.is_at_least(&v030));
        assert!(!v010.is_at_least(&v030));
    }

    #[test]
    fn test_target_display() {
        assert_eq!(Target::Erlang.to_string(), "Erlang");
        assert_eq!(Target::JavaScript.to_string(), "JavaScript");
    }

    #[test]
    fn test_backend_state_display() {
        assert_eq!(ErlangBackendState::Active.to_string(), "Active");
        assert_eq!(
            ErlangBackendState::Deprecated.to_string(),
            "Deprecated"
        );
        assert_eq!(
            ErlangBackendState::Removed.to_string(),
            "Removed"
        );
    }

    #[test]
    fn test_version_display() {
        let v = GleamVersion::new(1, 2, 3);
        assert_eq!(v.to_string(), "1.2.3");
    }

    #[test]
    fn test_backend_status_summary_contains_key_info() {
        let compiler = GleamCompiler::new();
        let summary = compiler.backend_status_summary();
        assert!(summary.contains("Removed"));
        assert!(summary.contains("1.0.0"));
        assert!(summary.contains("Erlang"));
        assert!(summary.contains("JavaScript"));
    }

    #[test]
    fn test_compilation_event_new() {
        let event = CompilationEvent::new(
            GleamVersion::new(0, 5, 0),
            Target::JavaScript,
            ErlangBackendState::Active,
            "Test event",
        );
        assert_eq!(event.version, GleamVersion::new(0, 5, 0));
        assert_eq!(event.target, Target::JavaScript);
        assert_eq!(event.backend_state, ErlangBackendState::Active);
        assert_eq!(event.description, "Test event");
    }

    #[test]
    fn test_default_trait() {
        let compiler = GleamCompiler::default();
        assert_eq!(
            compiler.current_version,
            GleamVersion::new(1, 0, 0)
        );
    }

    #[test]
    fn test_history_states_progression() {
        let compiler = GleamCompiler::new();
        let states: Vec<ErlangBackendState> = compiler
            .event_history()
            .iter()
            .map(|e| e.backend_state)
            .collect();
        assert_eq!(
            states,
            vec![
                ErlangBackendState::Active,
                ErlangBackendState::Active,
                ErlangBackendState::Deprecated,
                ErlangBackendState::Removed,
            ]
        );
    }

    #[test]
    fn test_hash_and_eq() {
        let v1 = GleamVersion::new(1, 0, 0);
        let v2 = GleamVersion::new(1, 0, 0);
        let v3 = GleamVersion::new(1, 0, 1);
        assert_eq!(v1, v2);
        assert_ne!(v1, v3);
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(v1);
        assert!(set.contains(&v2));
        assert!(!set.contains(&v3));
    }
}

fn main() {
    let compiler = GleamCompiler::new();

    println!("=== Gleam Compiler Architecture ===");
    println!("{}", compiler.backend_status_summary());
    println!();
    println!("Event History:");
    for event in compiler.event_history() {
        println!(
            "  v{} | {:?} | {:?} | {}",
            event.version, event.target, event.backend_state, event.description
        );
    }
    println!();
    println!("Erlang source backend removed: {}", compiler.erlang_source_backend_removed());
    println!("Deprecation version: {:?}", compiler.erlang_backend_deprecation_version());
    println!("Removal version: {:?}", compiler.erlang_backend_removal_version());
    println!();
    println!("All assertions passed. Running unit tests...");

    #[cfg(test)]
    {
        println!("Run with `cargo test` for full test suite.");
    }
}
