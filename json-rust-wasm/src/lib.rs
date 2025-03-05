use serde::Serialize; // 导入 Serialize trait
use serde_json::{from_str, to_string, Value};
use serde_wasm_bindgen::Serializer;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
// 反序列化：JSON 字符串 -> JsValue（通用的 JS 对象）
#[wasm_bindgen]
pub fn json_parse(json_str: &str) -> Result<JsValue, JsValue> {
    // 将 JSON 字符串解析为 serde_json::Value
    let value: Value = from_str(json_str).map_err(|e| JsValue::from_str(&e.to_string()))?;
    // 将 Value 转换为 JsValue
    let serializer = Serializer::new().serialize_maps_as_objects(true);
    value
        .serialize(&serializer)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

// 序列化：JsValue（任意 JS 数据） -> JSON 字符串
#[wasm_bindgen]
pub fn json_stringify(js_value: JsValue) -> Result<String, JsValue> {
    // 将 JsValue 转换为 serde_json::Value
    let value: Value = from_value(js_value).map_err(|e| JsValue::from_str(&e.to_string()))?;
    // 将 Value 序列化为 JSON 字符串
    to_string(&value).map_err(|e| JsValue::from_str(&e.to_string()))
}
