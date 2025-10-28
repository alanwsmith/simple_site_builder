#![allow(warnings)]
use super::FileDetails;
use minijinja::Value;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum DataNode {
  Branch(HashMap<String, DataNode>),
  Leaf(Value),
}

impl DataNode {
  pub fn new() -> Self {
    DataNode::Branch(HashMap::new())
  }

  fn get(
    &self,
    path: &[String],
  ) -> Option<&Value> {
    match self {
      DataNode::Branch(nodes) => {
        if path.is_empty() {
          return None;
        }
        let remainder = &path[1..];
        nodes.get(&path[0]).and_then(|next_node| {
          if remainder.is_empty() {
            match next_node {
              DataNode::Leaf(value) => Some(value),
              _ => None,
            }
          } else {
            next_node.get(remainder)
          }
        })
      }
      _ => None,
    }
  }

  pub fn insert(
    &mut self,
    path: &[String],
    value: Value,
  ) {
    if path.is_empty() {
      return;
    }
    if let DataNode::Branch(nodes) = self {
      let remainder = &path[1..];
      let next_node = nodes
        .entry(path[0].clone())
        .or_insert(if remainder.is_empty() {
          DataNode::Leaf(value.clone())
        } else {
          DataNode::Branch(HashMap::new())
        });
      next_node.insert(remainder, value);
    }
  }
}
