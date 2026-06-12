use rust_decimal::Decimal;

use crate::{
    dto::product::{CreateProductRequest, ProductListQuery, ProductResponse, UpdateProductRequest},
    errors::AppError,
    models::Product,
    repositories::ProductRepository,
};

pub struct ProductService {
    product_repo: ProductRepository,
}

impl ProductService {
    pub fn new(product_repo: ProductRepository) -> Self {
        Self { product_repo }
    }

    pub async fn list(
        &self,
        user_id: &str,
        query: ProductListQuery,
    ) -> Result<(Vec<ProductResponse>, i64, u32, u32), AppError> {
        let page = query.page.unwrap_or(1).max(1);
        let limit = query.limit.unwrap_or(10).min(100).max(1);

        let (products, total) = self
            .product_repo
            .find_all_by_user(user_id, page, limit, query.search.as_deref())
            .await?;

        let responses = products.into_iter().map(product_to_response).collect();

        Ok((responses, total, page, limit))
    }

    pub async fn get(&self, id: &str, user_id: &str) -> Result<ProductResponse, AppError> {
        let product = self
            .product_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

        if product.user_id != user_id {
            return Err(AppError::Forbidden(
                "You do not have access to this product".to_string(),
            ));
        }

        Ok(product_to_response(product))
    }

    pub async fn create(
        &self,
        user_id: &str,
        req: CreateProductRequest,
    ) -> Result<ProductResponse, AppError> {
        if req.price <= Decimal::ZERO {
            return Err(AppError::ValidationError(
                "Price must be greater than zero".to_string(),
            ));
        }

        if self
            .product_repo
            .name_exists_for_user(&req.name, user_id, None)
            .await?
        {
            return Err(AppError::Conflict(
                "A product with this name already exists".to_string(),
            ));
        }

        let stock = req.stock.unwrap_or(0).max(0);
        let id = Product::new_id();
        let product = self
            .product_repo
            .create(
                &id,
                user_id,
                &req.name,
                req.price,
                req.description.as_deref(),
                req.image_url.as_deref(),
                stock,
            )
            .await?;

        Ok(product_to_response(product))
    }

    pub async fn update(
        &self,
        id: &str,
        user_id: &str,
        req: UpdateProductRequest,
    ) -> Result<ProductResponse, AppError> {
        let product = self
            .product_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

        if product.user_id != user_id {
            return Err(AppError::Forbidden(
                "You do not have permission to update this product".to_string(),
            ));
        }

        if let Some(p) = req.price {
            if p <= Decimal::ZERO {
                return Err(AppError::ValidationError(
                    "Price must be greater than zero".to_string(),
                ));
            }
        }

        if let Some(name) = &req.name {
            if self
                .product_repo
                .name_exists_for_user(name, user_id, Some(id))
                .await?
            {
                return Err(AppError::Conflict(
                    "A product with this name already exists".to_string(),
                ));
            }
        }

        let updated = self
            .product_repo
            .update(
                id,
                req.name.as_deref(),
                req.price,
                req.description.as_ref().map(|d| Some(d.as_str())),
                req.image_url.as_ref().map(|u| Some(u.as_str())),
                req.stock,
            )
            .await?;

        Ok(product_to_response(updated))
    }

    pub async fn delete(&self, id: &str, user_id: &str) -> Result<(), AppError> {
        let product = self
            .product_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Product not found".to_string()))?;

        if product.user_id != user_id {
            return Err(AppError::Forbidden(
                "You do not have permission to delete this product".to_string(),
            ));
        }

        self.product_repo.soft_delete(id).await?;

        Ok(())
    }
}

pub fn product_to_response(p: Product) -> ProductResponse {
    ProductResponse {
        id: p.id,
        name: p.name,
        price: p.price,
        description: p.description,
        image_url: p.image_url,
        stock: p.stock,
        created_at: p.created_at,
    }
}
