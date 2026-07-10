# YOLO11 models

Place the exported YOLO11 TorchScript model here as `yolo11n.torchscript`.

Export with the same image size configured in `yolo_usb_detector.yaml`:

```bash
yolo export model=yolo11n.pt format=torchscript imgsz=320 optimize=False
```
