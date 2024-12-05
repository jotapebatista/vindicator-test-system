from PIL import Image, ImageDraw, ImageFont
import qrcode
from brother_ql.conversion import convert
from brother_ql.backends.helpers import send
from brother_ql.raster import BrotherQLRaster
from flask import Flask, request, jsonify
from flask_cors import CORS
Image.ANTIALIAS = Image.LANCZOS

app = Flask(__name__)
CORS(app, resources={r"/print": {"origins": "http://localhost:3000"}})


PRINTER_MODEL = 'QL-820NWB'
PRINTER_IP = '192.168.40.221'
PRINTER_PORT = 9100
LABEL_TYPE = '12'

def print_label(text):
    # Adjust label dimensions for a 12 mm label
    label_width_mm = 12  # 12 mm label width
    label_height_mm = 12  # Endless 29 mm length
    dpi = 300  # Printer DPI (dots per inch)
    mm_to_inch = 25.4  # Conversion factor

    # Convert dimensions to pixels
    label_width_px = int((label_width_mm / mm_to_inch) * dpi)
    label_height_px = int((label_height_mm / mm_to_inch) * dpi)

    # Create a blank image
    img = Image.new('RGB', (label_width_px, label_height_px), color=(255, 255, 255))
    draw = ImageDraw.Draw(img)

    # Add QR code
    qr = qrcode.QRCode(
        version=1,
        error_correction=qrcode.constants.ERROR_CORRECT_L,
        box_size=10,
        border=0,
    )
    qr.add_data(text)
    qr.make(fit=True)
    qr_img = qr.make_image(fill='black', back_color='white')

    # Resize QR code to fit within the label
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

    # Center the QR code on the label
    x_pos = (label_width_px - new_width) // 2
    y_pos = (label_height_px - new_height) // 2
    img.paste(qr_img, (x_pos, y_pos))

    # Save the image for debugging
    img.save('image.png', dpi=(dpi, dpi))

    # Send the image to the printer
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

@app.route('/print', methods=['POST'])
def print_label_from_request():
    data = request.json
    if not data or 'text' not in data:
        return jsonify({"error": "Missing 'text' field in JSON data"}), 400

    text = data['text']
    try:
        print_label(text)
        return jsonify({"status": "success"}), 200
    except Exception as e:
        return jsonify({"error": str(e)}), 500

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5500)
