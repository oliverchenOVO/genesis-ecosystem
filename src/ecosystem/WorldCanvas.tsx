import { useEffect, useRef, useState } from "react";
import type { Snapshot, DisplayOrganism } from "../types";
export function nearestOrganism(
  organisms: DisplayOrganism[],
  x: number,
  y: number,
  radius: number,
): DisplayOrganism | undefined {
  let best: DisplayOrganism | undefined;
  let distance = radius * radius;
  for (const o of organisms) {
    const d = (o.x - x) ** 2 + (o.y - y) ** 2;
    if (d < distance || (d === distance && (!best || o.id < best.id))) {
      distance = d;
      best = o;
    }
  }
  return best;
}
export function WorldCanvas({
  snapshot,
  select,
  selected,
}: {
  snapshot: Snapshot;
  select: (id: number) => void;
  selected: number | null;
}) {
  const canvas = useRef<HTMLCanvasElement>(null);
  const [layer, setLayer] = useState("Resources");
  useEffect(() => {
    const element = canvas.current;
    if (!element) return;
    const ctx = element.getContext("2d");
    if (!ctx) return;
    const size = 1024;
    element.width = size;
    element.height = size;
    const scale = size / snapshot.size;
    ctx.fillStyle = "#0d1711";
    ctx.fillRect(0, 0, size, size);
    const side = snapshot.size / 16;
    const cell = size / side;
    snapshot.cells.forEach(([food, offset], i) => {
      ctx.fillStyle =
        layer === "Resources"
          ? `rgb(${12 + food / 100},${24 + food / 22},${18 + food / 80})`
          : `hsl(${210 - (snapshot.temperature + offset) / 30} 28% 23%)`;
      ctx.fillRect((i % side) * cell, Math.floor(i / side) * cell, cell, cell);
    });
    for (const o of snapshot.organisms) {
      const x = o.x * scale,
        y = o.y * scale;
      const radius = (1.2 + o.body_size / 5) * Math.max(0.8, scale / 2);
      ctx.save();
      ctx.translate(x, y);
      ctx.rotate(Math.atan2(o.dy, o.dx));
      ctx.fillStyle =
        o.carnivory > 650
          ? "#d08e87"
          : `hsl(${100 + ((o.species_id * 23) % 90)} 44% 73%)`;
      ctx.strokeStyle = "#91b896";
      ctx.lineWidth = 0.8;
      if (o.speed > 6) {
        ctx.beginPath();
        ctx.moveTo(-radius * 2, 0);
        ctx.lineTo(-radius * 3.5, 0);
        ctx.stroke();
      }
      ctx.beginPath();
      if (o.carnivory > 650) {
        ctx.moveTo(radius * 1.7, 0);
        ctx.lineTo(-radius, -radius);
        ctx.lineTo(-radius, radius);
        ctx.closePath();
      } else {
        ctx.ellipse(0, 0, radius * 1.6, radius, 0, 0, Math.PI * 2);
      }
      ctx.fill();
      ctx.fillStyle = "#253d29";
      ctx.beginPath();
      ctx.arc(radius * 0.8, 0, 1, 0, Math.PI * 2);
      ctx.fill();
      if (o.id === selected) {
        ctx.strokeStyle = "#ffffff";
        ctx.lineWidth = 1.5;
        ctx.beginPath();
        ctx.arc(0, 0, radius * 2.4, 0, Math.PI * 2);
        ctx.stroke();
      }
      ctx.restore();
    }
  }, [snapshot, layer, selected]);
  return (
    <section className="ecosystem">
      <div className="canvas-toolbar">
        <span>
          World {snapshot.size} × {snapshot.size}
        </span>
        <div className="segments">
          {["Resources", "Temperature"].map((value) => (
            <button
              key={value}
              className={layer === value ? "active" : ""}
              onClick={() => setLayer(value)}
            >
              {value}
            </button>
          ))}
        </div>
      </div>
      <canvas
        ref={canvas}
        aria-label="Ecosystem map. Select a living organism to inspect its phenotype."
        onClick={(event) => {
          const rect = event.currentTarget.getBoundingClientRect();
          const o = nearestOrganism(
            snapshot.organisms,
            ((event.clientX - rect.left) / rect.width) * snapshot.size,
            ((event.clientY - rect.top) / rect.height) * snapshot.size,
            snapshot.size / 40,
          );
          if (o) select(o.id);
        }}
      />
      <div className="map-caption">
        <span>
          ● Herbivore <span className="predator">▲ Predator</span>
        </span>
        <span>
          {snapshot.population === 0
            ? "All organisms extinct"
            : `${snapshot.population.toLocaleString()} living organisms`}
        </span>
      </div>
      <div className="accessible-organisms">
        <label>
          Inspect individual
          <select
            value={selected ?? ""}
            onChange={(e) => select(Number(e.target.value))}
          >
            <option value="">Select organism</option>
            {snapshot.organisms.map((o) => (
              <option key={o.id} value={o.id}>
                Individual #{o.id}
              </option>
            ))}
          </select>
        </label>
      </div>
    </section>
  );
}
