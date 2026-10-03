import { useState, useEffect, useRef } from "react";
import { Shuffle, X } from "lucide-react";
import type { WorldConfig } from "../types";
export function validateConfig(config: WorldConfig): string | null {
  if (!Number.isSafeInteger(config.seed) || config.seed < 0)
    return "Seed must be a nonnegative safe integer.";
  if (![256, 512, 1024].includes(config.size))
    return "Select a supported world size.";
  if (
    !Number.isInteger(config.starting_population) ||
    config.starting_population < 1 ||
    config.starting_population > config.population_limit
  )
    return "Starting population must fit within the world limit.";
  if (
    !Number.isInteger(config.population_limit) ||
    config.population_limit > 5000 ||
    config.population_limit < 1
  )
    return "Population limit must be 1–5000.";
  if (![0, 50, 100, 200, 500].includes(config.mutation_multiplier))
    return "Select a supported mutation preset.";
  return null;
}
export function NewWorld({
  create,
  close,
  busy,
}: {
  create: (
    config: WorldConfig,
    temperature: number,
    regeneration: number,
  ) => void;
  close: () => void;
  busy: boolean;
}) {
  const dialog = useRef<HTMLDialogElement>(null);
  const [temperature, setTemperature] = useState(20);
  const [regeneration, setRegeneration] = useState(12);
  const [config, setConfig] = useState<WorldConfig>({
    seed: 42,
    size: 512,
    starting_population: 50,
    population_limit: 200,
    mutation_multiplier: 100,
  });
  const error = validateConfig(config);
  useEffect(() => {
    dialog.current?.showModal();
  }, []);
  return (
    <dialog ref={dialog} onCancel={close} aria-labelledby="new-world-title">
      <form
        onSubmit={(e) => {
          e.preventDefault();
          if (!error)
            create(config, Math.round(temperature * 100), regeneration);
        }}
      >
        <div className="view-heading">
          <h2 id="new-world-title">New world</h2>
          <button type="button" aria-label="Close new world" onClick={close}>
            <X size={18} />
          </button>
        </div>
        <p className="muted">
          A reproducible ecosystem begins with a seed. Creating a world replaces
          the current session after you choose how to handle unsaved progress.
        </p>
        <button
          type="button"
          disabled={busy}
          onClick={() =>
            setConfig({
              seed: 11,
              size: 512,
              starting_population: 50,
              population_limit: 200,
              mutation_multiplier: 100,
            })
          }
        >
          Showcase · Seed 11
        </button>
        <p className="muted">
          Showcase uses natural evolution: 50 founders, capacity 200. Run MAX to
          observe genetic divergence; speciation is not forced.
        </p>
        <label>
          Seed
          <div className="seed-input">
            <input
              aria-label="Seed"
              type="number"
              min="0"
              max={Number.MAX_SAFE_INTEGER}
              value={config.seed}
              onChange={(e) =>
                setConfig({ ...config, seed: Number(e.target.value) })
              }
            />
            <button
              type="button"
              aria-label="Random seed"
              onClick={() =>
                setConfig({
                  ...config,
                  seed: crypto.getRandomValues(new Uint32Array(1))[0],
                })
              }
            >
              <Shuffle size={16} />
            </button>
          </div>
        </label>
        <label>
          World size
          <select
            value={config.size}
            onChange={(e) =>
              setConfig({ ...config, size: Number(e.target.value) })
            }
          >
            <option value="256">Small · 256 × 256</option>
            <option value="512">Standard · 512 × 512</option>
            <option value="1024">Large · 1024 × 1024</option>
          </select>
        </label>
        <label>
          Technical safety population ceiling
          <input
            type="number"
            min="1"
            max="5000"
            value={config.population_limit}
            onChange={(e) =>
              setConfig({ ...config, population_limit: Number(e.target.value) })
            }
          />
        </label>
        <label>
          Starting population
          <input
            type="number"
            min="1"
            max={config.population_limit}
            value={config.starting_population}
            onChange={(e) =>
              setConfig({
                ...config,
                starting_population: Number(e.target.value),
              })
            }
          />
        </label>
        <label>
          Mutation tendency
          <select
            value={config.mutation_multiplier}
            onChange={(e) =>
              setConfig({
                ...config,
                mutation_multiplier: Number(e.target.value),
              })
            }
          >
            <option value="0">None</option>
            <option value="50">Low · 0.5×</option>
            <option value="100">Standard · 1×</option>
            <option value="200">High · 2×</option>
            <option value="500">Very high · 5×</option>
          </select>
        </label>
        <label>
          Initial temperature (°C)
          <input
            type="number"
            min="-20"
            max="60"
            step="1"
            value={temperature}
            onChange={(e) => setTemperature(Number(e.target.value))}
            required
          />
        </label>
        <label>
          Initial food regeneration
          <input
            type="number"
            min="0"
            max="100"
            step="1"
            value={regeneration}
            onChange={(e) => setRegeneration(Number(e.target.value))}
            required
          />
        </label>
        {error ? (
          <p className="error" role="alert">
            {error}
          </p>
        ) : null}
        <button
          className="primary full"
          type="submit"
          disabled={busy || !!error}
        >
          Start world
        </button>
      </form>
    </dialog>
  );
}
