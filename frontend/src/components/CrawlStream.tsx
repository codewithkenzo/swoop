import { useCrawlStream, type CrawlStreamData } from '../hooks/useStreaming';
import { useState, useEffect } from 'react';
import { Badge } from './ui/badge';

export default function CrawlStream() {
  const [jobId, setJobId] = useState('');
  const [updates, setUpdates] = useState<CrawlStreamData[]>([]);

  const { data, error } = useCrawlStream(jobId || null);

  // Append new crawl update entries as they arrive
  useEffect(() => {
    if (data) {
      setUpdates(prev => [...prev, data]);
    }
  }, [data]);

  return (
    <div className="max-w-2xl mx-auto py-10">
      <h2 className="text-2xl font-bold mb-4">Crawl Stream</h2>
      <div className="flex gap-2 mb-4">
        <input
          className="flex-1 border rounded px-3 py-2"
          placeholder="Enter crawl job ID…"
          value={jobId}
          onChange={(e) => setJobId(e.target.value)}
        />
      </div>
      {error && <p className="text-red-600">{error}</p>}
      <div className="space-y-2 max-h-96 overflow-y-auto">
        {updates.map((u, idx) => (
          <div key={idx} className="border rounded p-3 text-sm bg-gray-50 flex justify-between">
            <span>{u.current_url ?? u.job_id}</span>
            <span>
              <Badge variant={u.status === 'completed' ? 'secondary' : 'default'}>
                {u.status}
              </Badge>
              <span className="ml-2">{u.urls_processed} URLs</span>
              {u.progress !== undefined && <span className="ml-2">{u.progress?.toFixed(0)}%</span>}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
} 