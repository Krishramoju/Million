// frontend/src/api/memories.js

import api from "./index";

export const fetchMemories = () => api.getMemories();

export const createMemory = (memoryData) => api.addMemory(memoryData);
