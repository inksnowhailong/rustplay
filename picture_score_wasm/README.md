# Picture Score WASM

这是一个将 ONNX 模型转换为 WebAssembly 的项目，用于在浏览器中运行图片评分模型。

## 使用方法

1. 确保已安装 Rust 和 wasm-pack
2. 将 `picture_score_fp16.onnx` 模型文件放在项目根目录
3. 运行以下命令构建 WASM：

```bash
wasm-pack build --target web
```

4. 在 JavaScript 中使用：

```javascript
import { PictureScoreModel } from './pkg/picture_score_wasm.js';

// 初始化模型
const model = await PictureScoreModel.new();

// 从 canvas 或图片元素获取图像数据
const canvas = document.createElement('canvas');
canvas.width = 224;
canvas.height = 224;
const ctx = canvas.getContext('2d');
ctx.drawImage(imageElement, 0, 0, 224, 224);
const imageData = ctx.getImageData(0, 0, 224, 224);

// 将图像数据转换为 Uint8Array
const uint8Array = new Uint8Array(imageData.data.buffer);

// 进行预测
const score = await model.predict(uint8Array, 224, 224);
console.log('预测分数:', score); // 输出 1-100 的评分
```

## 注意事项

- 确保 ONNX 模型文件 `picture_score_fp16.onnx` 存在于项目根目录
- 输入图像会被自动调整为 224x224 大小
- 模型输出为 1-100 的整数评分
- 图像数据需要是 RGBA 格式的 Uint8Array
