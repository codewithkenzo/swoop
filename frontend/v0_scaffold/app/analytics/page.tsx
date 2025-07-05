"use client"

import { useState } from "react"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Progress } from "@/components/ui/progress"
import {
  BarChart,
  Bar,
  LineChart,
  Line,
  PieChart,
  Pie,
  Cell,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
  AreaChart,
  Area,
  RadialBarChart,
  RadialBar,
} from "recharts"
import {
  AlertTriangle,
  CheckCircle,
  Download,
  FileText,
  Globe,
  Zap,
  Server,
  HardDrive,
  Cpu,
  AlertCircle,
} from "lucide-react"

// Mock data for analytics
const processingMetrics = [
  { time: "00:00", throughput: 45, errors: 2, quality: 0.94 },
  { time: "01:00", throughput: 52, errors: 1, quality: 0.96 },
  { time: "02:00", throughput: 38, errors: 3, quality: 0.92 },
  { time: "03:00", throughput: 61, errors: 0, quality: 0.98 },
  { time: "04:00", throughput: 47, errors: 2, quality: 0.95 },
  { time: "05:00", throughput: 55, errors: 1, quality: 0.97 },
  { time: "06:00", throughput: 42, errors: 4, quality: 0.89 },
  { time: "07:00", throughput: 58, errors: 1, quality: 0.96 },
  { time: "08:00", throughput: 63, errors: 0, quality: 0.98 },
  { time: "09:00", throughput: 49, errors: 2, quality: 0.94 },
  { time: "10:00", throughput: 67, errors: 1, quality: 0.97 },
  { time: "11:00", throughput: 54, errors: 3, quality: 0.91 },
]

const documentTypes = [
  { name: "PDF", value: 45, count: 567, color: "#3b82f6" },
  { name: "DOCX", value: 25, count: 315, color: "#10b981" },
  { name: "TXT", value: 15, count: 189, color: "#f59e0b" },
  { name: "PPTX", value: 10, count: 126, color: "#ef4444" },
  { name: "XLSX", value: 5, count: 63, color: "#8b5cf6" },
]

const languageStats = [
  { language: "English", count: 856, percentage: 68.2 },
  { language: "Spanish", count: 142, percentage: 11.3 },
  { language: "French", count: 98, percentage: 7.8 },
  { language: "German", count: 76, percentage: 6.1 },
  { language: "Chinese", count: 54, percentage: 4.3 },
  { language: "Other", count: 29, percentage: 2.3 },
]

const systemHealth = [
  { metric: "CPU Usage", value: 68, status: "good", color: "#10b981" },
  { metric: "Memory", value: 74, status: "warning", color: "#f59e0b" },
  { metric: "Disk Space", value: 45, status: "good", color: "#10b981" },
  { metric: "Network", value: 92, status: "excellent", color: "#3b82f6" },
]

const processingTimes = [
  { range: "0-10s", count: 234, percentage: 18.7 },
  { range: "10-30s", count: 456, percentage: 36.4 },
  { range: "30-60s", count: 312, percentage: 24.9 },
  { range: "1-5m", count: 189, percentage: 15.1 },
  { range: "5m+", count: 63, percentage: 5.0 },
]

const qualityTrends = [
  { date: "2024-01-01", avgQuality: 0.89, documents: 45 },
  { date: "2024-01-02", avgQuality: 0.92, documents: 52 },
  { date: "2024-01-03", avgQuality: 0.88, documents: 38 },
  { date: "2024-01-04", avgQuality: 0.95, documents: 61 },
  { date: "2024-01-05", avgQuality: 0.91, documents: 47 },
  { date: "2024-01-06", avgQuality: 0.94, documents: 55 },
  { date: "2024-01-07", avgQuality: 0.87, documents: 42 },
]

const contentClassification = [
  { category: "Financial", count: 234, percentage: 18.7, color: "#3b82f6" },
  { category: "Legal", count: 189, percentage: 15.1, color: "#10b981" },
  { category: "Technical", count: 156, percentage: 12.4, color: "#f59e0b" },
  { category: "Marketing", count: 134, percentage: 10.7, color: "#ef4444" },
  { category: "HR", count: 98, percentage: 7.8, color: "#8b5cf6" },
  { category: "Other", count: 444, percentage: 35.4, color: "#6b7280" },
]

