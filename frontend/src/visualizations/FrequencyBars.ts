export const renderFrequencyBars = (
  ctx: CanvasRenderingContext2D,
  frequencies: Uint8Array,
) => {
  const { width, height } = ctx.canvas;
  const barWidth = Math.max(width / frequencies.length, 2);

  ctx.save();
  ctx.fillStyle = '#0f172a';
  ctx.fillRect(0, height * 0.65, width, height * 0.35);

  frequencies.forEach((value, index) => {
    const normalized = value / 255;
    const barHeight = normalized * (height * 0.35);
    const x = index * barWidth;
    const hue = 200 - normalized * 120;
    ctx.fillStyle = `hsl(${hue}, 70%, ${40 + normalized * 40}%)`;
    ctx.fillRect(x, height - barHeight, barWidth - 1, barHeight);
  });

  ctx.restore();
};
