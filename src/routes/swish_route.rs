use axum::Json;

pub async fn swish_callback(Json(json): Json<serde_json::Value>) {
    dbg!(json);
}