const piiDetection = [
  { type: "Email Addresses", detected: 1247, documents: 456 },
  { type: "Phone Numbers", detected: 892, documents: 234 },
  { type: "SSN/Tax IDs", detected: 156, documents: 89 },
  { type: "Credit Cards", detected: 67, documents: 34 },
  { type: "Addresses", detected: 445, documents: 178 },
]

function MetricCard({
  title,
  value,
  change,
  icon: Icon,
  trend,
}: {
  title: string
  value: string | number
  change?: string
  icon: any
  trend?: "up" | "down" | "neutral"
}) {
  return (
    <Card>
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle className="text-sm font-medium">{title}</CardTitle>
        <Icon className="h-4 w-4 text-muted-foreground" />
      </CardHeader>
      <CardContent>
        <div className="text-2xl font-bold">{value}</div>
        {change && (
          <p className="text-xs text-muted-foreground">
            <span className={trend === "up" ? "text-green-600" : trend === "down" ? "text-red-600" : ""}>{change}</span>
            {" from last period"}
          </p>
        )}
      </CardContent>
    </Card>
  )
}

function ProcessingMetricsChart() {
  return (
    <Card className="col-span-2">
      <CardHeader>
        <CardTitle>Real-time Processing Metrics</CardTitle>
        <CardDescription>Throughput and quality trends over the last 12 hours</CardDescription>
      </CardHeader>
      <CardContent>
        <ResponsiveContainer width="100%" height={300}>
          <LineChart data={processingMetrics}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="time" />
            <YAxis yAxisId="left" />
            <YAxis yAxisId="right" orientation="right" domain={[0.8, 1]} />
            <Tooltip />
            <Legend />
            <Bar yAxisId="left" dataKey="throughput" fill="#3b82f6" name="Documents/Hour" />
            <Line
              yAxisId="right"
              type="monotone"
              dataKey="quality"
              stroke="#10b981"
              name="Avg Quality"
              strokeWidth={2}
            />
          </LineChart>
        </ResponsiveContainer>
      </CardContent>
    </Card>
  )
}

function DocumentTypeChart() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Document Types</CardTitle>
        <CardDescription>Distribution of processed document formats</CardDescription>
      </CardHeader>
      <CardContent>
        <ResponsiveContainer width="100%" height={300}>
          <PieChart>
            <Pie
              data={documentTypes}
              cx="50%"
              cy="50%"
              labelLine={false}
              label={({ name, percentage }) => `${name} ${percentage}%`}
              outerRadius={80}
              fill="#8884d8"
              dataKey="value"
            >
              {documentTypes.map((entry, index) => (
                <Cell key={`cell-${index}`} fill={entry.color} />
              ))}
            </Pie>
            <Tooltip />
          </PieChart>
        </ResponsiveContainer>
      </CardContent>
    </Card>
  )
}

function SystemHealthGauges() {
  return (
    <Card className="col-span-2">
      <CardHeader>
        <CardTitle>System Health</CardTitle>
        <CardDescription>Real-time system performance indicators</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          {systemHealth.map((metric) => (
            <div key={metric.metric} className="text-center">
              <div className="relative w-24 h-24 mx-auto mb-2">
                <ResponsiveContainer width="100%" height="100%">
                  <RadialBarChart cx="50%" cy="50%" innerRadius="60%" outerRadius="90%" data={[metric]}>
                    <RadialBar dataKey="value" cornerRadius={10} fill={metric.color} />
                  </RadialBarChart>
                </ResponsiveContainer>
                <div className="absolute inset-0 flex items-center justify-center">
                  <span className="text-lg font-bold">{metric.value}%</span>
                </div>
              </div>
              <p className="text-sm font-medium">{metric.metric}</p>
              <Badge
                variant={metric.status === "good" ? "default" : metric.status === "warning" ? "secondary" : "outline"}
              >
                {metric.status}
              </Badge>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  )
}

function ProcessingTimeHistogram() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Processing Time Distribution</CardTitle>
        <CardDescription>Histogram of document processing times</CardDescription>
      </CardHeader>
      <CardContent>
        <ResponsiveContainer width="100%" height={300}>
          <BarChart data={processingTimes}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="range" />
            <YAxis />
            <Tooltip />
            <Bar dataKey="count" fill="#8b5cf6" />
          </BarChart>
        </ResponsiveContainer>
      </CardContent>
    </Card>
  )
}

function QualityTrendsChart() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Quality Score Trends</CardTitle>
        <CardDescription>Average quality scores over time</CardDescription>
      </CardHeader>
      <CardContent>
        <ResponsiveContainer width="100%" height={300}>
          <AreaChart data={qualityTrends}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="date" />
            <YAxis domain={[0.8, 1]} />
            <Tooltip />
            <Area type="monotone" dataKey="avgQuality" stroke="#10b981" fill="#10b981" fillOpacity={0.3} />
          </AreaChart>
        </ResponsiveContainer>
      </CardContent>
    </Card>
  )
}

