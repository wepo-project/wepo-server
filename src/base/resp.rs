use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ResultResponse {
  succ: bool
}

impl ResultResponse {
  pub fn new(val: bool) -> Self {
    ResultResponse { succ: val }
  }
  
  pub fn succ() -> Self {
    ResultResponse::new(true)
  }

  pub fn fail() -> Self {
    ResultResponse::new(false)
  }
}