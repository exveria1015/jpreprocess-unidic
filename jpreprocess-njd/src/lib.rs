pub mod accent_rule;
pub mod dictionary;
pub mod node;
pub mod node_details;
pub mod pos;
mod unk;

use dictionary::JPreprocessDict;
use jpreprocess_core::{error::JPreprocessErrorKind, JPreprocessResult};
use jpreprocess_dictionary::DictionaryTrait;
use lindera::Token;
pub use node::*;
use node_details::NodeDetails;

#[derive(Debug)]
pub struct NJD {
    pub nodes: Vec<NJDNode>,
}

impl NJD {
    pub fn remove_silent_node(&mut self) {
        self.nodes.retain(|node| node.get_pron().is_some())
    }
    pub fn from_tokens_string(tokens: Vec<Token>) -> Self {
        let mut nodes = Vec::new();
        for mut token in tokens {
            let text = token.get_text().to_string();
            let mut details_str = token.get_details().unwrap();
            let details = if details_str.len() == 1 && details_str[0] == "UNK" {
                vec![unk::UNK.to_owned()]
            } else {
                details_str.resize(13, "");
                NodeDetails::load(&details_str)
            };
            nodes.extend(NJDNode::load(&text, details));
        }
        Self { nodes }
    }
    pub fn from_tokens_dict(tokens: Vec<Token>, dict: JPreprocessDict) -> JPreprocessResult<Self> {
        let mut nodes = Vec::new();
        for token in tokens {
            let text = token.get_text().to_string();
            let details = if !token.word_id.is_unknown() {
                let id =
                    token.word_id.0.try_into().map_err(|e| {
                        JPreprocessErrorKind::DictionaryIndexOutOfRange.with_error(e)
                    })?;
                dict.get(id)
            } else {
                None
            }
            .unwrap_or_else(|| vec![unk::UNK.to_owned()]);

            nodes.extend(NJDNode::load(&text, details));
        }
        Ok(Self { nodes })
    }
}