function LanguageStatsTable() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Language Detection</CardTitle>
        <CardDescription>Detected languages in processed documents</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="space-y-3">
          {languageStats.map((lang) => (
            <div key={lang.language} className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Globe className="h-4 w-4 text-muted-foreground" />
                <span className="font-medium">{lang.language}</span>
              </div>
              <div className="flex items-center gap-2">
                <div className="w-24">
                  <Progress value={lang.percentage} className="h-2" />
                </div>
                <span className="text-sm text-muted-foreground w-12">{lang.count}</span>
              </div>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  )
}

function ContentClassificationChart() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>Content Classification</CardTitle>
        <CardDescription>Automatic document categorization results</CardDescription>
      </CardHeader>
      <CardContent>
        <ResponsiveContainer width="100%" height={300}>
          <BarChart data={contentClassification} layout="horizontal">
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis type="number" />
            <YAxis dataKey="category" type="category" width={80} />
            <Tooltip />
            <Bar dataKey="count" fill="#3b82f6" />
          </BarChart>
        </ResponsiveContainer>
      </CardContent>
    </Card>
  )
}

function PIIDetectionSummary() {
  return (
    <Card>
      <CardHeader>
        <CardTitle>PII Detection Summary</CardTitle>
        <CardDescription>Personally identifiable information found in documents</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          {piiDetection.map((pii) => (
            <div key={pii.type} className="flex items-center justify-between p-3 border rounded-lg">
              <div>
                <p className="font-medium">{pii.type}</p>
                <p className="text-sm text-muted-foreground">{pii.documents} documents affected</p>
              </div>
              <div className="text-right">
                <p className="text-2xl font-bold text-orange-600">{pii.detected}</p>
                <p className="text-xs text-muted-foreground">instances</p>
              </div>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  )
}

function AlertsPanel() {
  const alerts = [
    {
      id: 1,
      type: "error",
      message: "High error rate detected in PDF processing",
      time: "2 minutes ago",
      severity: "high",
    },
    {
      id: 2,
      type: "warning",
      message: "Memory usage approaching 80% threshold",
      time: "15 minutes ago",
      severity: "medium",
    },
    { id: 3, type: "info", message: "Processing queue cleared successfully", time: "1 hour ago", severity: "low" },
  ]

  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <AlertTriangle className="h-5 w-5" />
          System Alerts
        </CardTitle>
        <CardDescription>Recent system notifications and warnings</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="space-y-3">
          {alerts.map((alert) => (
            <div key={alert.id} className="flex items-start gap-3 p-3 border rounded-lg">
              <div
                className={`w-2 h-2 rounded-full mt-2 ${
                  alert.severity === "high"
                    ? "bg-red-500"
                    : alert.severity === "medium"
                      ? "bg-orange-500"
                      : "bg-blue-500"
                }`}
              />
              <div className="flex-1">
                <p className="text-sm font-medium">{alert.message}</p>
                <p className="text-xs text-muted-foreground">{alert.time}</p>
              </div>
              <Badge
                variant={
                  alert.severity === "high" ? "destructive" : alert.severity === "medium" ? "secondary" : "outline"
                }
              >
                {alert.severity}
              </Badge>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  )
}

export default function AnalyticsPage() {
  const [timeRange, setTimeRange] = useState("24h")

  return (
    <div className="flex-1 space-y-6 p-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Analytics & Monitoring</h1>
          <p className="text-muted-foreground">Comprehensive insights into document processing performance</p>
        </div>
        <div className="flex items-center gap-4">
          <Select value={timeRange} onValueChange={setTimeRange}>
            <SelectTrigger className="w-32">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="1h">Last Hour</SelectItem>
              <SelectItem value="24h">Last 24h</SelectItem>
              <SelectItem value="7d">Last 7 days</SelectItem>
              <SelectItem value="30d">Last 30 days</SelectItem>
            </SelectContent>
          </Select>
          <Button>
            <Download className="h-4 w-4 mr-2" />
            Export Data
          </Button>
        </div>
      </div>

      {/* Key Metrics */}
      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
        <MetricCard title="Total Documents" value="1,255" change="+12.5%" icon={FileText} trend="up" />
        <MetricCard title="Processing Rate" value="54/hr" change="+8.2%" icon={Zap} trend="up" />
        <MetricCard title="Average Quality" value="94.2%" change="+2.1%" icon={CheckCircle} trend="up" />
        <MetricCard title="Error Rate" value="1.8%" change="-0.5%" icon={AlertCircle} trend="down" />
      </div>

      <Tabs defaultValue="overview" className="space-y-6">
        <TabsList>
          <TabsTrigger value="overview">Overview</TabsTrigger>
          <TabsTrigger value="performance">Performance</TabsTrigger>
          <TabsTrigger value="intelligence">Intelligence</TabsTrigger>
          <TabsTrigger value="monitoring">Monitoring</TabsTrigger>
        </TabsList>

        <TabsContent value="overview" className="space-y-6">
          <div className="grid gap-6 md:grid-cols-3">
            <ProcessingMetricsChart />
            <DocumentTypeChart />
          </div>
          <div className="grid gap-6 md:grid-cols-2">
            <QualityTrendsChart />
            <ProcessingTimeHistogram />
          </div>
        </TabsContent>

        <TabsContent value="performance" className="space-y-6">
          <SystemHealthGauges />
          <div className="grid gap-6 md:grid-cols-2">
            <Card>
              <CardHeader>
                <CardTitle>Queue Depth</CardTitle>
                <CardDescription>Processing backlog over time</CardDescription>
              </CardHeader>
              <CardContent>
                <ResponsiveContainer width="100%" height={300}>
                  <AreaChart data={processingMetrics}>
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis dataKey="time" />
                    <YAxis />
                    <Tooltip />
                    <Area type="monotone" dataKey="throughput" stroke="#8b5cf6" fill="#8b5cf6" fillOpacity={0.3} />
                  </AreaChart>
                </ResponsiveContainer>
              </CardContent>
            </Card>
            <AlertsPanel />
          </div>
        </TabsContent>

        <TabsContent value="intelligence" className="space-y-6">
          <div className="grid gap-6 md:grid-cols-2">
            <LanguageStatsTable />
            <ContentClassificationChart />
          </div>
          <PIIDetectionSummary />
        </TabsContent>

        <TabsContent value="monitoring" className="space-y-6">
          <div className="grid gap-6 md:grid-cols-2">
            <Card>
              <CardHeader>
                <CardTitle>System Resources</CardTitle>
                <CardDescription>Real-time resource utilization</CardDescription>
              </CardHeader>
              <CardContent>
                <div className="space-y-4">
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <Cpu className="h-4 w-4" />
                      <span>CPU Usage</span>
                    </div>
                    <div className="flex items-center gap-2">
                      <Progress value={68} className="w-24" />
                      <span className="text-sm">68%</span>
                    </div>
                  </div>
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <HardDrive className="h-4 w-4" />
                      <span>Memory</span>
                    </div>
                    <div className="flex items-center gap-2">
                      <Progress value={74} className="w-24" />
                      <span className="text-sm">74%</span>
                    </div>
                  </div>
                  <div className="flex items-center justify-between">
                    <div className="flex items-center gap-2">
                      <Server className="h-4 w-4" />
                      <span>Disk I/O</span>
                    </div>
                    <div className="flex items-center gap-2">
                      <Progress value={45} className="w-24" />
                      <span className="text-sm">45%</span>
                    </div>
                  </div>
                </div>
              </CardContent>
            </Card>
            <Card>
              <CardHeader>
                <CardTitle>Error Breakdown</CardTitle>
                <CardDescription>Types and frequency of processing errors</CardDescription>
              </CardHeader>
              <CardContent>
                <div className="space-y-3">
                  {[
                    { type: "Corrupted File", count: 12, percentage: 45 },
                    { type: "Timeout", count: 8, percentage: 30 },
                    { type: "Format Error", count: 4, percentage: 15 },
                    { type: "Memory Limit", count: 3, percentage: 10 },
                  ].map((error) => (
                    <div key={error.type} className="flex items-center justify-between">
                      <span className="text-sm">{error.type}</span>
                      <div className="flex items-center gap-2">
                        <Progress value={error.percentage} className="w-16" />
                        <span className="text-sm text-muted-foreground w-8">{error.count}</span>
                      </div>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          </div>
        </TabsContent>
      </Tabs>
    </div>
  )
}
