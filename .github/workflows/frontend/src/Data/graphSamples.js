// frontend/src/data/graphSamples.js

export const interactionGraph = {
  nodes: [
    { id: "prompt1", label: "Photo Sort", group: "command" },
    { id: "memory1", label: "Kyoto Trip", group: "memory" },
  ],
  links: [
    { source: "prompt1", target: "memory1", weight: 0.9 },
  ],
};
