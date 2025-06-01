// frontend/src/components/MemoryLane.jsx

import React, { useState, useEffect } from "react";

const mockMemories = [
  {
    id: 1,
    title: "Family Trip to Mountains",
    date: "2025-05-15",
    summary:
      "Captured breathtaking views and moments of laughter under blue skies.",
    imageUrl:
      "https://images.unsplash.com/photo-1506744038136-46273834b3fb?auto=format&fit=crop&w=800&q=80",
  },
  {
    id: 2,
    title: "Brain Teaser Challenge",
    date: "2025-06-01",
    summary:
      "Solved advanced puzzles that boosted cognitive skills with NeuroOS’s adaptive training.",
    imageUrl:
      "https://images.unsplash.com/photo-1557683316-973673baf926?auto=format&fit=crop&w=800&q=80",
  },
  {
    id: 3,
    title: "Evening Coding Session",
    date: "2025-06-02",
    summary:
      "Built new modules on the NeuroOS kernel, focusing on seamless LLM integration.",
    imageUrl:
      "https://images.unsplash.com/photo-1519389950473-47ba0277781c?auto=format&fit=crop&w=800&q=80",
  },
];

export default function MemoryLane() {
  const [memories, setMemories] = useState([]);

  useEffect(() => {
    // Fetch memories from backend or local DB in a real app
    setMemories(mockMemories);
  }, []);

  return (
    <div style={styles.container}>
      <h2 style={styles.header}>Memory Lane</h2>
      <div style={styles.grid}>
        {memories.map(({ id, title, date, summary, imageUrl }) => (
          <div key={id} style={styles.card}>
            <img src={imageUrl} alt={title} style={styles.image} />
            <div style={styles.content}>
              <h3 style={styles.title}>{title}</h3>
              <small style={styles.date}>{date}</small>
              <p style={styles.summary}>{summary}</p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

const styles = {
  container: {
    maxWidth: "900px",
    margin: "30px auto",
    fontFamily: "'Segoe UI', Tahoma, Geneva, Verdana, sans-serif",
    backgroundColor: "#f9fafb",
    padding: "20px",
    borderRadius: "14px",
    boxShadow: "0 6px 16px rgba(0,0,0,0.05)",
  },
  header: {
    fontSize: "26px",
    fontWeight: "700",
    marginBottom: "18px",
    color: "#223344",
  },
  grid: {
    display: "grid",
    gridTemplateColumns: "repeat(auto-fit, minmax(280px, 1fr))",
    gap: "20px",
  },
  card: {
    backgroundColor: "#fff",
    borderRadius: "12px",
    boxShadow: "0 4px 12px rgba(0,0,0,0.1)",
    overflow: "hidden",
    display: "flex",
    flexDirection: "column",
  },
  image: {
    width: "100%",
    height: "160px",
    objectFit: "cover",
  },
  content: {
    padding: "12px 16px",
  },
  title: {
    fontSize: "18px",
    margin: "8px 0 4px",
    color: "#1a1a1a",
  },
  date: {
    fontSize: "12px",
    color: "#888",
  },
  summary: {
    marginTop: "10px",
    fontSize: "14px",
    color: "#555",
  },
};
