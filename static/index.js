'use strict';
const drawPoly = (ctx, poly, color) => {
    ctx.fillStyle = color;
    ctx.beginPath();
    poly.forEach((point, i) => {
        i ? ctx.lineTo(point.x, point.y) : ctx.moveTo(point.x, point.y);
    });
    ctx.closePath();
    ctx.fill();
}

const movePoly = (poly, xOffset, yOffset) => {
    poly.map(point => {
        point.x += xOffset
        point.y += yOffset
    })
}

const toRound = (poly) => {
    return poly.map(point => {
        return {x: Math.round(point.x),y: Math.round(point.y)}
    })
}

Rust.wasm_poly.then(module => {
    const poly = [
        { x: 160, y: 400 },
        { x: 160, y: 100 },
        { x: 500, y: 100 },
        { x: 500, y: 400 },
        { x: 400, y: 400 },
        { x: 300, y: 200 },
    ];
    const clip = [
        { x: 80, y: 240 },
        { x: 350, y: 140 },
        { x: 430, y: 310 },
        { x: 100, y: 500 },
        { x: 300, y: 300 },
    ];
    const canvas = document.querySelector("#canvas");
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    const ctx = canvas.getContext('2d');
    let offset = 1;
    let count = 0;
    const draw = time => {
        ctx.clearRect(0,0,canvas.width, canvas.height)
        if (count > 100) {
            count = 0

            offset = -offset;
        }
        count++;
        movePoly(poly, offset, offset);
        drawPoly(ctx, poly, '#ff00a2');
        movePoly(clip, -offset, -offset);
        drawPoly(ctx, clip, '#00ffc1');
        let before = window.performance.now();
        //console.log(toRound(poly), toRound(clip))

        let cl = module.get_clip(
            toRound(poly),
            toRound(clip));

        let diff = window.performance.now() - before;
        if (cl.length == 0 || cl[0].length == 0) {
            console.log(toRound(poly), toRound(clip));
            return
        }
        cl.forEach(p => drawPoly(ctx, p, '#ff0000'));
        window.requestAnimationFrame(draw);
    }
    window.requestAnimationFrame(draw);

})
