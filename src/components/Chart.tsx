export function seriesMaximum(values: number[]): number {
  return values.reduce((max, value) => Math.max(max, value), 1);
}
export function seriesPath(
  values: number[],
  width = 400,
  height = 100,
  ticks?: number[],
): string {
  if (!values.length) return "";
  const max = seriesMaximum(values);
  const timed = ticks?.length === values.length && ticks.length > 1;
  const start = timed ? ticks[0] : 0;
  const duration = timed ? ticks.at(-1)! - start : 0;
  return values
    .map((v, i) => {
      const x =
        timed && duration > 0
          ? ((ticks![i] - start) * width) / duration
          : (i * width) / Math.max(1, values.length - 1);
      return `${i ? "L" : "M"}${x.toFixed(2)},${(height - (v / max) * height).toFixed(2)}`;
    })
    .join(" ");
}
export function Chart({
  label,
  values,
  ticks,
  unit = "",
}: {
  label: string;
  values: number[];
  ticks: number[];
  unit?: string;
}) {
  const max = seriesMaximum(values);
  return (
    <section className="chart">
      <div className="chart-heading">
        <h3>{label}</h3>
        <strong>
          {values.at(-1)?.toLocaleString() ?? "—"}
          <small>{unit}</small>
        </strong>
      </div>
      <div className="plot">
        <span className="axis-max">{max.toLocaleString()}</span>
        <svg
          viewBox="0 0 400 100"
          role="img"
          aria-label={`${label} measured over ${values.length} samples`}
          preserveAspectRatio="none"
        >
          <path
            className="gridline"
            d="M0 0H400 M0 50H400 M0 100H400 M100 0V100 M200 0V100 M300 0V100"
          />
          <path className="series" d={seriesPath(values, 400, 100, ticks)} />
          {values.length === 1 ? (
            <circle
              cx="0"
              cy={100 - (values[0] / max) * 100}
              r="3"
              fill="currentColor"
            />
          ) : null}
        </svg>
      </div>
      <div className="axis">
        <span>{ticks[0]?.toLocaleString() ?? "—"}</span>
        <span>
          {values.length === 0
            ? "Waiting for the first sample"
            : `Latest sample · tick ${ticks.at(-1)?.toLocaleString() ?? "—"}`}
        </span>
      </div>
    </section>
  );
}
