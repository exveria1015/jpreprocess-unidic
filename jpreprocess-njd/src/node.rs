use std::fmt::Debug;

use crate::{node_details::NodeDetails, pos::*};

use super::accent_rule::ChainRules;

#[derive(Clone, PartialEq)]
pub struct NJDNode {
    string: String, //*は空文字列として扱う
    details: NodeDetails,
}

impl Debug for NJDNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{:?},{},{},{},{},{},{}/{},{},{}",
            self.string,
            self.details.pos,
            self.details.ctype,
            self.details.cform,
            self.details.orig,
            self.details.read.as_ref().unwrap_or(&"*".to_string()),
            self.details.pron.as_ref().unwrap_or(&"*".to_string()),
            self.details.acc,
            self.details.mora_size,
            self.details
                .chain_rule
                .as_ref()
                .map(|r| format!("{:?}", r))
                .unwrap_or("*".to_string()),
            match self.details.chain_flag {
                Some(true) => 1,
                Some(false) => 0,
                None => -1,
            }
        )
    }
}

impl NJDNode {
    pub fn new_single(s: &str) -> Self {
        let nodes = Self::load_str(s);
        if nodes.len() == 1 {
            nodes.into_iter().next().unwrap()
        } else {
            panic!("input string must contain exactly one node.");
        }
    }
    pub fn load_str(s: &str) -> Vec<Self> {
        let splited = {
            let mut splited: Vec<&str> = s.split(",").collect();
            splited.resize(13, "");
            splited
        };
        Self::load(splited[0], &splited[1..splited.len()])
    }
    pub fn load(string: &str, details: &[&str]) -> Vec<Self> {
        let details_vec = NodeDetails::load(details);
        let details_len = details_vec.len();
        details_vec
            .into_iter()
            .scan(0, |len, details| {
                *len += details.orig.len();
                Some((*len, details))
            })
            .enumerate()
            .map(|(i, (len, details))| Self {
                string: if i + 1 < details_len {
                    details.orig.to_string()
                } else {
                    string[len - details.orig.len()..string.len()].to_string()
                },
                details,
            })
            .collect()
    }

    pub fn transfer_from(&mut self, node: &mut Self) {
        self.string.push_str(&node.string);
        self.details.orig.push_str(&node.details.orig);
        self.add_mora_size(node.details.mora_size);
        if let Some(add) = &node.details.read {
            if let Some(read) = &mut self.details.read {
                read.push_str(add);
            } else {
                self.details.read = Some(add.to_string());
            }
        }
        if let Some(add) = &node.details.pron {
            if let Some(pron) = &mut self.details.pron {
                pron.push_str(add);
            } else {
                self.details.pron = Some(add.to_string());
            }
        }
        node.unset_pron();
    }

    pub fn get_chain_flag(&self) -> Option<bool> {
        self.details.chain_flag
    }
    pub fn set_chain_flag(&mut self, chain_flag: bool) {
        self.details.chain_flag = Some(chain_flag);
    }

    pub fn get_chain_rule(&self) -> Option<&ChainRules> {
        self.details.chain_rule.as_ref()
    }
    pub fn unset_chain_rule(&mut self) {
        self.details.chain_rule = None;
    }

    pub fn get_pos(&self) -> &PartOfSpeech {
        &self.details.pos
    }
    pub fn get_pos_mut(&mut self) -> &mut PartOfSpeech {
        &mut self.details.pos
    }

    pub fn is_renyou(&self) -> bool {
        self.details.cform.starts_with("連用")
    }

    pub fn get_string(&self) -> &str {
        self.string.as_str()
    }
    pub fn replace_string(&mut self, new_string: &str) {
        self.details.orig = new_string.to_string();
        self.string = new_string.to_string();
    }
    pub fn ensure_orig(&mut self) {
        if self.details.orig == "*" {
            self.details.orig = self.string.clone();
        }
    }

    pub fn get_read(&self) -> Option<&str> {
        self.details.read.as_ref().map(|read| read.as_str())
    }
    pub fn set_read(&mut self, read: &str) {
        self.details.read = Some(read.to_string());
    }
    pub fn unset_read(&mut self) {
        self.details.read = None;
    }

    pub fn get_acc(&self) -> i32 {
        self.details.acc
    }
    pub fn set_acc(&mut self, acc: i32) {
        self.details.acc = acc;
    }

    pub fn get_mora_size(&self) -> i32 {
        self.details.mora_size
    }
    pub fn set_mora_size(&mut self, mora_size: i32) {
        self.details.mora_size = mora_size;
    }
    pub fn add_mora_size(&mut self, mora_size: i32) {
        self.details.mora_size += mora_size;
        if self.details.mora_size < 0 {
            self.details.mora_size = 0;
        }
    }

    pub fn get_pron(&self) -> Option<&str> {
        self.details.pron.as_ref().map(|pron| pron.as_str())
    }
    pub fn set_pron(&mut self, pron: &str) {
        self.details.pron = Some(pron.to_string());
    }
    pub fn unset_pron(&mut self) {
        self.details.pron = None;
    }
}

#[cfg(test)]
mod tests {
    use crate::pos::*;

    use super::NJDNode;

    #[test]
    fn load_single_node() {
        let node = NJDNode::new_single("．,名詞,接尾,助数詞,*,*,*,．,テン,テン,0/2,*,-1");
        assert_eq!(node.string, "．");
        assert_eq!(node.get_pos().get_group0(), Group0::Meishi);
        assert_eq!(node.get_pos().get_group1(), Group1::Setsubi);
        assert_eq!(node.get_pos().get_group3(), Group3::Others);
        assert_eq!(node.details.ctype, "*");
        assert_eq!(node.details.cform, "*");
        assert_eq!(node.details.orig, "．");
        assert_eq!(node.details.read.unwrap(), "テン");
        assert_eq!(node.details.pron.unwrap(), "テン");
        assert_eq!(node.details.acc, 0);
        assert_eq!(node.details.mora_size, 2);
        assert_eq!(node.details.chain_rule.is_none(), true);
        assert_eq!(node.details.chain_flag, None);
    }

    #[test]
    fn load_multiple_nodes() {
        let nodes = NJDNode::load_str("あーあ,感動詞,*,*,*,*,*,あー:あ,アー:ア,アー:ア,1/2:1/1,C1");
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].string, "あー");
        assert_eq!(nodes[1].string, "あ");
        assert_eq!(nodes[0].details.orig, "あー");
        assert_eq!(nodes[1].details.orig, "あ");
        assert_eq!(nodes[0].details.acc, 1);
        assert_eq!(nodes[1].details.acc, 1);
        assert_eq!(nodes[0].details.mora_size, 2);
        assert_eq!(nodes[1].details.mora_size, 1);
    }
}
