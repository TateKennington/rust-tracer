import * as wasm from "rust-tracer";

const canvas = document.createElement('canvas');
canvas.width = 400
canvas.height = 225
document.body.append(canvas);
const ctx = canvas.getContext('2d');
ctx.globalCompositeOperation = 'lighten';
const images = [];
(async () => {
    for (let i = 0; i < 100; i++) {
        console.time('render');
        const buffer = wasm.render();
        console.timeEnd('render');
        const blob = new Blob([buffer]);
        const image = await createImageBitmap(blob);
        images.push(image);
        ctx.globalAlpha = 1 / images.length;
        ctx.clearRect(0, 0, 400, 225)
        images.forEach((image) => {
            ctx.drawImage(image, 0, 0);
        })
    }
})();
