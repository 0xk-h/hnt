import { Code2, ExternalLink } from "lucide-react";

function App() {
  return (
    <div className="min-h-screen bg-gradient-to-br from-[#0f172a] to-[#1e293b] text-white flex items-center justify-center p-8 cursor-default">
      <div className="max-w-2xl w-full">
        <div className="flex items-center gap-4 mb-8">
          <Code2 className="w-16 h-16 text-blue-400" />
          <h1 className="text-5xl font-bold">HNT Scaffold</h1>
        </div>

        <div className="bg-[#1e293b] rounded-lg p-8 border border-slate-700 mb-6 shadow-lg">
          <h2 className="text-2xl font-semibold mb-4 text-slate-200">
            You're all set!
          </h2>
          <p className="text-slate-400 mb-4">
            Your React frontend is ready. Start building your application by
            editing the entry file below.
          </p>
          <CodeBlock>src/App.jsx</CodeBlock>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <Card
            title="Edit src/App.jsx"
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

        <p className="text-center text-slate-500 mt-8 text-sm">
          Powered by HNT CLI
        </p>
      </div>
    </div>
  );
}

function CodeBlock({ children }) {
  return (
    <div className="bg-[#0f172a] px-4 py-3 rounded border border-slate-700 font-mono text-sm text-green-400">
      {children}
    </div>
  );
}

function Card({ title, description, href }) {
  return (
    <a
      href={href || "#"}
      target={href ? "_blank" : "_self"}
      rel="noreferrer"
      className="block bg-[#1e293b] rounded-lg p-6 border border-slate-700 hover:border-blue-500 transition-colors group shadow-sm"
    >
      <div className="flex items-start justify-between mb-2">
        <h3 className="text-lg font-semibold text-slate-200">{title}</h3>
        {href && (
          <ExternalLink className="w-4 h-4 text-slate-500 group-hover:text-blue-400 transition-colors" />
        )}
      </div>
      <p className="text-slate-400 text-sm">{description}</p>
    </a>
  );
}

export default App;
