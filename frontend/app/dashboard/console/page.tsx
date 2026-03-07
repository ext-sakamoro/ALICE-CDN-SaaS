"use client";

import { useState } from "react";

const API_BASE = process.env.NEXT_PUBLIC_API_URL ?? "http://localhost:8081";

type Tab = "push" | "purge" | "latency" | "stats";

export default function ConsolePage() {
  const [tab, setTab] = useState<Tab>("push");
  const [result, setResult] = useState<string>("");
  const [loading, setLoading] = useState(false);

  // push
  const [pushAssetUrl, setPushAssetUrl] = useState("https://origin.example.com/logo.png");
  const [pushEdges, setPushEdges] = useState("us-east,eu-west,ap-south");

  // purge
  const [purgePattern, setPurgePattern] = useState("/assets/logo.png");
  const [purgeEdge, setPurgeEdge] = useState("all");

  // latency
  const [latencyFrom, setLatencyFrom] = useState("us-east");
  const [latencyTo, setLatencyTo] = useState("ap-south");

  const run = async () => {
    setLoading(true);
    setResult("");
    try {
      let res: Response;
      if (tab === "push") {
        res = await fetch(`${API_BASE}/api/v1/cdn/push`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            asset_url: pushAssetUrl,
            edges: pushEdges.split(",").map((e) => e.trim()).filter(Boolean),
          }),
        });
      } else if (tab === "purge") {
        res = await fetch(`${API_BASE}/api/v1/cdn/purge`, {
          method: "DELETE",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({
            pattern: purgePattern,
            edge: purgeEdge === "all" ? undefined : purgeEdge,
          }),
        });
      } else if (tab === "latency") {
        res = await fetch(`${API_BASE}/api/v1/cdn/latency`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ from: latencyFrom, to: latencyTo }),
        });
      } else {
        res = await fetch(`${API_BASE}/api/v1/cdn/stats`);
      }
      const json = await res.json();
      setResult(JSON.stringify(json, null, 2));
    } catch (e) {
      setResult(`Error: ${e instanceof Error ? e.message : String(e)}`);
    } finally {
      setLoading(false);
    }
  };

  const tabs: Tab[] = ["push", "purge", "latency", "stats"];

  return (
    <div className="min-h-screen bg-gray-900 text-green-400 p-6 font-mono">
      <h1 className="text-2xl font-bold mb-6 text-green-300">
        ALICE-CDN-SaaS Console
      </h1>

      {/* Tab bar */}
      <div className="flex gap-2 mb-6">
        {tabs.map((t) => (
          <button
            key={t}
            onClick={() => { setTab(t); setResult(""); }}
            className={`px-4 py-2 rounded text-sm font-semibold transition-colors ${
              tab === t
                ? "bg-green-600 text-gray-900"
                : "bg-gray-800 text-green-400 hover:bg-gray-700"
            }`}
          >
            {t.toUpperCase()}
          </button>
        ))}
      </div>

      {/* Tab content */}
      <div className="bg-gray-800 rounded-lg p-6 mb-6 space-y-4">
        {tab === "push" && (
          <>
            <div>
              <label className="block text-xs text-green-500 mb-1">Asset URL (origin)</label>
              <input
                value={pushAssetUrl}
                onChange={(e) => setPushAssetUrl(e.target.value)}
                className="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-green-400 text-sm"
              />
            </div>
            <div>
              <label className="block text-xs text-green-500 mb-1">
                Target Edges (comma-separated)
              </label>
              <input
                value={pushEdges}
                onChange={(e) => setPushEdges(e.target.value)}
                className="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-green-400 text-sm"
                placeholder="us-east,eu-west,ap-south"
              />
            </div>
          </>
        )}

        {tab === "purge" && (
          <>
            <div>
              <label className="block text-xs text-green-500 mb-1">
                URL Pattern (glob supported)
              </label>
              <input
                value={purgePattern}
                onChange={(e) => setPurgePattern(e.target.value)}
                className="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-green-400 text-sm"
                placeholder="/assets/*"
              />
            </div>
            <div>
              <label className="block text-xs text-green-500 mb-1">Edge Region</label>
              <select
                value={purgeEdge}
                onChange={(e) => setPurgeEdge(e.target.value)}
                className="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-green-400 text-sm"
              >
                <option value="all">All Regions</option>
                <option value="us-east">us-east</option>
                <option value="eu-west">eu-west</option>
                <option value="ap-south">ap-south</option>
              </select>
            </div>
          </>
        )}

        {tab === "latency" && (
          <>
            <div>
              <label className="block text-xs text-green-500 mb-1">From (Edge Region)</label>
              <input
                value={latencyFrom}
                onChange={(e) => setLatencyFrom(e.target.value)}
                className="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-green-400 text-sm"
              />
            </div>
            <div>
              <label className="block text-xs text-green-500 mb-1">To (Edge Region)</label>
              <input
                value={latencyTo}
                onChange={(e) => setLatencyTo(e.target.value)}
                className="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-green-400 text-sm"
              />
            </div>
            <p className="text-xs text-gray-500">
              Uses Vivaldi coordinate system to compute estimated inter-region latency.
            </p>
          </>
        )}

        {tab === "stats" && (
          <p className="text-green-500 text-sm">
            Fetches GET /api/v1/cdn/stats — click Run to retrieve edge
            bandwidth, cache hit rates, and region status.
          </p>
        )}
      </div>

      <button
        onClick={run}
        disabled={loading}
        className="px-6 py-2 bg-green-600 hover:bg-green-500 disabled:bg-gray-700 text-gray-900 font-bold rounded transition-colors"
      >
        {loading ? "Running..." : "Run"}
      </button>

      {result && (
        <pre className="mt-6 bg-gray-800 rounded-lg p-4 text-green-300 text-sm overflow-x-auto whitespace-pre-wrap">
          {result}
        </pre>
      )}
    </div>
  );
}
