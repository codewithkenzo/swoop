import { useState, useEffect, useRef, useCallback } from 'react';
import { API_BASE_URL } from '../lib/env';

// Connection states for SSE
export type ConnectionState = 'connecting' | 'connected' | 'error' | 'closed';

// Document processing status from SSE
export interface DocumentStreamData {
  id: string;
  status: 'processing' | 'completed' | 'failed';
  progress?: number;
  message?: string;
  error?: string;
  stage?: string;
  timestamp: string;
}

// Crawl progress from SSE
export interface CrawlStreamData {
  job_id: string;
  status: 'running' | 'completed' | 'failed';
  urls_processed: number;
  successful_fetches: number;
  failed_fetches: number;
  bytes_downloaded: number;
  documents_extracted: number;
  links_discovered: number;
  avg_fetch_time_ms: number;
  start_time: string;
  end_time?: string;
  current_url?: string;
  progress?: number;
  message?: string;
  error?: string;
}

// Generic SSE hook options
interface UseSSEOptions {
  autoConnect?: boolean;
  retryAttempts?: number;
  retryDelay?: number;
  onError?: (error: Event) => void;
  onConnect?: () => void;
  onDisconnect?: () => void;
}

// Generic SSE hook
function useSSE<T>(
  url: string | null,
  options: UseSSEOptions = {}
) {
  const {
    autoConnect = true,
    retryAttempts = 3,
    retryDelay = 1000,
    onError,
    onConnect,
    onDisconnect
  } = options;

  const [data, setData] = useState<T | null>(null);
  const [connectionState, setConnectionState] = useState<ConnectionState>('closed');
  const [error, setError] = useState<string | null>(null);
  
  const eventSourceRef = useRef<EventSource | null>(null);
  const retryCountRef = useRef(0);
  const retryTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  const disconnect = useCallback(() => {
    if (eventSourceRef.current) {
      eventSourceRef.current.close();
      eventSourceRef.current = null;
    }
    if (retryTimeoutRef.current) {
      clearTimeout(retryTimeoutRef.current);
      retryTimeoutRef.current = null;
    }
    setConnectionState('closed');
    onDisconnect?.();
  }, [onDisconnect]);

  const connect = useCallback(() => {
    if (!url || eventSourceRef.current) return;

    setConnectionState('connecting');
    setError(null);

    const eventSource = new EventSource(url);
    eventSourceRef.current = eventSource;

    eventSource.onopen = () => {
      setConnectionState('connected');
      retryCountRef.current = 0;
      onConnect?.();
    };

    eventSource.onmessage = (event) => {
      try {
        const parsedData = JSON.parse(event.data) as T;
        setData(parsedData);
      } catch (err) {
        console.error('Failed to parse SSE data:', err);
        setError('Failed to parse server data');
      }
    };

    // Handle custom events
    eventSource.addEventListener('update', (event) => {
      try {
        const parsedData = JSON.parse(event.data) as T;
        setData(parsedData);
      } catch (err) {
        console.error('Failed to parse SSE update data:', err);
        setError('Failed to parse server data');
      }
    });

    eventSource.addEventListener('completed', (event) => {
      try {
        const parsedData = JSON.parse(event.data) as T;
        setData(parsedData);
        // Don't disconnect immediately - let the server send close event
      } catch (err) {
        console.error('Failed to parse SSE completed data:', err);
        setError('Failed to parse server data');
      }
    });

    eventSource.addEventListener('close', (event) => {
      // Server is closing the stream gracefully
      console.log('Stream closed gracefully by server');
      setConnectionState('closed');
      eventSource.close();
    });

    eventSource.addEventListener('error', (event) => {
      console.error('SSE error event:', event);
      setError('Server sent error event');
      // Don't auto-retry on server errors
    });

    eventSource.onerror = (event) => {
      // Only treat as connection error if it's not a graceful close
      if (eventSource.readyState === EventSource.CLOSED) {
        // Stream was closed, this is normal
        setConnectionState('closed');
        return;
      }
      
      setConnectionState('error');
      const errorMessage = 'Connection lost to server';
      setError(errorMessage);
      onError?.(event);

      // Auto-retry logic
      if (retryCountRef.current < retryAttempts) {
        retryCountRef.current++;
        console.log(`Retrying SSE connection (${retryCountRef.current}/${retryAttempts})...`);
        
        retryTimeoutRef.current = setTimeout(() => {
          disconnect();
          connect();
        }, retryDelay * retryCountRef.current); // Exponential backoff
      } else {
        disconnect();
      }
    };
  }, [url, retryAttempts, retryDelay, onError, onConnect, disconnect]);

  // Auto-connect when URL is provided
  useEffect(() => {
    if (autoConnect && url) {
      connect();
    }
    return disconnect;
  }, [url, autoConnect, connect, disconnect]);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      disconnect();
    };
  }, [disconnect]);

  return {
    data,
    connectionState,
    error,
    connect,
    disconnect,
    isConnecting: connectionState === 'connecting',
    isConnected: connectionState === 'connected',
    hasError: connectionState === 'error'
  };
}

// Document processing stream hook
export function useDocumentStream(documentId: string | null, options?: UseSSEOptions) {
  const url = documentId ? `${API_BASE_URL}/documents/${documentId}/stream` : null;
  
  return useSSE<DocumentStreamData>(url, {
    ...options,
    onError: (event) => {
      console.error('Document stream error:', event);
      options?.onError?.(event);
    }
  });
}

// Crawl progress stream hook
export function useCrawlStream(crawlId: string | null, options?: UseSSEOptions) {
  const url = crawlId ? `${API_BASE_URL}/api/crawl/${crawlId}/stream` : null;
  
  return useSSE<CrawlStreamData>(url, {
    ...options,
    onError: (event) => {
      console.error('Crawl stream error:', event);
      options?.onError?.(event);
    }
  });
}

// Helper hook for managing multiple streams
export function useMultipleStreams<T>(
  urls: (string | null)[],
  options?: UseSSEOptions
) {
  const streams = urls.map(url => useSSE<T>(url, options));
  
  const allConnected = streams.every(stream => stream.isConnected);
  const anyError = streams.some(stream => stream.hasError);
  const anyConnecting = streams.some(stream => stream.isConnecting);
  
  const disconnectAll = useCallback(() => {
    streams.forEach(stream => stream.disconnect());
  }, [streams]);
  
  const connectAll = useCallback(() => {
    streams.forEach(stream => stream.connect());
  }, [streams]);

  return {
    streams,
    allConnected,
    anyError,
    anyConnecting,
    disconnectAll,
    connectAll,
    data: streams.map(stream => stream.data),
    errors: streams.map(stream => stream.error)
  };
} 