use diesel::result::Error;
use crate::application::dto::product_count_options::ProductCountAvailable;
use crate::domain::entities::product::NewProduct;
use crate::domain::repositories::product_repository::ProductRepository;
use crate::infrastructure::lib::sequence_converter::{IdType, SequenceConverter};

pub struct ProductService<T> {
  product_repo: T,
}

impl<T: ProductRepository> ProductService<T> {
  pub fn new(product_repo: T) -> Self {
    ProductService {
      product_repo,
    }
  }

  pub async fn create_id(&self) -> Result<(String, String), Error> {
    let sequence = self.product_repo.next_val().await?;
    Ok((
      SequenceConverter::new(IdType::Secret).to_id(sequence),
      SequenceConverter::new(IdType::Public).to_id(sequence)
    ))
  }

  pub async fn get_product_count(&self, option: ProductCountAvailable) -> Result<i64, Error> {
    self.product_repo.count(option).await
  }

  pub async fn create(&self, new_product: &NewProduct) -> Result<(), Error> {
    self.product_repo.save(new_product).await
  }

  pub async fn delete_all(&self) -> Result<(), Error> {
    self.product_repo.delete_all().await
  }
}
