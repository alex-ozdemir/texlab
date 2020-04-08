mod bibtex;
mod latex;

use self::{
    bibtex::{
        entry_type::BibtexEntryTypeHoverProvider, field::BibtexFieldHoverProvider,
        string_reference::BibtexStringReferenceHoverProvider,
    },
    latex::{
        citation::LatexCitationHoverProvider, component::LatexComponentHoverProvider,
        label::LatexLabelHoverProvider,
    },
};
use crate::{
    feature::{ChoiceProvider, FeatureProvider, FeatureRequest},
    protocol::{Hover, TextDocumentPositionParams},
};
use futures_boxed::boxed;

pub struct HoverProvider {
    provider: ChoiceProvider<TextDocumentPositionParams, Hover>,
}

impl HoverProvider {
    pub fn new() -> Self {
        Self {
            provider: ChoiceProvider::new(vec![
                Box::new(BibtexEntryTypeHoverProvider),
                Box::new(BibtexStringReferenceHoverProvider),
                Box::new(BibtexFieldHoverProvider),
                Box::new(LatexCitationHoverProvider),
                Box::new(LatexComponentHoverProvider),
                Box::new(LatexLabelHoverProvider),
            ]),
        }
    }
}

impl Default for HoverProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl FeatureProvider for HoverProvider {
    type Params = TextDocumentPositionParams;
    type Output = Option<Hover>;

    #[boxed]
    async fn execute<'a>(&'a self, req: &'a FeatureRequest<Self::Params>) -> Self::Output {
        self.provider.execute(req).await
    }
}
