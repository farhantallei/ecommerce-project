use sqids::Sqids;

pub enum IdType {
  Public,
  Secret,
}

#[derive(Debug)]
pub struct SequenceConverter {
  sqids: Sqids,
}

impl SequenceConverter {
  pub fn new(id_type: IdType) -> Self {
    let alphabet_combination: String = match id_type {
      IdType::Secret => std::env::var("DB_SECRET_ID").expect("DB_SECRET_ID must be set"),
      IdType::Public => std::env::var("DB_PUBLIC_ID").expect("DB_PUBLIC_ID must be set")
    };

    SequenceConverter {
      sqids: Sqids::builder().min_length(4).alphabet(alphabet_combination.chars().collect()).build().unwrap()
    }
  }

  pub fn to_id(&self, sequence: i64) -> String {
    self.sqids.encode(&[sequence as u64]).unwrap()
  }

  pub fn from_id(&self, id: &str) -> Vec<u64> {
    self.sqids.decode(id)
  }
}
