import { useState } from 'react';
import { useDocumentStream } from '../hooks/useStreaming';
import { Badge } from './ui/badge';

export default function DocumentStream() {
  const [docId, setDocId] = useState('');
  const { data: update, error } = useDocumentStream(docId || null);

  return (
    <div className="max-w-2xl mx-auto py-10">
      <h2 className="text-2xl font-bold mb-4">Document Processing Stream</h2>
      <div className="flex gap-2 mb-4">
        <input
          className="flex-1 border rounded px-3 py-2"
          placeholder="Enter document ID…"
          value={docId}
          onChange={(e) => setDocId(e.target.value)}
        />
      </div>
      {error && <p className="text-red-600">{error}</p>}
      {update ? (
        <div className="border rounded p-4 bg-gray-50">
          <p>ID: {update.id}</p>
          <p>
            Status:{' '}
            <Badge variant={update.status === 'completed' ? 'secondary' : 'default'}>
              {update.status}
            </Badge>
          </p>
          {update.progress !== undefined && (
            <p>Progress: {update.progress.toFixed(0)}%</p>
          )}
          {update.stage && <p>Stage: {update.stage}</p>}
        </div>
      ) : (
        <p className="text-sm text-gray-500">Awaiting updates…</p>
      )}
    </div>
  );
} 