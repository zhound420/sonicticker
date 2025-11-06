export const renderWaveform = (
  ctx: CanvasRenderingContext2D,
  waveform: Float32Array,
  color: string,
) => {
  const { width, height } = ctx.canvas;
  ctx.save();
  ctx.strokeStyle = color;
  ctx.lineWidth = 2;
  ctx.beginPath();

  const slice = width / waveform.length;
  waveform.forEach((value, index) => {
    const x = index * slice;
    const y = (1 - (value + 1) / 2) * height;
    if (index === 0) {
      ctx.moveTo(x, y);
    } else {
      ctx.lineTo(x, y);
    }
  });

  ctx.stroke();
  ctx.restore();
};
