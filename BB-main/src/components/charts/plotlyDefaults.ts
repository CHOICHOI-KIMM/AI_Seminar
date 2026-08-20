export const darkLayout: Partial<Plotly.Layout> = {
  paper_bgcolor: 'transparent',
  plot_bgcolor: 'transparent',
  font: { family: 'Inter, sans-serif', size: 13, color: '#94a3b8' },
  margin: { l: 55, r: 20, t: 35, b: 45 },
  xaxis: {
    gridcolor: '#1e293b',
    zerolinecolor: '#334155',
    tickfont: { family: 'JetBrains Mono, monospace', size: 12 },
  },
  yaxis: {
    gridcolor: '#1e293b',
    zerolinecolor: '#334155',
    tickfont: { family: 'JetBrains Mono, monospace', size: 12 },
  },
  showlegend: false,
};

export const plotConfig: Partial<Plotly.Config> = {
  displayModeBar: false,
  responsive: true,
};

// Rainbow color scale for contour maps
export const viridisScale: [number, string][] = [
  [0, '#0000FF'],
  [0.125, '#0080FF'],
  [0.25, '#00FFFF'],
  [0.375, '#00FF80'],
  [0.5, '#00FF00'],
  [0.625, '#80FF00'],
  [0.75, '#FFFF00'],
  [0.875, '#FF8000'],
  [1, '#FF0000'],
];
