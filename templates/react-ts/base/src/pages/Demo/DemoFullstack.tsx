import { useEffect, useState } from "react";
import { fetchMessage } from "../../api/message";
import Card from "../../components/Card/Card";
import "./Demo.css";

const Demo: React.FC = () => {
  const [message, setMessage] = useState("Loading...");

  useEffect(() => {
    fetchMessage()
      .then((data) => setMessage(data.message))
      .catch(() => setMessage("Failed to load message"));
  }, []);

  return (
    <div className="demo-container">
      <div className="demo-inner">
        <div className="demo-header">
          <h1>HNT Scaffold</h1>
        </div>

        <div className="welcome-card">
          <h2>Frontend & Backend Connected</h2>
          <p>Your backend is running successfully! Here's what it says:</p>
          <div className="highlight-box">
            <span className="typewriter long">{message}</span>
          </div>
          <p className="api-note">
            (Fetched dynamically from your backend API at{" "}
            <code>/api/message</code>)
          </p>
        </div>

        <div className="card-grid">
          <Card
            title="Edit src/App.tsx"
            description="Your main application file. Modify it to start building."
          />
          <Card
            title="Documentation"
            description="Learn more about HNT CLI and scaffold options."
            href="https://github.com/0xk-h/hnt#readme"
          />
          <Card
            title="GitHub"
            description="View source or contribute."
            href="https://github.com/0xk-h/hnt"
          />
          <Card
            title="Report Issues"
            description="Found a bug? Let us know."
            href="https://github.com/0xk-h/hnt/issues"
          />
        </div>

        <p className="footer-text">Powered by HNT CLI</p>
      </div>
    </div>
  );
};

export default Demo;
