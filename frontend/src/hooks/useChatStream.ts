import { useState, useRef, useEffect, useCallback } from 'react';
import { API_BASE_URL } from '../lib/env';

interface ChatStreamOptions {
  onToken?: (token: string) => void;
  onDone?: () => void;
  onError?: (err: Error) => void;
  abortOnUnmount?: boolean;
}

export function useChatStream(options: ChatStreamOptions = {}) {
  const { onToken, onDone, onError, abortOnUnmount = true } = options;
  const [loading, setLoading] = useState(false);
  const [conversation, setConversation] = useState<string>('');

  const controllerRef = useRef<AbortController | null>(null);

  const sendMessage = useCallback(async (message: string) => {
    if (!message.trim()) return;

    controllerRef.current?.abort();
    const controller = new AbortController();
    controllerRef.current = controller;

    setLoading(true);

    try {
      const res = await fetch(`${API_BASE_URL}/chat/stream`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({ message }),
        signal: controller.signal
      });

      if (!res.ok || !res.body) {
        throw new Error(`HTTP ${res.status}`);
      }

      const reader = res.body.getReader();
      const textDecoder = new TextDecoder('utf-8');
      let partial = '';

      while (true) {
        const { value, done } = await reader.read();
        if (done) break;
        partial += textDecoder.decode(value, { stream: true });

        // Split by SSE event delimiter "\n\n"
        const parts = partial.split('\n\n');
        partial = parts.pop() || '';

        for (const evt of parts) {
          // Each event line may contain "data: ..."
          const dataLine = evt.split('\n').find(l => l.startsWith('data:'));
          if (!dataLine) continue;
          const payload = dataLine.replace(/^data:\s*/, '');
          try {
            const json = JSON.parse(payload);
            if (typeof json.content === 'string') {
              setConversation(prev => prev + json.content + ' ');
              onToken?.(json.content);
            }
          } catch (e) {
            console.warn('Invalid JSON chunk', e);
          }
        }
      }

      onDone?.();
    } catch (err: any) {
      if (err.name !== 'AbortError') {
        onError?.(err);
      }
    } finally {
      setLoading(false);
    }
  }, [onToken, onDone, onError]);

  const abort = useCallback(() => {
    controllerRef.current?.abort();
    controllerRef.current = null;
  }, []);

  useEffect(() => {
    return () => {
      if (abortOnUnmount) abort();
    };
  }, [abort, abortOnUnmount]);

  return { sendMessage, abort, loading, conversation };
} 