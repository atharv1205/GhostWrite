import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [clipboardText, setClipboardText] = useState("");
  const [rewrittenText, setRewrittenText] = useState("");
  const [loading, setLoading] = useState(false);
  const [copied, setCopied] = useState(false);
  const [selectedIndex, setSelectedIndex] = useState(0);

  const commands = [
    "Rewrite Selected Text",
    "Professional Tone",
    "Friendly Tone",
    "Summarize",
    "Fix Grammar",
  ];

  async function runCommand(mode: string) {
    try {
      setLoading(true);
      setCopied(false);

      const text = await invoke<string>(
        "get_clipboard_text"
      );

      setClipboardText(text);

      const improved = await invoke<string>(
        "rewrite_text",
        {
          text,
          mode,
        }
      );

      setRewrittenText(improved);

      await invoke(
        "set_clipboard_text",
        {
          text: improved,
        }
      );

      setCopied(true);

      setTimeout(() => {
        setCopied(false);
      }, 2000);

    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  }

  async function copyResult() {
    try {
      await invoke(
        "set_clipboard_text",
        {
          text: rewrittenText,
        }
      );

      setCopied(true);

      setTimeout(() => {
        setCopied(false);
      }, 2000);

    } catch (error) {
      console.error(error);
    }
  }

  function executeCommand(index: number) {
    switch (commands[index]) {
      case "Rewrite Selected Text":
        runCommand("rewrite");
        break;

      case "Professional Tone":
        runCommand("professional");
        break;

      case "Friendly Tone":
        runCommand("friendly");
        break;

      case "Summarize":
        runCommand("summarize");
        break;

      case "Fix Grammar":
        runCommand("grammar");
        break;
    }
  }

  useEffect(() => {
    const handleKeyDown = async (
      e: KeyboardEvent
    ) => {
      if (e.key === "Escape") {
        try {
          await invoke("hide_window");
        } catch (error) {
          console.error(error);
        }
      }

      if (e.key === "ArrowDown") {
        setSelectedIndex(
          (prev) => (prev + 1) % commands.length
        );
      }

      if (e.key === "ArrowUp") {
        setSelectedIndex(
          (prev) =>
            prev === 0
              ? commands.length - 1
              : prev - 1
        );
      }

      if (e.key === "Enter") {
        executeCommand(selectedIndex);
      }
    };

    window.addEventListener(
      "keydown",
      handleKeyDown
    );

    return () => {
      window.removeEventListener(
        "keydown",
        handleKeyDown
      );
    };
  }, [selectedIndex]);

  return (
    <main className="palette">
      <div className="header">
        <h2>GhostWrite</h2>
      </div>

      <div className="command-list">
        {commands.map((command, index) => (
          <button
            key={command}
            className={`command-item ${
              selectedIndex === index
                ? "selected"
                : ""
            }`}
            onClick={() =>
              executeCommand(index)
            }
          >
            {command}
          </button>
        ))}
      </div>

      {clipboardText && (
        <div
          style={{
            marginTop: "20px",
            padding: "12px",
            border: "1px solid #444",
            borderRadius: "8px",
            textAlign: "left",
          }}
        >
          <h3>Original Text</h3>
          <p>{clipboardText}</p>

          <h3>Improved Text</h3>

          {loading ? (
            <p>⏳ Rewriting with Qwen...</p>
          ) : (
            <>
              <p>{rewrittenText}</p>

              {rewrittenText && (
                <button
                  className="command-item"
                  onClick={copyResult}
                >
                  Copy Result
                </button>
              )}

              {copied && (
                <p
                  style={{
                    marginTop: "10px",
                  }}
                >
                  ✅ Result copied to clipboard
                </p>
              )}
            </>
          )}
        </div>
      )}
    </main>
  );
}

export default App;