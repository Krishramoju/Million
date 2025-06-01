// frontend/src/api/cortex.js

import api from "./index";

export const sendPrompt = (prompt) => api.getCortexResponse(prompt);
