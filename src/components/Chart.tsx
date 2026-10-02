export function seriesPath(
  values: number[],
  width = 400,
  height = 100,
): string {
  if (!values.length) return "";
  const max = Math.max(1, ...values);
  return values
    .map(
      (v, i) =>
        `${i ? "L" : "M"}${((i * width) / Math.max(1, values.length - 1)).toFixed(2)},${(height - (v / max) * height).toFixed(2)}`,
    )
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
  const max = Math.max(1, ...values);
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
          <path className="series" d={seriesPath(values)} />
        </svg>
      </div>
      <div className="axis">
        <span>{ticks[0] ?? 0}</span>
        <span>
          {values.length < 2
            ? "Waiting for the next sample"
            : `Tick ${ticks.at(-1)?.toLocaleString()}`}
        </span>
      </div>
    </section>
  );
}
