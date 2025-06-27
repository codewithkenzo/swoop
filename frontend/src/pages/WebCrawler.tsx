import { useState, useEffect, useRef, useCallback } from "react";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { Badge } from "@/components/ui/badge";
import { Progress } from "@/components/ui/progress";
import { Separator } from "@/components/ui/separator";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

import {
  Play,
  Square,
  Settings,
  Globe,
  Clock,
  FileText,
  CheckCircle,
  XCircle,
  AlertCircle,
  Download,
  Eye,
  Trash2,
  Activity,
} from "lucide-react";

interface CrawlResult {
  id: string;
  url: string;
  title: string;
  status: "success" | "error" | "pending";
  timestamp: Date;
  size: number;
  contentType: string;
}

interface CrawlSettings {
  startUrl: string;
  maxPages: number;
  crawlDepth: number;
  respectRobots: boolean;
  delay: number;
  userAgent: string;
  includePatterns: string;
  excludePatterns: string;
}

interface LogEntry {
  id: string;
  timestamp: Date;
  level: "info" | "warning" | "error" | "success";
  message: string;
}

interface CrawlStats {
  totalPages: number;
  successfulPages: number;
  failedPages: number;
  totalSize: number;
  avgResponseTime: number;
  startTime: Date | null;
  endTime: Date | null;
}

// Auto-scroll hook for live logs
const useAutoScroll = (content: any) => {
  const scrollRef = useRef<HTMLDivElement>(null);
  const [isAtBottom, setIsAtBottom] = useState(true);
  const [autoScrollEnabled, setAutoScrollEnabled] = useState(true);

  const scrollToBottom = useCallback(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, []);

  useEffect(() => {
    if (autoScrollEnabled && isAtBottom) {
      scrollToBottom();
    }
  }, [content, autoScrollEnabled, isAtBottom, scrollToBottom]);

  const handleScroll = useCallback(() => {
    if (scrollRef.current) {
      const { scrollTop, scrollHeight, clientHeight } = scrollRef.current;
      const atBottom = Math.abs(scrollHeight - scrollTop - clientHeight) < 10;
      setIsAtBottom(atBottom);
      setAutoScrollEnabled(atBottom);
    }
  }, []);

  useEffect(() => {
    const element = scrollRef.current;
    if (element) {
      element.addEventListener("scroll", handleScroll);
      return () => element.removeEventListener("scroll", handleScroll);
    }
  }, [handleScroll]);

  return { scrollRef, isAtBottom, autoScrollEnabled, scrollToBottom };
};

