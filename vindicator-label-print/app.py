from fastapi import FastAPI, HTTPException
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from PIL import Image, ImageDraw
import qrcode
from brother_ql.conversion import convert
from brother_ql.backends.helpers import send
from brother_ql.raster import BrotherQLRaster

app = FastAPI()

# CORS settings to allow requests from your Nuxt app
app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:3000"],  # Change to your frontend URL
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

PRINTER_MODEL = 'QL-820NWB'
PRINTER_IP = '192.168.40.221'
PRINTER_PORT = 9100
LABEL_TYPE = '12'

class PrintRequest(BaseModel):
    text: str

def print_label(text):
    label_width_mm = 12
    label_height_mm = 12
    dpi = 300
    mm_to_inch = 25.4

    label_width_px = int((label_width_mm / mm_to_inch) * dpi)
    label_height_px = int((label_height_mm / mm_to_inch) * dpi)

    img = Image.new('RGB', (label_width_px, label_height_px), color=(255, 255, 255))
    draw = ImageDraw.Draw(img)

    qr = qrcode.QRCode(
        version=1,
        error_correction=qrcode.constants.ERROR_CORRECT_L,
        box_size=10,
        border=0,
    )
    qr.add_data(text)
    qr.make(fit=True)
    qr_img = qr.make_image(fill='black', back_color='white')

    max_qr_width = int(label_width_px * 0.8)
    max_qr_height = int(label_height_px * 0.8)
    qr_width, qr_height = qr_img.size
    aspect_ratio = qr_width / qr_height

    if qr_width > qr_height:
        new_width = min(max_qr_width, qr_width)
        new_height = int(new_width / aspect_ratio)
    else:
        new_height = min(max_qr_height, qr_height)
        new_width = int(new_height * aspect_ratio)

    qr_img = qr_img.resize((new_width, new_height), resample=Image.Resampling.LANCZOS)

    x_pos = (label_width_px - new_width) // 2
    y_pos = (label_height_px - new_height) // 2
    img.paste(qr_img, (x_pos, y_pos))

    img.save('image.png', dpi=(dpi, dpi))

    qlr = BrotherQLRaster(PRINTER_MODEL)
    qlr.exception_on_warning = True

    instructions = convert(
        qlr=qlr,
        images=[img],
        label=LABEL_TYPE,
        rotate='90',
        threshold=70.0,
        dither=False,
        compress=False,
        red=False,
        dpi_600=False,
        hq=True,
        cut=True
    )

    PRINTER_INTERFACE = f"tcp://{PRINTER_IP}:{PRINTER_PORT}"
    send(instructions=instructions, printer_identifier=PRINTER_INTERFACE, backend_identifier='network', blocking=True)

@app.post("/print")
async def print_label_endpoint(request: PrintRequest):
    try:
        print_label(request.text)
        return {"status": "success"}
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=5500)
