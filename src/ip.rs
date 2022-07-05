use crate::component::Component;

#[derive(Debug, Clone)]
pub enum IPType {
  Data,
  OpenBracket,
  CloseBracket,
}

#[derive(Debug, Clone)]
pub struct IP<T> {
  pub ip_type: IPType,
  pub data: T,
  pub is_ip: bool,
  pub owner: Option<&'static Component>,
}

impl<T> IP<T> {
  pub fn move_owner(&mut self, owner: &'static Component) {
    self.owner = Some(owner);
  }
}