export default function WebCrawler() {
  const [isRunning, setIsRunning] = useState(false);
  const [settings, setSettings] = useState<CrawlSettings>({
    startUrl: "https://example.com",
    maxPages: 100,
    crawlDepth: 3,
    respectRobots: true,
    delay: 1000,
    userAgent: "Swoop Document Intelligence Bot",
    includePatterns: "",
    excludePatterns: "",
  });

  const [stats, setStats] = useState<CrawlStats>({
    totalPages: 0,
    successfulPages: 0,
    failedPages: 0,
    totalSize: 0,
    avgResponseTime: 0,
    startTime: null,
    endTime: null,
  });

  const [logs, setLogs] = useState<LogEntry[]>([
    {
      id: "1",
      timestamp: new Date(),
      level: "info",
      message: "Crawler initialized and ready to start",
    },
  ]);

  const [results, setResults] = useState<CrawlResult[]>([]);
  const [progress, setProgress] = useState(0);

  const { scrollRef: logsScrollRef } = useAutoScroll(logs);

  const addLog = useCallback((level: LogEntry["level"], message: string) => {
    const newLog: LogEntry = {
      id: Date.now().toString(),
      timestamp: new Date(),
      level,
      message,
    };
    setLogs((prev) => [...prev, newLog]);
  }, []);

  const startCrawl = useCallback(() => {
    setIsRunning(true);
    setStats((prev) => ({ ...prev, startTime: new Date(), endTime: null }));
    setProgress(0);
    setResults([]);
    addLog("info", `Starting crawl of ${settings.startUrl}`);
    addLog("info", `Configuration: Max pages: ${settings.maxPages}, Depth: ${settings.crawlDepth}`);

    // Simulate crawling (replace with real API integration)
    let currentProgress = 0;
    const interval = setInterval(() => {
      currentProgress += Math.random() * 10;
      if (currentProgress >= 100) {
        currentProgress = 100;
        clearInterval(interval);
        setIsRunning(false);
        setStats((prev) => ({ ...prev, endTime: new Date() }));
        addLog("success", "Crawl completed successfully");
      }

      setProgress(currentProgress);

      // Simulate discovering pages
      if (Math.random() > 0.7) {
        const newResult: CrawlResult = {
          id: Date.now().toString(),
          url: `${settings.startUrl}/page-${Math.floor(Math.random() * 1000)}`,
          title: `Sample Page ${Math.floor(Math.random() * 1000)}`,
          status: Math.random() > 0.1 ? "success" : "error",
          timestamp: new Date(),
          size: Math.floor(Math.random() * 50000) + 1000,
          contentType: "text/html",
        };

        setResults((prev) => [...prev, newResult]);
        setStats((prev) => ({
          ...prev,
          totalPages: prev.totalPages + 1,
          successfulPages: newResult.status === "success" ? prev.successfulPages + 1 : prev.successfulPages,
          failedPages: newResult.status === "error" ? prev.failedPages + 1 : prev.failedPages,
          totalSize: prev.totalSize + newResult.size,
          avgResponseTime: Math.floor(Math.random() * 500) + 100,
        }));

        addLog(newResult.status === "success" ? "info" : "error", `${newResult.status === "success" ? "Successfully crawled" : "Failed to crawl"}: ${newResult.url}`);
      }
    }, 500);
  }, [settings, addLog]);

  const stopCrawl = useCallback(() => {
    setIsRunning(false);
    setStats((prev) => ({ ...prev, endTime: new Date() }));
    addLog("warning", "Crawl stopped by user");
  }, [addLog]);

  const clearLogs = useCallback(() => {
    setLogs([]);
    addLog("info", "Logs cleared");
  }, [addLog]);

  const clearResults = useCallback(() => {
    setResults([]);
    addLog("info", "Results cleared");
  }, [addLog]);

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return "0 Bytes";
    const k = 1024;
    const sizes = ["Bytes", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  };

  const formatDuration = (start: Date | null, end: Date | null) => {
    if (!start) return "0s";
    const endTime = end || new Date();
    const duration = Math.floor((endTime.getTime() - start.getTime()) / 1000);
    return `${duration}s`;
  };

  const getLogIcon = (level: LogEntry["level"]) => {
    switch (level) {
      case "success":
        return <CheckCircle className="h-4 w-4 text-green-500" />;
      case "error":
        return <XCircle className="h-4 w-4 text-red-500" />;
      case "warning":
        return <AlertCircle className="h-4 w-4 text-yellow-500" />;
      default:
        return <Activity className="h-4 w-4 text-blue-500" />;
    }
  };

  return (
    <div className="min-h-screen bg-background p-6">
      <div className="mx-auto max-w-7xl space-y-6">
        {/* Header */}
        <div className="flex items-center justify-between">
          <div>
            <h1 className="text-3xl font-bold text-foreground">Web Crawler</h1>
            <p className="text-muted-foreground">Swoop Document Intelligence Platform</p>
          </div>
          <div className="flex items-center gap-3">
            <Button onClick={isRunning ? stopCrawl : startCrawl} variant={isRunning ? "destructive" : "default"} size="lg" disabled={!settings.startUrl}>
              {isRunning ? (
                <>
                  <Square className="mr-2 h-4 w-4" /> Stop Crawl
                </>
              ) : (
                <>
                  <Play className="mr-2 h-4 w-4" /> Start Crawl
                </>
              )}
            </Button>
          </div>
        </div>

        {/* Stats */}
        <div className="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-4">
          <Card>
            <CardContent className="p-4">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-muted-foreground">Total Pages</p>
                  <p className="text-2xl font-bold">{stats.totalPages}</p>
                </div>
                <Globe className="h-8 w-8 text-blue-500" />
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardContent className="p-4">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-muted-foreground">Success Rate</p>
                  <p className="text-2xl font-bold">
                    {stats.totalPages > 0 ? Math.round((stats.successfulPages / stats.totalPages) * 100) : 0}%
                  </p>
                </div>
                <CheckCircle className="h-8 w-8 text-green-500" />
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardContent className="p-4">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-muted-foreground">Data Collected</p>
                  <p className="text-2xl font-bold">{formatBytes(stats.totalSize)}</p>
                </div>
                <FileText className="h-8 w-8 text-purple-500" />
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardContent className="p-4">
              <div className="flex items-center justify-between">
                <div>
                  <p className="text-sm text-muted-foreground">Duration</p>
                  <p className="text-2xl font-bold">{formatDuration(stats.startTime, stats.endTime)}</p>
                </div>
                <Clock className="h-8 w-8 text-orange-500" />
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Progress */}
        {isRunning && (
          <Card>
            <CardContent className="p-4">
              <div className="space-y-2">
                <div className="flex items-center justify-between">
                  <span className="text-sm font-medium">Crawl Progress</span>
                  <span className="text-sm text-muted-foreground">{Math.round(progress)}%</span>
                </div>
                <Progress value={progress} className="h-2" />
              </div>
            </CardContent>
          </Card>
        )}

        <div className="grid grid-cols-1 gap-6 lg:grid-cols-3">
          {/* Settings */}
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Settings className="h-5 w-5" /> Crawler Settings
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-2">
                <Label htmlFor="startUrl">Start URL</Label>
                <Input id="startUrl" value={settings.startUrl} onChange={(e) => setSettings((prev) => ({ ...prev, startUrl: e.target.value }))} placeholder="https://example.com" disabled={isRunning} />
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div className="space-y-2">
                  <Label htmlFor="maxPages">Max Pages</Label>
                  <Input id="maxPages" type="number" value={settings.maxPages} onChange={(e) => setSettings((prev) => ({ ...prev, maxPages: parseInt(e.target.value) || 0 }))} disabled={isRunning} />
                </div>
                <div className="space-y-2">
                  <Label htmlFor="crawlDepth">Depth</Label>
                  <Input id="crawlDepth" type="number" value={settings.crawlDepth} onChange={(e) => setSettings((prev) => ({ ...prev, crawlDepth: parseInt(e.target.value) || 0 }))} disabled={isRunning} />
                </div>
              </div>

              <div className="space-y-2">
                <Label htmlFor="delay">Delay (ms)</Label>
                <Input id="delay" type="number" value={settings.delay} onChange={(e) => setSettings((prev) => ({ ...prev, delay: parseInt(e.target.value) || 0 }))} disabled={isRunning} />
              </div>

              <div className="space-y-2">
                <Label htmlFor="includePatterns">Include Patterns</Label>
                <Textarea id="includePatterns" value={settings.includePatterns} onChange={(e) => setSettings((prev) => ({ ...prev, includePatterns: e.target.value }))} placeholder="*.pdf, /docs/*" disabled={isRunning} rows={2} />
              </div>
            </CardContent>
          </Card>

          {/* Logs */}
          <Card>
            <CardHeader>
              <div className="flex items-center justify-between">
                <CardTitle className="flex items-center gap-2">
                  <Activity className="h-5 w-5" /> Live Logs
                </CardTitle>
                <Button variant="outline" size="sm" onClick={clearLogs} disabled={isRunning}>
                  <Trash2 className="h-4 w-4" />
                </Button>
              </div>
            </CardHeader>
            <CardContent>
              <ScrollArea className="h-[400px]">
                <div ref={logsScrollRef} className="space-y-2">
                  {logs.map((log) => (
                    <div key={log.id} className="flex items-start gap-2 text-sm">
                      {getLogIcon(log.level)}
                      <div className="min-w-0 flex-1">
                        <div className="flex items-center gap-2">
                          <span className="text-xs text-muted-foreground">{log.timestamp.toLocaleTimeString()}</span>
                          <Badge variant="outline" className="text-xs">
                            {log.level}
                          </Badge>
                        </div>
                        <p className="break-words text-foreground">{log.message}</p>
                      </div>
                    </div>
                  ))}
                </div>
              </ScrollArea>
            </CardContent>
          </Card>

          {/* Quick Stats */}
          <Card>
            <CardHeader>
              <CardTitle>Session Statistics</CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="space-y-3">
                <div className="flex justify-between">
                  <span className="text-sm text-muted-foreground">Successful</span>
                  <span className="text-sm font-medium text-green-600">{stats.successfulPages}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-sm text-muted-foreground">Failed</span>
                  <span className="text-sm font-medium text-red-600">{stats.failedPages}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-sm text-muted-foreground">Avg Response</span>
                  <span className="text-sm font-medium">{stats.avgResponseTime}ms</span>
                </div>
                <Separator />
                <div className="flex justify-between">
                  <span className="text-sm text-muted-foreground">Total Size</span>
                  <span className="text-sm font-medium">{formatBytes(stats.totalSize)}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-sm text-muted-foreground">Status</span>
                  <Badge variant={isRunning ? "default" : "secondary"}>{isRunning ? "Running" : "Idle"}</Badge>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Results */}
        <Card>
          <CardHeader>
            <div className="flex items-center justify-between">
              <CardTitle className="flex items-center gap-2">
                <FileText className="h-5 w-5" /> Crawl Results ({results.length})
              </CardTitle>
              <div className="flex items-center gap-2">
                <Button variant="outline" size="sm" onClick={clearResults} disabled={isRunning}>
                  <Trash2 className="mr-2 h-4 w-4" /> Clear
                </Button>
                <Button variant="outline" size="sm" disabled={results.length === 0}>
                  <Download className="mr-2 h-4 w-4" /> Export
                </Button>
              </div>
            </div>
          </CardHeader>
          <CardContent>
            <div className="rounded-md border">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Status</TableHead>
                    <TableHead>URL</TableHead>
                    <TableHead>Title</TableHead>
                    <TableHead>Size</TableHead>
                    <TableHead>Timestamp</TableHead>
                    <TableHead>Actions</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {results.length === 0 ? (
                    <TableRow>
                      <TableCell colSpan={6} className="py-8 text-center text-muted-foreground">
                        No results yet. Start a crawl to see data here.
                      </TableCell>
                    </TableRow>
                  ) : (
                    results.slice(-10).map((result) => (
                      <TableRow key={result.id}>
                        <TableCell>
                          <Badge variant={result.status === "success" ? "default" : "destructive"}>
                            {result.status === "success" ? <CheckCircle className="mr-1 h-3 w-3" /> : <XCircle className="mr-1 h-3 w-3" />} {result.status}
                          </Badge>
                        </TableCell>
                        <TableCell className="max-w-xs truncate" title={result.url}>
                          {result.url}
                        </TableCell>
                        <TableCell className="max-w-xs truncate" title={result.title}>
                          {result.title}
                        </TableCell>
                        <TableCell>{formatBytes(result.size)}</TableCell>
                        <TableCell>{result.timestamp.toLocaleTimeString()}</TableCell>
                        <TableCell>
                          <Button variant="ghost" size="sm">
                            <Eye className="h-4 w-4" />
                          </Button>
                        </TableCell>
                      </TableRow>
                    ))
                  )}
                </TableBody>
              </Table>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
} 