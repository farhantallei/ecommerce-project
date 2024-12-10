use diesel::{sql_query, QueryResult, QueryableByName, RunQueryDsl};
use diesel::sql_types::BigInt;
use crate::infrastructure::db::connection::DBPool;

#[derive(QueryableByName)]
pub struct NextVal {
  #[diesel(sql_type = BigInt)]
  pub nextval: i64,
}

pub fn get_next_value(pool: DBPool, sequence_name: &str) -> QueryResult<i64> {
  let query = format!("SELECT nextval('{}')", sequence_name);
  let result: Vec<NextVal> = sql_query(query).load(&mut pool.get().unwrap())?;

  Ok(result[0].nextval)
}
