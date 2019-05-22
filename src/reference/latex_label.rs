use crate::feature::FeatureRequest;
use crate::syntax::latex::LatexLabelKind;
use crate::syntax::SyntaxTree;
use lsp_types::{Location, ReferenceParams};

pub struct LatexLabelReferenceProvider;

impl LatexLabelReferenceProvider {
    pub async fn execute(request: &FeatureRequest<ReferenceParams>) -> Vec<Location> {
        let mut references = Vec::new();
        if let Some(definition) = Self::find_definition(request) {
            for document in &request.related_documents {
                if let SyntaxTree::Latex(tree) = &document.tree {
                    tree.labels
                        .iter()
                        .filter(|label| label.kind() == LatexLabelKind::Reference)
                        .filter(|label| label.name().text() == definition)
                        .map(|label| Location::new(document.uri.clone(), label.command.range))
                        .for_each(|location| references.push(location))
                }
            }
        }
        references
    }

    fn find_definition(request: &FeatureRequest<ReferenceParams>) -> Option<&str> {
        if let SyntaxTree::Latex(tree) = &request.document.tree {
            tree.labels
                .iter()
                .find(|label| {
                    label.kind() == LatexLabelKind::Definition
                        && label.command.range.contains(request.params.position)
                })
                .map(|label| label.name().text())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::FeatureSpec;
    use crate::test_feature;
    use lsp_types::{Position, Range};

    #[test]
    fn test() {
        let references = test_feature!(
            LatexLabelReferenceProvider,
            FeatureSpec {
                files: vec![
                    FeatureSpec::file("foo.tex", "\\label{foo}"),
                    FeatureSpec::file("bar.tex", "\\input{foo.tex}\n\\ref{foo}"),
                    FeatureSpec::file("baz.tex", "\\ref{foo}")
                ],
                main_file: "foo.tex",
                position: Position::new(0, 8),
                ..FeatureSpec::default()
            }
        );
        assert_eq!(
            references,
            vec![Location::new(
                FeatureSpec::uri("bar.tex"),
                Range::new_simple(1, 0, 1, 9)
            )]
        );
    }

    #[test]
    fn test_bibtex() {
        let references = test_feature!(
            LatexLabelReferenceProvider,
            FeatureSpec {
                files: vec![FeatureSpec::file("foo.bib", ""),],
                main_file: "foo.bib",
                position: Position::new(0, 0),
                ..FeatureSpec::default()
            }
        );
        assert_eq!(references, Vec::new());
    }
}
