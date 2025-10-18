import Card from "../../components/Card/Card";
import "./Demo.css";

const Demo: React.FC = () => {
  return (
    <div className="demo-container">
      <div className="demo-inner">
        <div className="demo-header">
          <h1>HNT Scaffold</h1>
        </div>

        <div className="welcome-card">
          <h2>You're all set!</h2>
          <p>
            Your React frontend is ready. Start building your application by
            editing the entry file below.
          </p>
          <div className="highlight-box">
            <span className="typewriter mini">src/App.tsx</span>
          </div>
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
