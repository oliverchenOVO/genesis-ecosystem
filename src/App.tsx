import { useEffect, useState, useCallback, useRef } from "react";
import { Copy, FolderOpen, Pause, Play, Plus, Save, Check } from "lucide-react";
import { request, readableError } from "./api";
import type {
  Action,
  Page,
  Snapshot,
  Organism,
  Species,
  Telemetry,
  HistoryEvent,
  WorldConfig,
} from "./types";
import { WorldCanvas } from "./ecosystem/WorldCanvas";
import { Environment } from "./components/Environment";
import { Inspector } from "./components/Inspector";
import { Chart } from "./components/Chart";
import { NewWorld } from "./components/NewWorld";
import { SpeciesView } from "./species/SpeciesView";
import { EvolutionView } from "./evolution/EvolutionView";
import { HistoryView } from "./history/HistoryView";

export function App() {
  const [snapshot, setSnapshot] = useState<Snapshot | null>(null);
  const [page, setPage] = useState<Page>("World");
  const [species, setSpecies] = useState<Species[]>([]);
  const [telemetry, setTelemetry] = useState<Telemetry[]>([]);
  const [history, setHistory] = useState<HistoryEvent[]>([]);
  const [selected, setSelected] = useState<number | null>(null);
  const [organism, setOrganism] = useState<Organism | null>(null);
  const [selectedSpecies, setSelectedSpecies] = useState<number | null>(null);
  const [missing, setMissing] = useState("");
  const [error, setError] = useState("");
  const [message, setMessage] = useState("");
  const [busy, setBusy] = useState(false);
  const [creating, setCreating] = useState(false);
  const [epoch, setEpoch] = useState(0);
  const [connected, setConnected] = useState(false);
  const revision = useRef(0);
  const lastPanels = useRef(0);
  const polling = useRef(false);
  const mounted = useRef(true);
  useEffect(() => {
    mounted.current = true;
    return () => {
      mounted.current = false;
    };
  }, []);
  const refresh = useCallback(async () => {
    if (polling.current) return;
    polling.current = true;
    const currentRevision = revision.current;
    const current = () =>
      mounted.current && currentRevision === revision.current;
    try {
      const next = await request<Snapshot>({ op: "snapshot" });
      if (!current()) return;
      setSnapshot(next);
      setConnected(true);
      if (Date.now() - lastPanels.current > 1000) {
        lastPanels.current = Date.now();
        const [records, samples] = await Promise.all([
          request<Species[]>({ op: "species" }),
          request<Telemetry[]>({ op: "telemetry" }),
        ]);
        if (!current()) return;
        setSpecies(records);
        setTelemetry(samples);
        if (page === "History") {
          const events = await request<HistoryEvent[]>({ op: "history" });
          if (current()) setHistory(events);
        }
      }
      if (selected !== null) {
        try {
          const detail = await request<Organism>({
            op: "detail",
            id: selected,
          });
          if (current()) {
            setOrganism(detail);
            setMissing("");
          }
        } catch {
          if (current()) {
            setOrganism(null);
            setMissing(
              `Individual #${selected} is no longer alive. Its ancestry remains in the save.`,
            );
          }
        }
      }
    } catch (e) {
      if (current()) {
        setConnected(false);
        setError(readableError(e));
      }
    } finally {
      polling.current = false;
    }
  }, [selected, page]);
  useEffect(() => {
    lastPanels.current = 0;
    void refresh();
    const interval = setInterval(() => {
      void refresh();
    }, 150);
    return () => clearInterval(interval);
  }, [refresh, epoch]);
  async function act(action: Action, success?: string) {
    setBusy(true);
    setError("");
    setMessage("");
    if (action.op === "new" || action.op === "load") revision.current++;
    try {
      const result = await request<Record<string, unknown>>(action);
      if (action.op === "new" || action.op === "load") {
        setSelected(null);
        setOrganism(null);
        setMissing("");
        setTelemetry([]);
        setHistory([]);
        setEpoch((v) => v + 1);
        setSnapshot(result as unknown as Snapshot);
      }
      if (action.op === "control" || action.op === "environment")
        setSnapshot(result as unknown as Snapshot);
      if (action.op === "replay")
        setMessage(
          `Replay verified · tick ${result.tick} · hash ${String(result.hash).slice(0, 16)}…`,
        );
      else if (success)
        setMessage(success + (action.op === "save" ? ` · ${result.path}` : ""));
      lastPanels.current = 0;
      if (action.op !== "new" && action.op !== "load") await refresh();
      return true;
    } catch (e) {
      setError(readableError(e));
      return false;
    } finally {
      setBusy(false);
    }
  }
  function selectSpecies(id: number) {
    setSelectedSpecies(id);
    setPage("Species");
  }
  async function create(config: WorldConfig) {
    if (await act({ op: "new", config }, "New world started"))
      setCreating(false);
  }
  return (
    <div className="app-shell">
      <header>
        <div className="brand">
          GENESIS
          <small>
            DETERMINISTIC LIFE
            <br />
            RESEARCH ENVIRONMENT
          </small>
        </div>
        <nav aria-label="Main navigation">
          {(["World", "Species", "Evolution", "History"] as Page[]).map((p) => (
            <button
              className={page === p ? "active" : ""}
              key={p}
              onClick={() => setPage(p)}
            >
              {p}
            </button>
          ))}
        </nav>
        <div className="header-actions">
          <button disabled={busy} onClick={() => setCreating(true)}>
            <Plus size={17} />
            New World
          </button>
          <button
            disabled={busy || !snapshot}
            onClick={() => {
              void act({ op: "save", path: null }, "World saved");
            }}
          >
            <Save size={17} />
            Save
          </button>
          <button
            disabled={busy}
            onClick={() => {
              void act({ op: "load", path: null }, "World loaded · paused");
            }}
          >
            <FolderOpen size={17} />
            Load
          </button>
        </div>
      </header>
      {snapshot ? (
        <>
          <div className="time-toolbar">
            <div className="seed-display">
              <span>Seed</span>
              <strong>{snapshot.seed}</strong>
              <button
                aria-label="Copy seed"
                onClick={() => {
                  void navigator.clipboard
                    .writeText(snapshot.seed)
                    .then(() => setMessage("Seed copied"))
                    .catch((e) => setError(readableError(e)));
                }}
              >
                <Copy size={15} />
                <span>Copy Seed</span>
              </button>
            </div>
            <span className="stat">
              Tick <strong>{snapshot.tick.toLocaleString()}</strong>
            </span>
            <span className="stat">
              Population <strong>{snapshot.population.toLocaleString()}</strong>
            </span>
            <span className="stat">
              Species <strong>{snapshot.species_count}</strong>
            </span>
            <div className="playback">
              <button
                disabled={busy}
                className="play-button"
                onClick={() => {
                  void act({
                    op: "control",
                    running: !snapshot.running,
                    speed: snapshot.speed,
                  });
                }}
              >
                {snapshot.running ? <Pause size={17} /> : <Play size={17} />}{" "}
                {snapshot.running ? "Pause" : "Play"}
              </button>
              {[1, 4, 16, 64, 0].map((speed) => (
                <button
                  key={speed}
                  disabled={busy}
                  className={snapshot.speed === speed ? "active" : ""}
                  onClick={() => {
                    void act({
                      op: "control",
                      running: snapshot.running,
                      speed,
                    });
                  }}
                >
                  {speed === 0 ? "MAX" : `${speed}x`}
                </button>
              ))}
            </div>
          </div>
          <main>
            {page === "World" ? (
              <>
                <div className="world-layout">
                  <Environment
                    key={epoch}
                    snapshot={snapshot}
                    busy={busy}
                    apply={(temperature, regeneration) => {
                      void act(
                        { op: "environment", temperature, regeneration },
                        "Environment intervention recorded",
                      );
                    }}
                  />
                  <WorldCanvas
                    snapshot={snapshot}
                    selected={selected}
                    select={(id) => {
                      setSelected(id);
                      setOrganism(null);
                      setMissing("");
                    }}
                  />
                  <Inspector
                    organism={organism}
                    snapshot={snapshot}
                    species={species}
                    selectSpecies={selectSpecies}
                    missing={missing}
                  />
                </div>
                <div className="telemetry-band">
                  <Chart
                    label="Population"
                    values={telemetry.map((t) => t.population)}
                    ticks={telemetry.map((t) => t.tick)}
                    unit="organisms"
                  />
                  <Chart
                    label="Species"
                    values={telemetry.map((t) => t.species_count)}
                    ticks={telemetry.map((t) => t.tick)}
                    unit="living"
                  />
                  <Chart
                    label="Food"
                    values={telemetry.map((t) => t.food)}
                    ticks={telemetry.map((t) => t.tick)}
                    unit="resource units"
                  />
                </div>
              </>
            ) : page === "Species" ? (
              <SpeciesView
                species={species}
                telemetry={telemetry}
                selected={selectedSpecies}
                select={setSelectedSpecies}
              />
            ) : page === "Evolution" ? (
              <EvolutionView species={species} select={selectSpecies} />
            ) : (
              <HistoryView
                history={history}
                species={species}
                select={selectSpecies}
              />
            )}
          </main>
          <footer>
            <span className={connected ? "connection" : "error"}>
              {connected ? "Rust simulation connected" : "Connection lost"} ·
              Simulation v{snapshot.simulation_version} ·{" "}
              {snapshot.running ? "Running" : "Paused"}
            </span>
            <button
              disabled={busy}
              onClick={() => {
                void act({ op: "replay" });
              }}
            >
              <Check size={14} />
              Verify replay
            </button>
            <span>Autosave every 5,000 ticks · 3 rotating slots</span>
          </footer>
        </>
      ) : (
        <main className="connection-screen">
          <h1>Connecting to the ecosystem</h1>
          <p>The desktop simulation worker is starting.</p>
        </main>
      )}
      {error ? (
        <div className="notification error" role="alert">
          <strong>Action could not complete</strong>
          <span>{error}</span>
          <button aria-label="Dismiss error" onClick={() => setError("")}>
            Dismiss
          </button>
        </div>
      ) : message ? (
        <div className="notification" role="status">
          {message}
          <button
            aria-label="Dismiss notification"
            onClick={() => setMessage("")}
          >
            Dismiss
          </button>
        </div>
      ) : null}
      {snapshot?.autosave_error ? (
        <div className="notification error" role="alert">
          Autosave failed: {snapshot.autosave_error}
        </div>
      ) : null}
      {busy ? (
        <div className="working" role="status">
          Working…
        </div>
      ) : null}
      {creating ? (
        <NewWorld
          create={(config) => {
            void create(config);
          }}
          close={() => setCreating(false)}
          busy={busy}
        />
      ) : null}
    </div>
  );
}
