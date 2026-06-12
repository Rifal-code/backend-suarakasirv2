use rust_decimal::Decimal;

use crate::{
    dto::order::{
        CreateOrderRequest, OrderItemResponse, OrderListQuery, OrderResponse, UpdateOrderRequest,
    },
    errors::AppError,
    models::{Order, OrderItem},
    repositories::{OrderRepository, ProductRepository},
};

pub struct OrderService {
    order_repo: OrderRepository,
    product_repo: ProductRepository,
}

impl OrderService {
    pub fn new(order_repo: OrderRepository, product_repo: ProductRepository) -> Self {
        Self {
            order_repo,
            product_repo,
        }
    }

    pub async fn list(
        &self,
        user_id: &str,
        query: OrderListQuery,
    ) -> Result<(Vec<OrderResponse>, i64, u32, u32), AppError> {
        let page = query.page.unwrap_or(1).max(1);
        let limit = query.limit.unwrap_or(10).min(100).max(1);

        let (orders, total) = self
            .order_repo
            .find_all_by_user(
                user_id,
                page,
                limit,
                query.status,
                query.start_date,
                query.end_date,
            )
            .await?;

        let mut responses = Vec::with_capacity(orders.len());
        for order in orders {
            let items = self.order_repo.find_items_by_order(&order.id).await?;
            responses.push(order_to_response(order, items));
        }

        Ok((responses, total, page, limit))
    }

    pub async fn get(&self, id: &str, user_id: &str) -> Result<OrderResponse, AppError> {
        let order = self
            .order_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Order not found".to_string()))?;

        if order.user_id != user_id {
            return Err(AppError::Forbidden(
                "You do not have access to this order".to_string(),
            ));
        }

        let items = self.order_repo.find_items_by_order(&order.id).await?;

        Ok(order_to_response(order, items))
    }

    pub async fn create(
        &self,
        user_id: &str,
        req: CreateOrderRequest,
    ) -> Result<OrderResponse, AppError> {
        if req.items.is_empty() {
            return Err(AppError::ValidationError(
                "Order must have at least one item".to_string(),
            ));
        }

        // 1. Resolve products, validate stock, calculate totals
        let mut calculated_items: Vec<(String, String, i32, Decimal, Decimal)> = Vec::new();
        let mut total_amount = Decimal::ZERO;

        for item in &req.items {
            if item.quantity <= 0 {
                return Err(AppError::ValidationError(
                    "Quantity must be greater than 0".to_string(),
                ));
            }

            let product = self
                .product_repo
                .find_by_id(&item.product_id)
                .await?
                .ok_or_else(|| {
                    AppError::NotFound(format!("Product '{}' not found", item.product_id))
                })?;

            // Validate stock availability
            if product.stock < item.quantity {
                return Err(AppError::ValidationError(format!(
                    "Insufficient stock for '{}'. Available: {}, requested: {}",
                    product.name, product.stock, item.quantity
                )));
            }

            let unit_price = product.price;
            let qty = Decimal::from(item.quantity);
            let subtotal = unit_price * qty;
            total_amount += subtotal;

            calculated_items.push((
                OrderItem::new_id(),
                item.product_id.clone(),
                item.quantity,
                unit_price,
                subtotal,
            ));
        }

        // 2. Persist the order
        let order_id = Order::new_id();
        let order = self
            .order_repo
            .create(&order_id, user_id, total_amount, &calculated_items)
            .await?;

        // 3. Decrement stock for each item (after successful order creation)
        for (_, product_id, qty, _, _) in &calculated_items {
            self.product_repo.decrement_stock(product_id, *qty).await?;
        }

        let items = self.order_repo.find_items_by_order(&order.id).await?;

        Ok(order_to_response(order, items))
    }

    pub async fn update(
        &self,
        id: &str,
        user_id: &str,
        req: UpdateOrderRequest,
    ) -> Result<OrderResponse, AppError> {
        let order = self
            .order_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Order not found".to_string()))?;

        if order.user_id != user_id {
            return Err(AppError::Forbidden(
                "You do not have permission to update this order".to_string(),
            ));
        }

        if req.items.is_empty() {
            return Err(AppError::ValidationError(
                "Order must have at least one item".to_string(),
            ));
        }

        // Recalculate with server-side prices (no additional stock check on update)
        let mut calculated_items: Vec<(String, String, i32, Decimal, Decimal)> = Vec::new();
        let mut total_amount = Decimal::ZERO;

        for item in &req.items {
            if item.quantity <= 0 {
                return Err(AppError::ValidationError(
                    "Quantity must be greater than 0".to_string(),
                ));
            }

            let product = self
                .product_repo
                .find_by_id(&item.product_id)
                .await?
                .ok_or_else(|| {
                    AppError::NotFound(format!("Product '{}' not found", item.product_id))
                })?;

            let unit_price = product.price;
            let qty = Decimal::from(item.quantity);
            let subtotal = unit_price * qty;
            total_amount += subtotal;

            calculated_items.push((
                OrderItem::new_id(),
                item.product_id.clone(),
                item.quantity,
                unit_price,
                subtotal,
            ));
        }

        let updated = self
            .order_repo
            .update(id, total_amount, req.status, &calculated_items)
            .await?;

        let items = self.order_repo.find_items_by_order(&updated.id).await?;

        Ok(order_to_response(updated, items))
    }

    pub async fn delete(&self, id: &str, user_id: &str) -> Result<(), AppError> {
        let order = self
            .order_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Order not found".to_string()))?;

        if order.user_id != user_id {
            return Err(AppError::Forbidden(
                "You do not have permission to delete this order".to_string(),
            ));
        }

        self.order_repo.soft_delete(id).await?;

        Ok(())
    }
}

pub fn order_to_response(order: Order, items: Vec<OrderItemResponse>) -> OrderResponse {
    OrderResponse {
        id: order.id,
        total_amount: order.total_amount,
        status: order.status,
        items,
        created_at: order.created_at,
    }
}
