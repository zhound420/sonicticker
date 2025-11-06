interface Particle {
  x: number;
  y: number;
  radius: number;
  hue: number;
  vx: number;
  vy: number;
  life: number;
}

export class ParticleSystem {
  private particles: Particle[] = [];

  update(
    ctx: CanvasRenderingContext2D,
    intensity: number,
    bullish: boolean,
    delta: number,
  ) {
    const spawnCount = Math.floor(intensity * 10);
    for (let i = 0; i < spawnCount; i += 1) {
      this.particles.push({
        x: Math.random() * ctx.canvas.width,
        y: ctx.canvas.height,
        radius: Math.random() * 4 + 2,
        hue: bullish ? 140 : 0,
        vx: (Math.random() - 0.5) * 0.5,
        vy: -Math.random() * 1.5 - 0.5,
        life: 1.0,
      });
    }

    ctx.save();
    ctx.globalCompositeOperation = 'lighter';

    this.particles = this.particles
      .map((particle) => {
        const next = { ...particle };
        next.x += next.vx * delta;
        next.y += next.vy * delta * 60;
        next.life -= 0.01 * delta * 60;
        return next;
      })
      .filter((particle) => particle.life > 0 && particle.y > -20);

    this.particles.forEach((particle) => {
      ctx.beginPath();
      ctx.fillStyle = `hsla(${particle.hue}, 70%, 55%, ${particle.life})`;
      ctx.arc(particle.x, particle.y, particle.radius, 0, Math.PI * 2);
      ctx.fill();
    });

    ctx.restore();
  }
}
