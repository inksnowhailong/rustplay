use ort::{Environment, Session, GraphOptimizationLevel, Value};
use wasm_bindgen::prelude::*;
use ndarray::{Array, Array4};
use serde::{Deserialize, Serialize};
use image::{DynamicImage, ImageBuffer, Rgb};
use js_sys::Uint8Array;

#[derive(Serialize, Deserialize)]
pub struct Scores {
    visual: i32,
    composition: i32,
    quality: i32,
}

#[wasm_bindgen]
pub struct OnnxModel {
    session: Session,
}

#[wasm_bindgen]
impl OnnxModel {
    #[wasm_bindgen]
    pub fn create() -> Result<OnnxModel, JsValue> {
        // 创建 ONNX Runtime 环境
        let env = Environment::builder()
            .with_name("onnx_env")
            .build()
            .map_err(|e| JsValue::from_str(&format!("创建环境失败: {:?}", e)))?;

        // 加载模型
        let model_bytes = include_bytes!("./picture_score_fp16.onnx");
        let session = Session::builder()
            .with_optimization_level(GraphOptimizationLevel::Basic)
            .map_err(|e| JsValue::from_str(&format!("创建会话失败: {:?}", e)))?
            .with_model_from_memory(model_bytes)
            .map_err(|e| JsValue::from_str(&format!("加载模型失败: {:?}", e)))?;

        Ok(OnnxModel { session })
    }

    #[wasm_bindgen]
    pub fn predict(&self, image_data: Uint8Array, width: u32, height: u32) -> Result<JsValue, JsValue> {
        // 将 Uint8Array 转换为 Rust 字节数组
        let image_data = image_data.to_vec();
        if image_data.len() != (width * height * 4) as usize {
            return Err(JsValue::from_str(&format!(
                "输入数据长度不正确，应为 {}，实际为 {}",
                width * height * 4,
                image_data.len()
            )));
        }

        // 将 RGBA 数据转换为 image 库的格式
        let img_buffer = ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(
            width,
            height,
            image_data.into_iter().enumerate()
                .filter(|(i, _)| i % 4 != 3) // 跳过 Alpha 通道
                .map(|(_, v)| v)
                .collect()
        ).ok_or_else(|| JsValue::from_str("无法创建图像缓冲区"))?;

        let img = DynamicImage::ImageRgb8(img_buffer);

        // 调整图片大小为         224x224
        let resized_img = img.resize_exact(224, 224, image::imageops::FilterType::Lanczos3);

        // 转换为 RGB 数据并归一化
        let rgb_img = resized_img.into_rgb8();
        let mut input = vec![0.0f16; 1 * 3 * 224 * 224];
        let mean = [0.485f32, 0.456f32, 0.406f32];
        let std = [0.229f32, 0.224f32, 0.225f32];

        for (i, pixel) in rgb_img.pixels().enumerate() {
            let channel = i % 3;
            let idx = i / 3 * 3 + channel;
            let normalized = (pixel[channel] as f32 / 255.0 - mean[channel]) / std[channel];
            input[idx] = f16::from_f32(normalized);
        }

        // 创建输入张量
        let input_array: Array4<f16> = Array::from_shape_vec((1, 3, 224, 224), input)
            .map_err(|e| JsValue::from_str(&format!("创建输入张量失败: {:?}", e)))?;

        // 转换为 ort 的 Value
        let input_value = Value::from_array(&input_array)
            .map_err(|e| JsValue::from_str(&format!("创建输入值失败: {:?}", e)))?;

        // 运行推理
        let outputs = self.session.run(&[input_value])
            .map_err(|e| JsValue::from_str(&format!("推理失败: {:?}", e)))?;

        // 获取输出
        let output = outputs[0].try_extract::<f16>()
            .map_err(|e| JsValue::from_str(&format!("提取输出失败: {:?}", e)))?;
        let scores = output.view();
        let scores = scores.as_slice().unwrap();

        // 转换为 Scores 结构体
        let scores = Scores {
            visual: (scores[0].to_f32() * 99.0 + 1.0).round() as i32,
            composition: (scores[1].to_f32() * 99.0 + 1.0).round() as i32,
            quality: (scores[2].to_f32() * 99.0 + 1.0).round() as i32,
        };

        serde_wasm_bindgen::to_value(&scores)
            .map_err(|e| JsValue::from_str(&format!("序列化失败: {:?}", e)))
    }
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
