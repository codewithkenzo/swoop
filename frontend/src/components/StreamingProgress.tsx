import React from 'react';
import { AlertCircle, CheckCircle, Loader2, Wifi, WifiOff } from 'lucide-react';
import { Progress } from './ui/progress';
import { Badge } from './ui/badge';
import { Card, CardContent, CardHeader, CardTitle } from './ui/card';
import { useDocumentStream, useCrawlStream, type ConnectionState } from '../hooks/useStreaming';

interface StreamingProgressProps {
  type: 'document' | 'crawl';
  id: string;
  title?: string;
  onComplete?: () => void;
  onError?: (error: string) => void;
}

function ConnectionIndicator({ state }: { state: ConnectionState }) {
  const icons = {
    connecting: <Loader2 className="h-4 w-4 animate-spin" />,
    connected: <Wifi className="h-4 w-4 text-green-500" />,
    error: <WifiOff className="h-4 w-4 text-red-500" />,
    closed: <WifiOff className="h-4 w-4 text-gray-400" />
  };

  const labels = {
    connecting: 'Connecting...',
    connected: 'Live',
    error: 'Connection Error',
    closed: 'Disconnected'
  };

  return (
    <div className="flex items-center gap-2 text-sm">
      {icons[state]}
      <span className={`
        ${state === 'connected' ? 'text-green-600' : ''}
        ${state === 'error' ? 'text-red-600' : ''}
        ${state === 'connecting' ? 'text-blue-600' : ''}
        ${state === 'closed' ? 'text-gray-500' : ''}
      `}>
        {labels[state]}
      </span>
    </div>
  );
}

export function StreamingProgress({ 
  type, 
  id, 
  title, 
  onComplete, 
  onError 
}: StreamingProgressProps) {
  const documentStream = useDocumentStream(
    type === 'document' ? id : null,
    {
      onError: (event) => {
        console.error('Document stream error:', event);
        onError?.('Failed to connect to document processing stream');
      }
    }
  );

  const crawlStream = useCrawlStream(
    type === 'crawl' ? id : null,
    {
      onError: (event) => {
        console.error('Crawl stream error:', event);
        onError?.('Failed to connect to crawl progress stream');
      }
    }
  );

  const stream = type === 'document' ? documentStream : crawlStream;
  const data = stream.data;

  // Handle completion
  React.useEffect(() => {
    if (data?.status === 'completed') {
      onComplete?.();
    } else if (data?.status === 'failed' && data.error) {
      onError?.(data.error);
    }
  }, [data?.status, data?.error, onComplete, onError]);

  const getStatusBadge = () => {
    if (!data) return <Badge variant="secondary">Waiting...</Badge>;
    
    switch (data.status) {
      case 'processing':
      case 'running':
        return <Badge variant="default" className="bg-blue-500">Processing</Badge>;
      case 'completed':
        return <Badge variant="default" className="bg-green-500">Completed</Badge>;
      case 'failed':
        return <Badge variant="destructive">Failed</Badge>;
      default:
        return <Badge variant="secondary">Unknown</Badge>;
    }
  };

  const getProgress = () => {
    if (!data) return 0;
    
    if (type === 'crawl' && 'pages_crawled' in data) {
      const crawlData = data as any;
      if (crawlData.total_pages) {
        return (crawlData.pages_crawled / crawlData.total_pages) * 100;
      }
      return crawlData.pages_crawled * 2; // Rough estimate when total unknown
    }
    
    return data.progress || 0;
  };

  const getDetails = () => {
    if (!data) return null;
    
    if (type === 'document') {
      const docData = data as any; // Type assertion for document-specific fields
      return (
        <div className="space-y-2">
          {docData.stage && (
            <p className="text-sm text-gray-600">
              Stage: <span className="font-medium">{docData.stage}</span>
            </p>
          )}
          {data.message && (
            <p className="text-sm text-gray-600">{data.message}</p>
          )}
        </div>
      );
    }
    
    if (type === 'crawl' && 'pages_crawled' in data) {
      const crawlData = data as any;
      return (
        <div className="space-y-2">
          <p className="text-sm text-gray-600">
            Pages crawled: <span className="font-medium">{crawlData.pages_crawled}</span>
            {crawlData.total_pages && ` of ${crawlData.total_pages}`}
          </p>
          {crawlData.current_url && (
            <p className="text-sm text-gray-600 truncate">
              Current: <span className="font-medium">{crawlData.current_url}</span>
            </p>
          )}
          {crawlData.message && (
            <p className="text-sm text-gray-600">{crawlData.message}</p>
          )}
        </div>
      );
    }
    
    return null;
  };

  return (
    <Card className="w-full">
      <CardHeader className="pb-3">
        <div className="flex items-center justify-between">
          <CardTitle className="text-lg">
            {title || `${type === 'document' ? 'Document Processing' : 'Web Crawling'}`}
          </CardTitle>
          <div className="flex items-center gap-3">
            {getStatusBadge()}
            <ConnectionIndicator state={stream.connectionState} />
          </div>
        </div>
      </CardHeader>
      <CardContent className="space-y-4">
        {/* Progress Bar */}
        <div className="space-y-2">
          <div className="flex justify-between text-sm">
            <span>Progress</span>
            <span>{Math.round(getProgress())}%</span>
          </div>
          <Progress value={getProgress()} className="h-2" />
        </div>

        {/* Details */}
        {getDetails()}

        {/* Error Display */}
        {data?.error && (
          <div className="flex items-start gap-2 p-3 bg-red-50 border border-red-200 rounded-md">
            <AlertCircle className="h-5 w-5 text-red-500 mt-0.5 flex-shrink-0" />
            <div>
              <p className="text-sm font-medium text-red-800">Error occurred</p>
              <p className="text-sm text-red-600">{data.error}</p>
            </div>
          </div>
        )}

        {/* Connection Error */}
        {stream.hasError && stream.error && (
          <div className="flex items-start gap-2 p-3 bg-yellow-50 border border-yellow-200 rounded-md">
            <WifiOff className="h-5 w-5 text-yellow-500 mt-0.5 flex-shrink-0" />
            <div>
              <p className="text-sm font-medium text-yellow-800">Connection Issue</p>
              <p className="text-sm text-yellow-600">{stream.error}</p>
              <button 
                onClick={stream.connect}
                className="text-sm text-yellow-700 underline hover:no-underline mt-1"
              >
                Try reconnecting
              </button>
            </div>
          </div>
        )}

        {/* Success */}
        {data?.status === 'completed' && (
          <div className="flex items-center gap-2 p-3 bg-green-50 border border-green-200 rounded-md">
            <CheckCircle className="h-5 w-5 text-green-500" />
            <p className="text-sm font-medium text-green-800">
              {type === 'document' ? 'Document processed successfully!' : 'Crawl completed successfully!'}
            </p>
          </div>
        )}

        {/* Timestamp */}
        {data?.timestamp && (
          <p className="text-xs text-gray-500">
            Last update: {new Date(data.timestamp).toLocaleTimeString()}
          </p>
        )}
      </CardContent>
    </Card>
  );
}

