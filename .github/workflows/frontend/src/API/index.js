// frontend/src/api/index.js

const BASE_URL = process.env.REACT_APP_API_BASE_URL || "http://localhost:5000/api";

async function request(endpoint, options = {}) {
  const url = `${BASE_URL}${endpoint}`;
  const response = await fetch(url, {
    headers: {
      "Content-Type": "application/json",
      ...(options.headers || {}),
    },
    ...options,
  });
  if (!response.ok) {
    const error = await response.json().catch(() => ({}));
    throw new Error(error.message || `API error ${response.status}`);
  }
  return response.json();
}

export default {
  getMemories: () => request("/memories"),
  addMemory: (data) =>
    request("/memories", {
      method: "POST",
      body: JSON.stringify(data),
    }),
  getCortexResponse: (prompt) =>
    request("/cortex", {
      method: "POST",
      body: JSON.stringify({ prompt }),
    }),
  getTokenStats: () => request("/llm/token-stats"),
  // add other API methods here
};
