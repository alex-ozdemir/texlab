use base_db::{util::filter_regex_patterns, Document, Owner, Workspace};
use multimap::MultiMap;
use rustc_hash::FxHashMap;
use url::Url;

use crate::types::Diagnostic;

/// Manages all diagnostics for a workspace.
#[derive(Debug, Default)]
pub struct Manager {
    grammar: MultiMap<Url, Diagnostic>,
    chktex: FxHashMap<Url, Vec<Diagnostic>>,
    build_log: FxHashMap<Url, MultiMap<Url, Diagnostic>>,
}

impl Manager {
    /// Updates the syntax-based diagnostics for the given document.
    pub fn update_syntax(&mut self, workspace: &Workspace, document: &Document) {
        if !Self::is_relevant(document) {
            return;
        }

        self.grammar.remove(&document.uri);
        super::grammar::tex::update(document, workspace.config(), &mut self.grammar);
        super::grammar::bib::update(document, &mut self.grammar);

        self.build_log.remove(&document.uri);
        super::build_log::update(workspace, document, &mut self.build_log);
    }

    /// Updates the ChkTeX diagnostics for the given document.
    pub fn update_chktex(&mut self, uri: Url, diagnostics: Vec<Diagnostic>) {
        self.chktex.insert(uri, diagnostics);
    }

    /// Returns all filtered diagnostics for the given workspace.
    pub fn get(&self, workspace: &Workspace) -> MultiMap<Url, Diagnostic> {
        let mut results = MultiMap::default();
        for (uri, diagnostics) in &self.grammar {
            results.insert_many_from_slice(uri.clone(), diagnostics);
        }

        for (uri, diagnostics) in self.build_log.values().flatten() {
            results.insert_many_from_slice(uri.clone(), diagnostics);
        }

        for (uri, diagnostics) in &self.chktex {
            if workspace
                .lookup(uri)
                .map_or(false, |document| document.owner == Owner::Client)
            {
                results.insert_many_from_slice(uri.clone(), diagnostics);
            }
        }

        for document in workspace
            .iter()
            .filter(|document| Self::is_relevant(document))
        {
            let project = workspace.project(document);
            super::citations::detect_undefined_citations(&project, document, &mut results);
            super::citations::detect_unused_entries(&project, document, &mut results);
        }

        super::citations::detect_duplicate_entries(workspace, &mut results);
        super::labels::detect_duplicate_labels(workspace, &mut results);
        super::labels::detect_undefined_and_unused_labels(workspace, &mut results);

        let config = &workspace.config().diagnostics;

        results.retain(|uri, _| {
            workspace
                .lookup(uri)
                .map_or(false, |document| Self::is_relevant(document))
        });

        for (_, diagnostics) in &mut results {
            diagnostics.retain(|diagnostic| {
                filter_regex_patterns(
                    &diagnostic.message(),
                    &config.allowed_patterns,
                    &config.ignored_patterns,
                )
            });
        }

        results
    }

    fn is_relevant(document: &Document) -> bool {
        match document.owner {
            Owner::Client => true,
            Owner::Server => true,
            Owner::Distro => false,
        }
    }
}
