use crate::http::http_common::{HttpErrorBody, HttpState};
use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
};
use sim_utils::sim::utils::database::project_with_param::ProjectWithParamManager;
use tower_http::request_id::RequestId;

pub async fn project_list_get(
    State(state): State<HttpState>,
    Path(_): Path<i64>,
    Extension(request_id): Extension<RequestId>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<HttpErrorBody>)> {
    let trace_id = request_id
        .header_value()
        .to_str()
        .unwrap_or("unknown")
        .to_owned();
    let project_list = match ProjectWithParamManager::select_all(state.pg_pool())
        .await
    {
        Ok(project_list) => project_list,
        Err(err) => {
            tracing::error!(
                error = %err,
                "failed to fetch project with param list"
            );
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(HttpErrorBody {
                    error_message: "Internal server error".to_owned(),
                    trace_id,
                }),
            ));
        }
    };

    Ok(Json(serde_json::json!({
        "project_list": project_list,
        "trace_id": trace_id,
    })))
}

pub async fn project_param_default_get(
    Extension(request_id): Extension<RequestId>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<HttpErrorBody>)> {
    let trace_id = request_id
        .header_value()
        .to_str()
        .unwrap_or("unknown")
        .to_owned();
    Ok(Json(serde_json::json!(
{
    "param": [
        {
            "desc": "场景名称",
            "name": "scenario_name",
            "range": [],
            "type": "string",
            "unit": "",
            "value": "新建场景"
        },
        {
            "desc": "场景类型",
            "name": "scenario_type",
            "range": [
                "HIL",
                "SIL",
                "CLOUD"
            ],
            "type": "enum",
            "unit": "",
            "value": "HIL"
        },
        {
            "desc": "天气配置",
            "name": "weather",
            "range": [
                "晴",
                "雨",
                "雪"
            ],
            "type": "enum",
            "unit": "",
            "value": "晴"
        }
    ],
    "trace_id": trace_id,
})))
}

pub async fn project_car_param_default_get(
    Extension(request_id): Extension<RequestId>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<HttpErrorBody>)> {
    let trace_id = request_id
        .header_value()
        .to_str()
        .unwrap_or("unknown")
        .to_owned();
    Ok(Json(serde_json::json!(
{
    "carparam": [
        {
            "id": 0,
            "param": [
                {
                    "desc": "仿真车标签",
                    "name": "tag",
                    "range": [],
                    "type": "string",
                    "unit": "",
                    "value": "01"
                },
                {
                    "desc": "车辆类型",
                    "name": "vehicle_type",
                    "range": [
                        "三一",
                        "同力",
                        "长安",
                        "AGV"
                    ],
                    "type": "enum",
                    "unit": "",
                    "value": "三一"
                },
                {
                    "desc": "域控名称",
                    "name": "controller",
                    "range": [
                        "default"
                    ],
                    "type": "enum",
                    "unit": "",
                    "value": "default"
                },
                {
                    "desc": "动力学模型",
                    "name": "vehicle_dynamics",
                    "range": [
                        "tx_car_ice"
                    ],
                    "type": "enum",
                    "unit": "",
                    "value": "tx_car_ice"
                },
                {
                    "desc": "矿卡imei",
                    "name": "car_imei",
                    "range": [],
                    "type": "string",
                    "unit": "",
                    "value": ""
                },
                {
                    "desc": "矿卡id",
                    "name": "car_id",
                    "range": [],
                    "type": "string",
                    "unit": "",
                    "value": ""
                },
                {
                    "desc": "调度类型",
                    "name": "task_type",
                    "range": [
                        "0",
                        "1"
                    ],
                    "type": "enum",
                    "unit": "",
                    "value": "0"
                },
                {
                    "desc": "挖机id",
                    "name": "shovle_id",
                    "range": [],
                    "type": "string",
                    "unit": "",
                    "value": ""
                },
                {
                    "desc": "物料编码",
                    "name": "material_code",
                    "range": [],
                    "type": "string",
                    "unit": "",
                    "value": ""
                },
                {
                    "desc": "卸载点",
                    "name": "unload_area_id",
                    "range": [],
                    "type": "string",
                    "unit": "",
                    "value": ""
                },
                {
                    "desc": "初始位置经度",
                    "name": "initial_lon",
                    "range": "[-180,180]",
                    "type": "double",
                    "unit": "角度",
                    "value": 0.0
                },
                {
                    "desc": "初始位置纬度",
                    "name": "initial_lat",
                    "range": "[-85,85]",
                    "type": "double",
                    "unit": "角度",
                    "value": 0.0
                },
                {
                    "desc": "初始位置海拔高度",
                    "name": "initial_alt",
                    "range": [],
                    "type": "double",
                    "unit": "米",
                    "value": 0.0
                },
                {
                    "desc": "初始航向角",
                    "name": "initial_yaw",
                    "range": "[0,360)",
                    "type": "double",
                    "unit": "角度",
                    "value": 0.0
                },
                {
                    "desc": "初始俯仰角",
                    "name": "initial_pitch",
                    "range": "[-90,90]",
                    "type": "double",
                    "unit": "角度",
                    "value": 0.0
                },
                {
                    "desc": "初始横滚角",
                    "name": "initial_roll",
                    "range": "[-180,180]",
                    "type": "double",
                    "unit": "角度",
                    "value": 0.0
                },
                {
                    "desc": "初始速度",
                    "name": "initial_speed",
                    "range": "[0,40]",
                    "type": "double",
                    "unit": "km/h",
                    "value": 0.0
                },
                {
                    "desc": "挡位N->R延迟",
                    "name": "nr_delay",
                    "range": "[0, 10000]",
                    "type": "double",
                    "unit": "ms",
                    "value": 500
                },
                {
                    "desc": "挡位N->D延迟",
                    "name": "nd_delay",
                    "range": "[0, 10000]",
                    "type": "double",
                    "unit": "ms",
                    "value": 500
                },
                {
                    "desc": "挡位R->N延迟",
                    "name": "rn_delay",
                    "range": "[0, 10000]",
                    "type": "double",
                    "unit": "ms",
                    "value": 500
                },
                {
                    "desc": "挡位D->N延迟",
                    "name": "dn_delay",
                    "range": "[0, 10000]",
                    "type": "double",
                    "unit": "ms",
                    "value": 500
                },
                {
                    "desc": "驻车解除->拉起手刹延迟",
                    "name": "park_01_delay",
                    "range": "[0, 10000]",
                    "type": "double",
                    "unit": "ms",
                    "value": 500
                },
                {
                    "desc": "拉起手刹->驻车解除延迟",
                    "name": "park_10_delay",
                    "range": "[0, 10000]",
                    "type": "double",
                    "unit": "ms",
                    "value": 500
                },
                {
                    "desc": "感知配置",
                    "name": "sensor",
                    "range": [],
                    "type": "enum",
                    "unit": "",
                    "value": ""
                }
            ]
        }
    ],
    "trace_id": trace_id,
}
    )))
}
