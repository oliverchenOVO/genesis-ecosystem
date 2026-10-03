import { useRef, useState } from "react";
import { Minus, Plus, RotateCcw } from "lucide-react";
import type { Species } from "../types";
export function treePositions(
  species: Species[],
): Map<number, { x: number; y: number }> {
  const positions = new Map<number, { x: number; y: number }>();
  [...species]
    .sort((a, b) => a.id - b.id)
    .forEach((s, i) => {
      const parent = s.ancestor ? positions.get(s.ancestor) : undefined;
      positions.set(s.id, {
        x: 60 + (parent ? (parent.x - 60) / 300 + 1 : 0) * 300,
        y: 80 + i * 120,
      });
    });
  return positions;
}
export function EvolutionView({
  species,
  select,
}: {
  species: Species[];
  select: (id: number) => void;
}) {
  const [scale, setScale] = useState(1);
  const [pan, setPan] = useState({ x: 0, y: 0 });
  const drag = useRef<{ x: number; y: number; px: number; py: number } | null>(
    null,
  );
  const positions = treePositions(species);
  return (
    <section className="evolution-workspace">
      <div className="view-heading">
        <div>
          <h1>Evolution</h1>
          <p className="muted">
            Species ancestry from recorded divergence events
          </p>
        </div>
        <div className="segments">
          <button
            aria-label="Zoom out"
            onClick={() => setScale((s) => Math.max(0.25, s / 1.2))}
          >
            <Minus size={16} />
          </button>
          <span>{Math.round(scale * 100)}%</span>
          <button
            aria-label="Zoom in"
            onClick={() => setScale((s) => Math.min(4, s * 1.2))}
          >
            <Plus size={16} />
          </button>
          <button
            aria-label="Reset tree view"
            onClick={() => {
              setScale(1);
              setPan({ x: 0, y: 0 });
            }}
          >
            <RotateCcw size={16} />
          </button>
        </div>
      </div>
      <svg
        className="evolution-tree"
        aria-label="Evolution tree"
        viewBox="0 0 1100 650"
        onPointerDown={(e) => {
          if ((e.target as Element).closest("button")) return;
          drag.current = { x: e.clientX, y: e.clientY, px: pan.x, py: pan.y };
          e.currentTarget.setPointerCapture(e.pointerId);
        }}
        onPointerMove={(e) => {
          if (drag.current) {
            const ratio = 1100 / e.currentTarget.getBoundingClientRect().width;
            setPan({
              x: drag.current.px + (e.clientX - drag.current.x) * ratio,
              y: drag.current.py + (e.clientY - drag.current.y) * ratio,
            });
          }
        }}
        onPointerUp={() => {
          drag.current = null;
        }}
        onPointerCancel={() => {
          drag.current = null;
        }}
      >
        <defs>
          <pattern
            id="tree-grid"
            width="24"
            height="24"
            patternUnits="userSpaceOnUse"
          >
            <circle cx="1" cy="1" r=".6" fill="#30413a" />
          </pattern>
        </defs>
        <rect width="1100" height="650" fill="url(#tree-grid)" />
        <g transform={`translate(${pan.x} ${pan.y}) scale(${scale})`}>
          {species.map((s) => {
            const to = positions.get(s.id)!;
            const from = s.ancestor ? positions.get(s.ancestor) : undefined;
            return from ? (
              <path
                key={`edge-${s.id}`}
                className="tree-edge"
                d={`M${from.x + 230},${from.y + 35}C${from.x + 270},${from.y + 35} ${to.x - 40},${to.y + 35} ${to.x},${to.y + 35}`}
              />
            ) : null;
          })}
          {species.map((s) => {
            const pos = positions.get(s.id)!;
            return (
              <foreignObject
                key={s.id}
                x={pos.x}
                y={pos.y}
                width="235"
                height="85"
              >
                <button
                  className={`tree-node ${s.extinct_tick !== null ? "extinct" : ""}`}
                  onClick={() => select(s.id)}
                >
                  <strong>{s.name}</strong>
                  <span>
                    Generation {s.origin_generation} · tick{" "}
                    {s.origin_tick.toLocaleString()}
                  </span>
                  <small>
                    {s.representative_morphology.segment_count} units ·{" "}
                    {s.representative_morphology.mouth} · armor{" "}
                    {Math.round(s.representative_morphology.armor / 10)}% ·{" "}
                    {s.extinct_tick === null
                      ? `${s.population} living`
                      : `Extinct · tick ${s.extinct_tick}`}
                  </small>
                </button>
              </foreignObject>
            );
          })}
        </g>
      </svg>
      <p className="muted note">
        Drag the canvas to pan. Select a species for its evidence and trait
        history. A single node means no branch has yet passed the species
        criteria.
      </p>
    </section>
  );
}
