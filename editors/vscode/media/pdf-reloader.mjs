// Keep the current document alive until a complete replacement has loaded.
// Loading tasks share an explicitly owned worker, so destroying an old task
// does not destroy the replacement's worker.
export function createPdfReloader({ load, commit, onError }) {
  let current;
  let requested = 0;
  let sequence = Promise.resolve();
  return (url) => {
    const version = ++requested;
    sequence = sequence.then(async () => {
      if (version !== requested) { return; }
      let candidate;
      try {
        candidate = await load(url);
        const pdf = await candidate.promise;
        await pdf.getPage(1);
        if (version !== requested) { await candidate.destroy(); return; }
        await commit(pdf);
        const previous = current;
        current = candidate;
        candidate = undefined;
        if (previous) { await previous.destroy(); }
      } catch (error) {
        if (candidate) { await candidate.destroy().catch(() => {}); }
        if (version === requested) { onError(error, Boolean(current)); }
      }
    });
    return sequence;
  };
}
