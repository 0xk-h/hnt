import Card from "../components/Card";

const Demo = () => {
  return (
    <div className="min-h-screen bg-gradient-to-b from-[#0f172a] to-[#1e293b] text-white flex items-center justify-center p-8 cursor-default">
      <div className="max-w-2xl w-full">
        <div className="flex items-center gap-4 mb-8">
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
          <div className="bg-[#0f172a] px-4 py-3 rounded-lg font-mono text-sm text-green-400">
            src/App.jsx
          </div>
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
};

export default Demo;
