use crate::feature::FeatureRequest;
use crate::syntax::bibtex::BibtexDeclaration;
use crate::syntax::text::SyntaxNode;
use crate::syntax::SyntaxTree;
use lsp_types::{FoldingRange, FoldingRangeKind, FoldingRangeParams};

pub struct BibtexDeclarationFoldingProvider;

impl BibtexDeclarationFoldingProvider {
    pub async fn execute(request: &FeatureRequest<FoldingRangeParams>) -> Vec<FoldingRange> {
        if let SyntaxTree::Bibtex(tree) = &request.document.tree {
            tree.root.children.iter().flat_map(Self::fold).collect()
        } else {
            Vec::new()
        }
    }

    fn fold(declaration: &BibtexDeclaration) -> Option<FoldingRange> {
        let ty = match declaration {
            BibtexDeclaration::Comment(_) => None,
            BibtexDeclaration::Preamble(preamble) => Some(&preamble.ty),
            BibtexDeclaration::String(string) => Some(&string.ty),
            BibtexDeclaration::Entry(entry) => Some(&entry.ty),
        }?;

        let right = match declaration {
            BibtexDeclaration::Comment(_) => None,
            BibtexDeclaration::Preamble(preamble) => preamble.right.as_ref(),
            BibtexDeclaration::String(string) => string.right.as_ref(),
            BibtexDeclaration::Entry(entry) => entry.right.as_ref(),
        }?;

        Some(FoldingRange {
            start_line: ty.range().start.line,
            start_character: Some(ty.range().start.character),
            end_line: right.range().start.line,
            end_character: Some(right.range().start.character),
            kind: Some(FoldingRangeKind::Region),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::completion::LatexComponentDatabase;
    use crate::feature::FeatureSpec;
    use crate::test_feature;
    use lsp_types::Position;

    #[test]
    fn test_preamble() {
        let foldings = test_feature!(
            BibtexDeclarationFoldingProvider,
            FeatureSpec {
                files: vec![FeatureSpec::file("foo.bib", "\n@preamble{\"foo\"}")],
                main_file: "foo.bib",
                position: Position::default(),
                new_name: "",
                component_database: LatexComponentDatabase::default(),
            }
        );
        assert_eq!(
            foldings,
            vec![FoldingRange {
                start_line: 1,
                start_character: Some(0),
                end_line: 1,
                end_character: Some(15),
                kind: Some(FoldingRangeKind::Region),
            }]
        );
    }

    #[test]
    fn test_string() {
        let foldings = test_feature!(
            BibtexDeclarationFoldingProvider,
            FeatureSpec {
                files: vec![FeatureSpec::file("foo.bib", "@string{foo = \"bar\"}")],
                main_file: "foo.bib",
                position: Position::default(),
                new_name: "",
                component_database: LatexComponentDatabase::default(),
            }
        );
        assert_eq!(
            foldings,
            vec![FoldingRange {
                start_line: 0,
                start_character: Some(0),
                end_line: 0,
                end_character: Some(19),
                kind: Some(FoldingRangeKind::Region),
            }]
        );
    }

    #[test]
    fn test_entry() {
        let foldings = test_feature!(
            BibtexDeclarationFoldingProvider,
            FeatureSpec {
                files: vec![FeatureSpec::file("foo.bib", "@article{foo, bar = baz\n}")],
                main_file: "foo.bib",
                position: Position::default(),
                new_name: "",
                component_database: LatexComponentDatabase::default(),
            }
        );
        assert_eq!(
            foldings,
            vec![FoldingRange {
                start_line: 0,
                start_character: Some(0),
                end_line: 1,
                end_character: Some(0),
                kind: Some(FoldingRangeKind::Region),
            }]
        );
    }

    #[test]
    fn test_comment() {
        let foldings = test_feature!(
            BibtexDeclarationFoldingProvider,
            FeatureSpec {
                files: vec![FeatureSpec::file("foo.bib", "foo")],
                main_file: "foo.bib",
                position: Position::default(),
                new_name: "",
                component_database: LatexComponentDatabase::default(),
            }
        );
        assert_eq!(foldings, Vec::new());
    }

    #[test]
    fn test_entry_invalid() {
        let foldings = test_feature!(
            BibtexDeclarationFoldingProvider,
            FeatureSpec {
                files: vec![FeatureSpec::file("foo.bib", "@article{foo,")],
                main_file: "foo.bib",
                position: Position::default(),
                new_name: "",
                component_database: LatexComponentDatabase::default(),
            }
        );
        assert_eq!(foldings, Vec::new());
    }

    #[test]
    fn test_latex() {
        let foldings = test_feature!(
            BibtexDeclarationFoldingProvider,
            FeatureSpec {
                files: vec![FeatureSpec::file("foo.tex", ""),],
                main_file: "foo.tex",
                position: Position::default(),
                new_name: "",
                component_database: LatexComponentDatabase::default(),
            }
        );
        assert_eq!(foldings, Vec::new());
    }
}
