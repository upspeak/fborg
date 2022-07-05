use crate::component::Component;

pub struct BasePort {
  name: Option<String>,
  options: BasePortOptions,
  node: Option<String>,
  node_instance: Option<Component>,
}

impl BasePort {
  pub fn id(&self) -> String {
    match &self.node {
      Some(node) => match &self.name {
        Some(name) => format!("{} {}", node, name.to_uppercase()),
        None => "Port".to_string(),
      },
      None => "Port".to_string(),
    }
  }
}

pub struct BasePortOptions {}
