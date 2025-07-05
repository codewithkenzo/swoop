import { useQuery } from '@tanstack/react-query'
import { 
  FileText, 
  Brain, 
  Globe, 
  TrendingUp,
  Activity,
  Clock,
  Tag
} from 'lucide-react'

import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { formatFileSize, formatDate } from '@/lib/utils'
import { apiClient } from '@/lib/api'

interface DashboardStats {
  totalDocuments: number
  totalSize: number
  recentUploads: number
  activeJobs: number
  categorizedDocuments: number
  topCategories: Array<{ category: string; count: number }>
  recentActivity: Array<{
    id: string
    type: 'upload' | 'crawl' | 'analysis'
    description: string
    timestamp: string
  }>
}

export function Dashboard() {
  const { data: stats, isLoading } = useQuery({
    queryKey: ["dashboard-stats"],
    queryFn: async (): Promise<DashboardStats> => {
      const metrics: any = await apiClient.getMetrics();
      return {
        totalDocuments: metrics.data.total_documents || 0,
        totalSize: 0,
        recentUploads: 0,
        activeJobs: 0,
        categorizedDocuments: metrics.data.total_documents || 0,
        topCategories: [],
        recentActivity: metrics.data.recent_activity || [],
      } as DashboardStats;
    },
  });

  if (isLoading) {
    return (
      <div className="space-y-6">
        <div className="animate-pulse">
          <div className="h-8 bg-muted rounded w-1/4 mb-2"></div>
          <div className="h-4 bg-muted rounded w-1/2"></div>
        </div>
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
          {[...Array(4)].map((_, i) => (
            <div key={i} className="animate-pulse">
              <div className="h-32 bg-muted rounded-lg"></div>
            </div>
          ))}
        </div>
      </div>
    )
  }

  const statCards = [
    {
      title: 'Total Documents',
      value: stats?.totalDocuments.toLocaleString() || '0',
      description: '+12% from last month',
      icon: FileText,
      color: 'text-blue-600',
    },
    {
      title: 'Storage Used',
      value: formatFileSize(stats?.totalSize || 0),
      description: `${stats?.recentUploads || 0} uploaded this week`,
      icon: Activity,
      color: 'text-green-600',
    },
    {
      title: 'AI Processed',
      value: `${stats?.categorizedDocuments || 0}`,
      description: `${Math.round(((stats?.categorizedDocuments || 0) / (stats?.totalDocuments || 1)) * 100)}% coverage`,
      icon: Brain,
      color: 'text-purple-600',
    },
    {
      title: 'Active Jobs',
      value: `${stats?.activeJobs || 0}`,
      description: 'Crawling and processing',
      icon: Globe,
      color: 'text-orange-600',
    },
  ]

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Dashboard</h1>
        <p className="text-muted-foreground">
          Welcome to your AI-powered document intelligence platform
        </p>
      </div>

      {/* Stats Cards */}
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        {statCards.map((stat) => {
          const Icon = stat.icon
          return (
            <Card key={stat.title}>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                  {stat.title}
                </CardTitle>
                <Icon className={`h-4 w-4 ${stat.color}`} />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">{stat.value}</div>
                <p className="text-xs text-muted-foreground">
                  {stat.description}
                </p>
              </CardContent>
            </Card>
          )
        })}
      </div>

      <div className="grid gap-6 md:grid-cols-2">
        {/* Document Categories */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center space-x-2">
              <Tag className="h-5 w-5" />
              <span>Document Categories</span>
            </CardTitle>
            <CardDescription>
              Distribution of your analyzed documents
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-3">
              {stats?.topCategories.map((category) => (
                <div key={category.category} className="flex items-center justify-between">
                  <div className="flex items-center space-x-2">
                    <div className="w-3 h-3 rounded-full bg-primary" />
                    <span className="text-sm font-medium">{category.category}</span>
                  </div>
                  <span className="text-sm text-muted-foreground">
                    {category.count}
                  </span>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>

        {/* Recent Activity */}
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center space-x-2">
              <Clock className="h-5 w-5" />
              <span>Recent Activity</span>
            </CardTitle>
            <CardDescription>
              Latest actions in your workspace
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              {stats?.recentActivity.map((activity) => (
                <div key={activity.id} className="flex items-start space-x-3">
                  <div className="w-2 h-2 rounded-full bg-primary mt-2" />
                  <div className="flex-1 min-w-0">
                    <p className="text-sm font-medium truncate">
                      {activity.description}
                    </p>
                    <p className="text-xs text-muted-foreground">
                      {formatDate(activity.timestamp)}
                    </p>
                  </div>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Quick Actions */}
      <Card>
        <CardHeader>
          <CardTitle>Quick Actions</CardTitle>
          <CardDescription>
            Get started with common tasks
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex flex-wrap gap-4">
            <Button>
              <FileText className="mr-2 h-4 w-4" />
              Upload Documents
            </Button>
            <Button variant="outline">
              <Globe className="mr-2 h-4 w-4" />
              Start Web Crawl
            </Button>
            <Button variant="outline">
              <Brain className="mr-2 h-4 w-4" />
              Analyze Documents
            </Button>
            <Button variant="outline">
              <TrendingUp className="mr-2 h-4 w-4" />
              View Analytics
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  )
} 