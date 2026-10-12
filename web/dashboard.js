export function rows(report) {
  if (
    report.classification !== "OBSERVED" ||
    !report.baseline?.summary ||
    !report.candidate?.summary
  )
    throw Error("Invalid report");
  return ["baseline", "candidate"].map((name) => ({
    name,
    ...report[name].summary,
  }));
}
if (typeof document !== "undefined") {
  document.querySelector("input").addEventListener("change", async (e) => {
    const out = document.querySelector("main");
    try {
      const file = e.target.files[0];
      if (file.size > 2e6) throw Error("Too large");
      const r = JSON.parse(await file.text());
      out.textContent = r.verdict;
      for (const x of rows(r)) {
        const p = document.createElement("p");
        p.textContent = `${x.name}: ${x.n} runs, mean ${x.mean.toFixed(3)} ms; SD ${x.sample_sd.toFixed(3)} ms; exploratory 95% interval ${x.approximate_mean_ci95.map((v) => v.toFixed(3)).join(" … ")} ms`;
        out.append(p);
      }
    } catch (error) {
      out.textContent = String(error);
    }
  });
}
