import { useState } from 'react';
import { useChatStream } from '../hooks/useChatStream';

interface ChatStreamProps {}

export default function ChatStream(_props: ChatStreamProps) {
  const [input, setInput] = useState('');
  const { sendMessage, loading, conversation } = useChatStream();

  const handleSend = () => {
    sendMessage(input);
    setInput('');
  };

  return (
    <div className="max-w-xl mx-auto py-10">
      <h2 className="text-2xl font-bold mb-4">AI Chat (Streaming)</h2>
      <div className="border rounded p-4 h-60 overflow-y-auto whitespace-pre-wrap mb-4 bg-gray-50">
        {conversation || 'Assistant response will appear here…'}
      </div>
      <div className="flex gap-2">
        <input
          className="flex-1 border rounded px-3 py-2"
          placeholder="Ask something…"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === 'Enter') handleSend();
          }}
        />
        <button
          className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded"
          disabled={loading}
          onClick={handleSend}
        >
          {loading ? 'Streaming…' : 'Send'}
        </button>
      </div>
    </div>
  );
} 