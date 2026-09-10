# Image Conversion Service

A image conversion and resizing service built with [Spin](https://www.spinframework.dev) and compiled to WebAssembly. Convert between multiple image formats and resize images on-the-fly with a simple HTTP API.

## Features

- **Multi-format Support**: Convert between PNG, JPEG, WebP, GIF, BMP, ICO, and TIFF
- **Dynamic Resizing**: Resize images by width, height, or exact dimensions
- **Quality Control**: 
  - JPEG: Adjust quality (1-100)
  - PNG: Control compression level (1-100, mapped to levels 1-9)
  - WebP: Support for both lossy and lossless compression with explicit control via `lossless` parameter
- **Auto-detection**: Automatically detects input image format

## Building and Running

1. Build the application:
```bash
spin build
```

2. Run the service:
```bash
spin up
```

The service will be available at `http://localhost:3000`

## Demo UI

Open `http://localhost:3000` in a browser for an interactive demo: pick (or drag &
drop) an image, choose the target format, size, and quality, and the conversion runs
on the edge Function. The original and converted images are shown side by side with
timing and output size, and the result can be downloaded directly.

The UI simply `POST`s the selected file to the `/convert` endpoint documented below.

## Usage

Send a POST request to `/convert` with the image data in the request body and specify
conversion parameters via query parameters.

### Basic Conversion

Convert JPEG to PNG (default):
```bash
curl -X POST --data-binary @input.jpg http://localhost:3000/convert -o output.png
```

### Format Conversion

Convert to JPEG with quality setting:
```bash
curl -X POST --data-binary @input.png \
  "http://localhost:3000/convert?format=jpeg&quality=85" \
  -o output.jpg
```

Convert to WebP with quality control:
```bash
curl -X POST --data-binary @input.jpg \
  "http://localhost:3000/convert?format=webp&quality=90" \
  -o output.webp
```

Convert to WebP lossless:
```bash
curl -X POST --data-binary @input.jpg \
  "http://localhost:3000/convert?format=webp&lossless=true" \
  -o output.webp
```

Convert to PNG with compression:
```bash
curl -X POST --data-binary @input.jpg \
  "http://localhost:3000/convert?format=png&quality=85" \
  -o output.png
```

### Resizing

Resize by width (maintains aspect ratio):
```bash
curl -X POST --data-binary @input.jpg \
  "http://localhost:3000/convert?width=800" \
  -o output.png
```

Resize by height (maintains aspect ratio):
```bash
curl -X POST --data-binary @input.jpg \
  "http://localhost:3000/convert?height=600" \
  -o output.png
```

Resize to exact dimensions:
```bash
curl -X POST --data-binary @input.jpg \
  "http://localhost:3000/convert?width=1024&height=768" \
  -o output.png
```

### Combined Operations

Resize and convert with quality control:
```bash
curl -X POST --data-binary @input.png \
  "http://localhost:3000/convert?format=jpeg&width=640&quality=90" \
  -o output.jpg
```

## API Reference

### Endpoint

```
POST /convert
```

### Query Parameters

| Parameter | Type | Description | Default |
|-----------|------|-------------|---------|
| `format` | string | Output format: `png`, `jpeg`, `webp`, `gif`, `bmp`, `ico`, `tiff` | `png` |
| `width` | integer | Target width in pixels (maintains aspect ratio if height not set) | - |
| `height` | integer | Target height in pixels (maintains aspect ratio if width not set) | - |
| `quality` | integer | Output quality (1-100):<br>• JPEG: Quality level<br>• PNG: Compression level (1-100 mapped to 1-9)<br>• WebP: Lossy quality when `lossless=false` (ignored when `lossless=true`) | `90` |
| `lossless` | boolean | WebP only: Use lossless compression (`true`/`false`). When enabled, `quality` parameter is ignored. | `false` |

### Request Body

Binary image data in any supported format (JPEG, PNG, GIF, BMP, TIFF, WebP, etc.)

### Response

**Success (200 OK)**
- Body: Converted image binary data
- Headers:
  - `Content-Type`: Appropriate MIME type for the output format

**Error (4xx/5xx)**
- Body: JSON error message
```json
{
  "error": "Error description"
}
```

## Testing

The sample includes a test suite that validates all conversion operations, resizing functionality, and error handling.

### Run Tests

```bash
./test.sh
```

### Test Requirements

- `input.jpg` file in the project directory (download a test image or use your own)
- `curl` command-line tool
- `file` command for format verification
