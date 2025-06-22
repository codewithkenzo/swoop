"use client"

import type React from "react"

import { useState } from "react"
import { Upload, FileText, Clock, CheckCircle, AlertCircle, XCircle, TrendingUp, Users, Zap } from "lucide-react"
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card"
import { Progress } from "@/components/ui/progress"
import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { cn } from "@/lib/utils"

// Mock data
const mockStats = {
  totalDocuments: 1247,
  processingSpeed: 95,
  qualityScore: 98.5,
  activeUsers: 23,
}

const mockQueue = [
  { id: 1, name: "Financial_Report_Q3.pdf", status: "processing", progress: 65, size: "2.4 MB" },
  { id: 2, name: "Contract_Amendment.docx", status: "completed", progress: 100, size: "1.8 MB" },
  { id: 3, name: "Invoice_12345.pdf", status: "pending", progress: 0, size: "0.9 MB" },
  { id: 4, name: "Legal_Document.pdf", status: "error", progress: 0, size: "3.2 MB" },
  { id: 5, name: "Marketing_Proposal.pptx", status: "processing", progress: 32, size: "5.1 MB" },
]

const mockActivity = [
  { id: 1, action: "Document processed", document: "Annual_Report.pdf", time: "2 minutes ago", type: "success" },
  { id: 2, action: "New document uploaded", document: "Contract_v2.docx", time: "5 minutes ago", type: "info" },
  { id: 3, action: "Processing failed", document: "Corrupted_file.pdf", time: "12 minutes ago", type: "error" },
  { id: 4, action: "Quality check passed", document: "Invoice_batch.zip", time: "18 minutes ago", type: "success" },
  { id: 5, action: "User joined workspace", document: "john.doe@company.com", time: "1 hour ago", type: "info" },
]

function UploadArea() {
  const [isDragOver, setIsDragOver] = useState(false)

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault()
    setIsDragOver(true)
  }

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault()
    setIsDragOver(false)
  }

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault()
    setIsDragOver(false)
    // Handle file drop logic here
  }

  return (
    <Card className="col-span-full">
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <Upload className="h-5 w-5" />
          Document Upload
        </CardTitle>
        <CardDescription>Drag and drop your documents here or click to browse</CardDescription>
      </CardHeader>
      <CardContent>
        <div
          className={cn(
            "border-2 border-dashed rounded-lg p-8 text-center transition-colors",
            isDragOver ? "border-primary bg-primary/5" : "border-muted-foreground/25 hover:border-muted-foreground/50",
          )}
          onDragOver={handleDragOver}
          onDragLeave={handleDragLeave}
          onDrop={handleDrop}
        >
          <Upload className="mx-auto h-12 w-12 text-muted-foreground mb-4" />
          <div className="space-y-2">
            <p className="text-lg font-medium">Drop your documents here</p>
            <p className="text-sm text-muted-foreground">Supports PDF, DOCX, PPTX, and more. Maximum file size: 50MB</p>
            <Button className="mt-4">Browse Files</Button>
          </div>
        </div>
      </CardContent>
    </Card>
  )
}

function StatsCards() {
  return (
    <>
      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">Total Documents</CardTitle>
          <FileText className="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{mockStats.totalDocuments.toLocaleString()}</div>
          <p className="text-xs text-muted-foreground">
            <span className="text-green-600">+12%</span> from last month
          </p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">Processing Speed</CardTitle>
          <Zap className="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{mockStats.processingSpeed}%</div>
          <Progress value={mockStats.processingSpeed} className="mt-2" />
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">Quality Score</CardTitle>
          <TrendingUp className="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{mockStats.qualityScore}%</div>
          <Progress value={mockStats.qualityScore} className="mt-2" />
        </CardContent>
      </Card>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle className="text-sm font-medium">Active Users</CardTitle>
          <Users className="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div className="text-2xl font-bold">{mockStats.activeUsers}</div>
          <p className="text-xs text-muted-foreground">Currently online</p>
        </CardContent>
      </Card>
    </>
  )
}

function ProcessingQueue() {
  const getStatusIcon = (status: string) => {
    switch (status) {
      case "completed":
        return <CheckCircle className="h-4 w-4 text-green-600" />
      case "processing":
        return <Clock className="h-4 w-4 text-orange-600" />
      case "error":
        return <XCircle className="h-4 w-4 text-red-600" />
      default:
        return <AlertCircle className="h-4 w-4 text-gray-600" />
    }
  }

  const getStatusBadge = (status: string) => {
    const variants = {
      completed: "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300",
      processing: "bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-300",
      error: "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300",
      pending: "bg-gray-100 text-gray-800 dark:bg-gray-900 dark:text-gray-300",
    }

    return (
      <Badge className={variants[status as keyof typeof variants]}>
        {status.charAt(0).toUpperCase() + status.slice(1)}
      </Badge>
    )
  }

  return (
    <Card className="col-span-2">
      <CardHeader>
        <CardTitle>Processing Queue</CardTitle>
        <CardDescription>Current document processing status</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          {mockQueue.map((item) => (
            <div key={item.id} className="flex items-center space-x-4 p-3 rounded-lg border">
              {getStatusIcon(item.status)}
              <div className="flex-1 min-w-0">
                <p className="text-sm font-medium truncate">{item.name}</p>
                <p className="text-xs text-muted-foreground">{item.size}</p>
                {item.status === "processing" && <Progress value={item.progress} className="mt-2 h-2" />}
              </div>
              <div className="flex items-center space-x-2">
                {item.status === "processing" && (
                  <span className="text-xs text-muted-foreground">{item.progress}%</span>
                )}
                {getStatusBadge(item.status)}
              </div>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  )
}

function RecentActivity() {
  const getActivityIcon = (type: string) => {
    switch (type) {
      case "success":
        return <CheckCircle className="h-4 w-4 text-green-600" />
      case "error":
        return <XCircle className="h-4 w-4 text-red-600" />
      default:
        return <Clock className="h-4 w-4 text-blue-600" />
    }
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Recent Activity</CardTitle>
        <CardDescription>Latest platform events and updates</CardDescription>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          {mockActivity.map((activity) => (
            <div key={activity.id} className="flex items-start space-x-3">
              {getActivityIcon(activity.type)}
              <div className="flex-1 min-w-0">
                <p className="text-sm font-medium">{activity.action}</p>
                <p className="text-xs text-muted-foreground truncate">{activity.document}</p>
                <p className="text-xs text-muted-foreground">{activity.time}</p>
              </div>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  )
}

export default function Dashboard() {
  return (
    <div className="flex-1 space-y-6 p-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Dashboard</h1>
          <p className="text-muted-foreground">Welcome to Swoop - Your document intelligence platform</p>
        </div>
      </div>

      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
        <StatsCards />
      </div>

      <UploadArea />

      <div className="grid gap-6 md:grid-cols-3">
        <ProcessingQueue />
        <RecentActivity />
      </div>
    </div>
  )
}
