// frontend/src/components/IntentTimeline.jsx

import React, { useState, useEffect } from "react";

const mockIntentData = [
  {
    id: 1,
    timestamp: "2025-06-01T09:12:00Z",
    intent: "Open photo organizer",
    tags: ["app-launch", "photos"],
  },
  {
    id: 2,
    timestamp: "2025-06-01T09:15:23Z",
    intent: "Filter images with blurred faces",
    tags: ["filter", "image-processing", "privacy"],
  },
  {
    id: 3,
    timestamp: "2025-06-01T09:20:10Z",
    intent: "Send results to private gallery called 'memories'",
    tags: ["gallery", "privacy", "storage"],
  },
  {
    id: 4,
    timestamp: "2025-06-01T10:00:00Z",
    intent: "Start brain teaser module",
    tags: ["brain-teaser", "cognition"],
  },
];

export default function IntentTimeline() {
  const [intents, setIntents] = useState([]);

  useEffect(() => {
    // In real app, fetch intent timeline from Neurokernel backend or indexed DB
    // Here we simulate with mock data
    setIntents(mockIntentData);
  }, []);

  return (
    <div style={styles.container}>
      <h2 style={styles.header}>Intent Timeline</h2>
      <ul style={styles.list}>
        {intents.map(({ id, timestamp, intent, tags }) => (
          <li key={id} style={styles.listItem}>
            <div style={styles.timestamp}>
              {new Date(timestamp).toLocaleString()}
            </div>
            <div style={styles.intent}>{intent}</div>
            <div style={styles.tags}>
              {tags.map((tag) => (
                <span key={tag} style={styles.tag}>
                  {tag}
                </span>
              ))}
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
}

const styles = {
  container: {
    maxWidth: "600px",
    margin: "20px auto",
    fontFamily: "'Segoe UI', Tahoma, Geneva, Verdana, sans-serif",
    backgroundColor: "#fff",
    padding: "20px",
    borderRadius: "12px",
    boxShadow: "0 4px 10px rgba(0,0,0,0.1)",
  },
  header: {
    fontSize: "24px",
    fontWeight: "700",
    marginBottom: "12px",
    color: "#123456",
  },
  list: {
    listStyle: "none",
    padding: 0,
  },
  listItem: {
    borderBottom: "1px solid #eee",
    padding: "12px 0",
  },
  timestamp: {
    fontSize: "12px",
    color: "#999",
    marginBottom: "4px",
  },
  intent: {
    fontSize: "16px",
    fontWeight: "600",
    color: "#2E86AB",
  },
  tags: {
    marginTop: "6px",
  },
  tag: {
    display: "inline-block",
    backgroundColor: "#E0F0FF",
    color: "#2E86AB",
    fontSize: "12px",
    fontWeight: "600",
    borderRadius: "10px",
    padding: "2px 8px",
    marginRight: "6px",
  },
};
