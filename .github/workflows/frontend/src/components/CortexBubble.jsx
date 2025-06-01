// frontend/src/components/CortexBubble.jsx

import React, { useState, useEffect, useRef } from "react";

export default function CortexBubble() {
  const [messages, setMessages] = useState([
    { id: 0, from: "system", text: "Hello! I’m Cortex, your NeuroOS assistant." },
  ]);
  const [input, setInput] = useState("");
  const bottomRef = useRef(null);

  // Scroll to bottom on new message
  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  // Mock sending message to Neurokernel backend
  async function sendMessage(text) {
    setMessages((msgs) => [...msgs, { id: msgs.length, from: "user", text }]);
    setInput("");

    // Simulate backend processing delay
    setTimeout(() => {
      const reply = generateReply(text);
      setMessages((msgs) => [...msgs, { id: msgs.length, from: "cortex", text: reply }]);
    }, 1000);
  }

  // Mock reply logic (replace with real Neurokernel API call later)
  function generateReply(userText) {
    if (userText.toLowerCase().includes("memory")) {
      return "I see you want to explore your Memory Lane. Shall I open it?";
    } else if (userText.toLowerCase().includes("brain teaser")) {
      return "Here’s a puzzle for you: What has keys but can't open locks?";
    }
    return "Got it! I’m thinking...";
  }

  function handleSubmit(e) {
    e.preventDefault();
    if (input.trim()) sendMessage(input.trim());
  }

  return (
    <div style={styles.container}>
      <div style={styles.header}>Cortex Bubble</div>
      <div style={styles.messages}>
        {messages.map(({ id, from, text }) => (
          <div
            key={id}
            style={{
              ...styles.message,
              alignSelf: from === "user" ? "flex-end" : "flex-start",
              backgroundColor: from === "user" ? "#2E86AB" : "#F0F0F0",
              color: from === "user" ? "white" : "black",
            }}
          >
            {text}
          </div>
        ))}
        <div ref={bottomRef} />
      </div>
      <form onSubmit={handleSubmit} style={styles.form}>
        <input
          type="text"
          placeholder="Talk to Cortex..."
          value={input}
          onChange={(e) => setInput(e.target.value)}
          style={styles.input}
        />
        <button type="submit" style={styles.button}>Send</button>
      </form>
    </div>
  );
}

const styles = {
  container: {
    width: "300px",
    height: "400px",
    position: "fixed",
    bottom: "20px",
    right: "20px",
    boxShadow: "0 4px 12px rgba(0,0,0,0.2)",
    borderRadius: "12px",
    display: "flex",
    flexDirection: "column",
    backgroundColor: "white",
    fontFamily: "Segoe UI, Tahoma, Geneva, Verdana, sans-serif",
    zIndex: 9999,
  },
  header: {
    padding: "12px",
    borderBottom: "1px solid #ddd",
    fontWeight: "600",
    fontSize: "18px",
    backgroundColor: "#123456",
    color: "white",
    borderTopLeftRadius: "12px",
    borderTopRightRadius: "12px",
  },
  messages: {
    flex: 1,
    padding: "12px",
    overflowY: "auto",
    display: "flex",
    flexDirection: "column",
    gap: "8px",
  },
  message: {
    maxWidth: "70%",
    padding: "8px 12px",
    borderRadius: "18px",
    fontSize: "14px",
    lineHeight: "1.4",
  },
  form: {
    display: "flex",
    padding: "10px 12px",
    borderTop: "1px solid #ddd",
  },
  input: {
    flex: 1,
    padding: "8px 12px",
    fontSize: "14px",
    borderRadius: "18px",
    border: "1px solid #ccc",
    outline: "none",
  },
  button: {
    marginLeft: "8px",
    backgroundColor: "#2E86AB",
    border: "none",
    color: "white",
    padding: "8px 16px",
    fontWeight: "600",
    borderRadius: "18px",
    cursor: "pointer",
  },
};