// Example usage component
export function StreamingDemo() {
  const [documentId, setDocumentId] = React.useState<string | null>(null);
  const [crawlId, setCrawlId] = React.useState<string | null>(null);

  return (
    <div className="space-y-6 p-6">
      <h2 className="text-2xl font-bold">Real-time Progress Tracking</h2>
      
      <div className="grid gap-6 md:grid-cols-2">
        {/* Document Processing Example */}
        <div className="space-y-4">
          <div className="flex gap-2">
            <input
              type="text"
              placeholder="Document ID"
              value={documentId || ''}
              onChange={(e) => setDocumentId(e.target.value || null)}
              className="flex-1 px-3 py-2 border rounded-md"
            />
            <button
              onClick={() => setDocumentId(null)}
              className="px-3 py-2 bg-gray-200 rounded-md hover:bg-gray-300"
            >
              Clear
            </button>
          </div>
          
          {documentId && (
            <StreamingProgress
              type="document"
              id={documentId}
              title="Document Processing"
              onComplete={() => console.log('Document processing completed!')}
              onError={(error) => console.error('Document error:', error)}
            />
          )}
        </div>

        {/* Crawl Progress Example */}
        <div className="space-y-4">
          <div className="flex gap-2">
            <input
              type="text"
              placeholder="Crawl ID"
              value={crawlId || ''}
              onChange={(e) => setCrawlId(e.target.value || null)}
              className="flex-1 px-3 py-2 border rounded-md"
            />
            <button
              onClick={() => setCrawlId(null)}
              className="px-3 py-2 bg-gray-200 rounded-md hover:bg-gray-300"
            >
              Clear
            </button>
          </div>
          
          {crawlId && (
            <StreamingProgress
              type="crawl"
              id={crawlId}
              title="Web Crawling"
              onComplete={() => console.log('Crawl completed!')}
              onError={(error) => console.error('Crawl error:', error)}
            />
          )}
        </div>
      </div>
    </div>
  );
} 