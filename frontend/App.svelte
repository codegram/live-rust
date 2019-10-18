<script>
  let stats;

  import("../pkg").then(({ Stats, decrease_stats }) => {
    stats = Stats.new();

    let start = null;

    const handleStats = timestamp => {
      const elapsedTime = timestamp - start;
      start = timestamp;
      decrease_stats(stats, elapsedTime / 1000);
      stats = stats; // pffffff
      window.requestAnimationFrame(handleStats);
    };

    window.requestAnimationFrame(handleStats);
  });
</script>

{#if stats}
  <ul>
    <li>🍗 {stats.food.value}</li>
    <li>💧 {stats.water.value}</li>
    <li>⚡️ {stats.energy.value}</li>
    <li>❤️ {stats.health.value}</li>
  </ul>
{/if}
