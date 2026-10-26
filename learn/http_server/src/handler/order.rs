use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use crate::handler::common::{AppState, Order};

#[derive(Debug, Deserialize)]
pub struct ListOrdersParam {
    limit: usize,
    offset: usize,
}

#[derive(Debug, Serialize)]
struct ListOrdersResponseBody<'a> {
    error_message: &'static str,
    orders: Vec<&'a Order>,
}

pub async fn get_order(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(param): Query<ListOrdersParam>,
) -> Response {
    let orders = match state.order.read() {
        Ok(orders) => orders,
        Err(err) => {
            tracing::error!(
                error = %err,
                "get_order failed, read orders list error"
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ListOrdersResponseBody {
                    error_message: "读取订单数据失败",
                    orders: vec![],
                }),
            ).into_response();
        },
    };
    tracing::info!(
        id = %id,
        limit = param.limit,
        offset = param.offset,
        "get_order param"
    );
    match orders.get(&id) {
        None => {
            tracing::warn!("order not found");
            (
                StatusCode::NOT_FOUND,
                Json(ListOrdersResponseBody {
                    error_message: "order not found",
                    orders: vec![],
                })
            ).into_response()
        }
        Some(order) => {
            tracing::info!("list_orders order success, id:{}, order:{:?}", id, order);
            (
                StatusCode::OK,
                Json(ListOrdersResponseBody {
                    error_message: "",
                    orders: vec![order],
                })
            ).into_response()
        }
    }
}

pub async fn get_order_list(
    State(state): State<AppState>,
    Query(param): Query<ListOrdersParam>,
) -> Response {
    let orders = match state.order.read() {
        Ok(orders) => orders,
        Err(err) => {
            tracing::error!(
                error = %err,
                "list_orders failed, read orders list error"
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ListOrdersResponseBody {
                    error_message: "读取订单数据失败",
                    orders: vec![],
                }),
            ).into_response();
        },
    };
    tracing::info!(
        limit = param.limit,
        offset = param.offset,
        "list_orders param"
    );

    let mut order_list: Vec<Order> = orders
        .values()
        .cloned()
        .collect();
    order_list.sort_by_key(|u| u.id);

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "total": order_list.len(),
            "items": order_list,
        }))
    ).into_response()
}

// async fn get_order_by_id()