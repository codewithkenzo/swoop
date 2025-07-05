import { useState } from 'react';
import { StreamingProgress } from '@/components/StreamingProgress';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { Activity, FileText, Globe, Zap } from 'lucide-react';

export function RealtimeMonitoring() {
  const [documentId, setDocumentId] = useState<string>('');
  const [crawlId, setCrawlId] = useState<string>('');
  const [activeDocuments, setActiveDocuments] = useState<string[]>([]);
  const [activeCrawls, setActiveCrawls] = useState<string[]>([]);

  const addDocumentStream = () => {
    if (documentId && !activeDocuments.includes(documentId)) {
      setActiveDocuments(prev => [...prev, documentId]);
      setDocumentId('');
    }
  };

  const addCrawlStream = () => {
    if (crawlId && !activeCrawls.includes(crawlId)) {
      setActiveCrawls(prev => [...prev, crawlId]);
      setCrawlId('');
    }
  };

  const removeDocumentStream = (id: string) => {
    setActiveDocuments(prev => prev.filter(docId => docId !== id));
  };

  const removeCrawlStream = (id: string) => {
    setActiveCrawls(prev => prev.filter(crawlId => crawlId !== id));
  };

  const exampleDocuments = [
    'doc_123456789',
    'doc_987654321',
    'doc_555666777'
  ];

  const exampleCrawls = [
    'crawl_abc123def',
    'crawl_xyz789ghi',
    'crawl_mno456pqr'
  ];

  return (
    <div className="space-y-6 p-6 max-w-7xl mx-auto">
      {/* Header */}
      <div className="text-center space-y-2">
        <h1 className="text-3xl font-bold tracking-tight flex items-center justify-center gap-2">
          <Activity className="h-8 w-8 text-blue-500" />
          Operations Monitor
        </h1>
        <p className="text-muted-foreground max-w-2xl mx-auto">
          Real-time monitoring and progress tracking for document processing and web crawling operations.
          Powered by Server-Sent Events (SSE) for instant updates.
        </p>
      </div>

      {/* Quick Start Cards */}
      <div className="grid gap-4 md:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <FileText className="h-5 w-5 text-blue-500" />
              Document Processing
            </CardTitle>
            <CardDescription>
              Track document analysis, categorization, and embedding generation in real-time
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex gap-2">
              <div className="flex-1">
                <Label htmlFor="documentId">Document ID</Label>
                <Input
                  id="documentId"
                  value={documentId}
                  onChange={(e) => setDocumentId(e.target.value)}
                  placeholder="Enter document ID..."
                />
              </div>
              <Button 
                onClick={addDocumentStream} 
                disabled={!documentId}
                className="mt-6"
              >
                Track
              </Button>
            </div>
            
            <div className="space-y-2">
              <Label>Example IDs:</Label>
              <div className="flex flex-wrap gap-2">
                {exampleDocuments.map(id => (
                  <Badge 
                    key={id}
                    variant="outline" 
                    className="cursor-pointer hover:bg-muted"
                    onClick={() => setDocumentId(id)}
                  >
                    {id}
                  </Badge>
                ))}
              </div>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              <Globe className="h-5 w-5 text-green-500" />
              Web Crawling
            </CardTitle>
            <CardDescription>
              Monitor website crawling progress, page discovery, and content extraction
            </CardDescription>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="flex gap-2">
              <div className="flex-1">
                <Label htmlFor="crawlId">Crawl Job ID</Label>
                <Input
                  id="crawlId"
                  value={crawlId}
                  onChange={(e) => setCrawlId(e.target.value)}
                  placeholder="Enter crawl job ID..."
                />
              </div>
              <Button 
                onClick={addCrawlStream} 
                disabled={!crawlId}
                className="mt-6"
              >
                Track
              </Button>
            </div>
            
            <div className="space-y-2">
              <Label>Example IDs:</Label>
              <div className="flex flex-wrap gap-2">
                {exampleCrawls.map(id => (
                  <Badge 
                    key={id}
                    variant="outline" 
                    className="cursor-pointer hover:bg-muted"
                    onClick={() => setCrawlId(id)}
                  >
                    {id}
                  </Badge>
                ))}
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Active Streams */}
      {(activeDocuments.length > 0 || activeCrawls.length > 0) && (
        <>
          <Separator />
          <div className="flex items-center gap-2">
            <Activity className="h-5 w-5 text-orange-500" />
            <h2 className="text-xl font-semibold">Active Streams</h2>
            <Badge variant="secondary">
              {activeDocuments.length + activeCrawls.length} running
            </Badge>
          </div>
        </>
      )}

      {/* Document Streams */}
      {activeDocuments.length > 0 && (
        <div className="space-y-4">
          <h3 className="text-lg font-medium text-blue-600">Document Processing</h3>
          <div className="grid gap-4 lg:grid-cols-2">
            {activeDocuments.map(id => (
              <div key={id} className="relative">
                <StreamingProgress
                  type="document"
                  id={id}
                  title={`Document ${id}`}
                  onComplete={() => {
                    console.log(`Document ${id} completed!`);
                    // Keep the stream for demo purposes
                  }}
                  onError={(error) => {
                    console.error(`Document ${id} error:`, error);
                  }}
                />
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => removeDocumentStream(id)}
                  className="absolute top-2 right-2"
                >
                  ×
                </Button>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Crawl Streams */}
      {activeCrawls.length > 0 && (
        <div className="space-y-4">
          <h3 className="text-lg font-medium text-green-600">Web Crawling</h3>
          <div className="grid gap-4 lg:grid-cols-2">
            {activeCrawls.map(id => (
              <div key={id} className="relative">
                <StreamingProgress
                  type="crawl"
                  id={id}
                  title={`Crawl Job ${id}`}
                  onComplete={() => {
                    console.log(`Crawl ${id} completed!`);
                    // Keep the stream for demo purposes
                  }}
                  onError={(error) => {
                    console.error(`Crawl ${id} error:`, error);
                  }}
                />
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => removeCrawlStream(id)}
                  className="absolute top-2 right-2"
                >
                  ×
                </Button>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Features Info */}
      <Card className="bg-gradient-to-r from-blue-50 to-green-50 border-dashed">
        <CardHeader>
          <CardTitle className="text-center">✨ Streaming Features</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid gap-4 md:grid-cols-3 text-sm">
            <div className="text-center space-y-2">
              <div className="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center mx-auto">
                <Activity className="h-4 w-4 text-blue-600" />
              </div>
              <h4 className="font-medium">Real-time Updates</h4>
              <p className="text-muted-foreground">
                Live progress via Server-Sent Events - no polling needed!
              </p>
            </div>
            <div className="text-center space-y-2">
              <div className="w-8 h-8 bg-green-100 rounded-full flex items-center justify-center mx-auto">
                <Zap className="h-4 w-4 text-green-600" />
              </div>
              <h4 className="font-medium">Auto-reconnect</h4>
              <p className="text-muted-foreground">
                Automatic retry with exponential backoff on connection loss
              </p>
            </div>
            <div className="text-center space-y-2">
              <div className="w-8 h-8 bg-purple-100 rounded-full flex items-center justify-center mx-auto">
                <FileText className="h-4 w-4 text-purple-600" />
              </div>
              <h4 className="font-medium">Multiple Streams</h4>
              <p className="text-muted-foreground">
                Track multiple operations simultaneously with clean UI
              </p>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Empty State */}
      {activeDocuments.length === 0 && activeCrawls.length === 0 && (
        <Card className="text-center py-12">
          <CardContent>
            <Activity className="h-12 w-12 text-muted-foreground mx-auto mb-4" />
            <h3 className="text-lg font-medium mb-2">No Active Streams</h3>
            <p className="text-muted-foreground mb-4">
              Add a document or crawl ID above to see real-time streaming in action!
            </p>
            <div className="flex justify-center gap-2">
              <Button 
                variant="outline" 
                onClick={() => setDocumentId(exampleDocuments[0])}
              >
                Try Example Document
              </Button>
              <Button 
                variant="outline" 
                onClick={() => setCrawlId(exampleCrawls[0])}
              >
                Try Example Crawl
              </Button>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
} 