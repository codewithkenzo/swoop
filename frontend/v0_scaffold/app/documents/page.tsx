"use client"

import { useState, useMemo } from "react"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"
import { Badge } from "@/components/ui/badge"
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select"
import { Checkbox } from "@/components/ui/checkbox"
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Progress } from "@/components/ui/progress"
import {
  Search,
  Download,
  Trash2,
  RefreshCw,
  FileText,
  Calendar,
  Eye,
  MoreHorizontal,
  Globe,
  Shield,
  Clock,
} from "lucide-react"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu"

// Mock document data
const mockDocuments = [
  {
    id: 1,
    name: "Q3_Financial_Report.pdf",
    type: "PDF",
    size: "2.4 MB",
    uploadDate: "2024-01-15",
    processedDate: "2024-01-15",
    status: "completed",
    quality: 0.96,
    language: "English",
    category: "Financial",
    processingTime: 45,
    piiDetected: true,
    complexity: "Medium",
    pages: 24,
    wordCount: 8450,
  },
  {
    id: 2,
    name: "Employee_Handbook_2024.docx",
    type: "DOCX",
    size: "1.8 MB",
    uploadDate: "2024-01-14",
    processedDate: "2024-01-14",
    status: "completed",
    quality: 0.94,
    language: "English",
    category: "HR",
    processingTime: 32,
    piiDetected: false,
    complexity: "Low",
    pages: 45,
    wordCount: 12300,
  },
  {
    id: 3,
    name: "Contract_Amendment_v3.pdf",
    type: "PDF",
    size: "0.9 MB",
    uploadDate: "2024-01-13",
    processedDate: "2024-01-13",
    status: "completed",
    quality: 0.98,
    language: "English",
    category: "Legal",
    processingTime: 28,
    piiDetected: true,
    complexity: "High",
    pages: 12,
    wordCount: 4200,
  },
  {
    id: 4,
    name: "Marketing_Strategy_2024.pptx",
    type: "PPTX",
    size: "5.1 MB",
    uploadDate: "2024-01-12",
    processedDate: "2024-01-12",
    status: "processing",
    quality: 0.0,
    language: "English",
    category: "Marketing",
    processingTime: 0,
    piiDetected: false,
    complexity: "Medium",
    pages: 32,
    wordCount: 2800,
  },
  {
    id: 5,
    name: "Technical_Specifications.pdf",
    type: "PDF",
    size: "3.2 MB",
    uploadDate: "2024-01-11",
    processedDate: "2024-01-11",
    status: "error",
    quality: 0.0,
    language: "English",
    category: "Technical",
    processingTime: 0,
    piiDetected: false,
    complexity: "High",
    pages: 67,
    wordCount: 15600,
  },
  {
    id: 6,
    name: "Budget_Analysis_Q4.xlsx",
    type: "XLSX",
    size: "1.2 MB",
    uploadDate: "2024-01-10",
    processedDate: "2024-01-10",
    status: "completed",
    quality: 0.92,
    language: "English",
    category: "Financial",
    processingTime: 18,
    piiDetected: false,
    complexity: "Low",
    pages: 8,
    wordCount: 1200,
  },
]

function DocumentDetailsModal({ document, isOpen, onClose }: { document: any; isOpen: boolean; onClose: () => void }) {
  if (!document) return null

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent className="max-w-4xl max-h-[80vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <FileText className="h-5 w-5" />
            {document.name}
          </DialogTitle>
          <DialogDescription>Detailed document information and processing history</DialogDescription>
        </DialogHeader>

        <Tabs defaultValue="overview" className="w-full">
          <TabsList className="grid w-full grid-cols-4">
            <TabsTrigger value="overview">Overview</TabsTrigger>
            <TabsTrigger value="quality">Quality</TabsTrigger>
            <TabsTrigger value="content">Content</TabsTrigger>
            <TabsTrigger value="history">History</TabsTrigger>
          </TabsList>

          <TabsContent value="overview" className="space-y-4">
            <div className="grid grid-cols-2 gap-4">
              <Card>
                <CardHeader>
                  <CardTitle className="text-sm">Basic Information</CardTitle>
                </CardHeader>
                <CardContent className="space-y-2">
                  <div className="flex justify-between">
                    <span className="text-sm text-muted-foreground">Type:</span>
                    <Badge variant="outline">{document.type}</Badge>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-sm text-muted-foreground">Size:</span>
                    <span className="text-sm">{document.size}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-sm text-muted-foreground">Pages:</span>
                    <span className="text-sm">{document.pages}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-sm text-muted-foreground">Words:</span>
                    <span className="text-sm">{document.wordCount.toLocaleString()}</span>
                  </div>
                </CardContent>
              </Card>

              <Card>
                <CardHeader>
                  <CardTitle className="text-sm">Processing Info</CardTitle>
                </CardHeader>
                <CardContent className="space-y-2">
                  <div className="flex justify-between">
                    <span className="text-sm text-muted-foreground">Status:</span>
                    <Badge
                      variant={
                        document.status === "completed"
                          ? "default"
                          : document.status === "error"
                            ? "destructive"
                            : "secondary"
                      }
                    >
                      {document.status}
                    </Badge>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-sm text-muted-foreground">Quality:</span>
                    <span className="text-sm">{(document.quality * 100).toFixed(1)}%</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-sm text-muted-foreground">Language:</span>
                    <span className="text-sm">{document.language}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-sm text-muted-foreground">Category:</span>
                    <Badge variant="outline">{document.category}</Badge>
                  </div>
                </CardContent>
              </Card>
            </div>
          </TabsContent>

          <TabsContent value="quality" className="space-y-4">
            <Card>
              <CardHeader>
                <CardTitle className="text-sm">Quality Breakdown</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span>Text Extraction</span>
                    <span>98%</span>
                  </div>
                  <Progress value={98} />
                </div>
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span>Structure Recognition</span>
                    <span>94%</span>
                  </div>
                  <Progress value={94} />
                </div>
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span>Language Detection</span>
                    <span>100%</span>
                  </div>
                  <Progress value={100} />
                </div>
                <div className="space-y-2">
                  <div className="flex justify-between text-sm">
                    <span>Content Classification</span>
                    <span>92%</span>
                  </div>
                  <Progress value={92} />
                </div>
              </CardContent>
            </Card>
          </TabsContent>

          <TabsContent value="content" className="space-y-4">
            <Card>
              <CardHeader>
                <CardTitle className="text-sm">Content Analysis</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="grid grid-cols-2 gap-4">
                  <div className="space-y-2">
                    <div className="flex items-center gap-2">
                      <Globe className="h-4 w-4" />
                      <span className="text-sm font-medium">Language</span>
                    </div>
                    <p className="text-sm text-muted-foreground">{document.language} (99.8% confidence)</p>
                  </div>
                  <div className="space-y-2">
                    <div className="flex items-center gap-2">
                      <Shield className="h-4 w-4" />
                      <span className="text-sm font-medium">PII Detection</span>
                    </div>
                    <p className="text-sm text-muted-foreground">
                      {document.piiDetected ? "PII detected - review required" : "No PII detected"}
                    </p>
                  </div>
                </div>
                <div className="space-y-2">
                  <span className="text-sm font-medium">Complexity Score</span>
                  <div className="flex items-center gap-2">
                    <Badge
                      variant={
                        document.complexity === "High"
                          ? "destructive"
                          : document.complexity === "Medium"
                            ? "secondary"
                            : "default"
                      }
                    >
                      {document.complexity}
                    </Badge>
                    <span className="text-sm text-muted-foreground">
                      Based on structure, vocabulary, and formatting
                    </span>
                  </div>
                </div>
              </CardContent>
            </Card>
          </TabsContent>

          <TabsContent value="history" className="space-y-4">
            <Card>
              <CardHeader>
                <CardTitle className="text-sm">Processing Timeline</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-4">
                  <div className="flex items-center gap-3">
                    <div className="w-2 h-2 bg-blue-500 rounded-full" />
                    <div className="flex-1">
                      <p className="text-sm font-medium">Document uploaded</p>
                      <p className="text-xs text-muted-foreground">{document.uploadDate} at 10:30 AM</p>
                    </div>
                  </div>
                  <div className="flex items-center gap-3">
                    <div className="w-2 h-2 bg-orange-500 rounded-full" />
                    <div className="flex-1">
                      <p className="text-sm font-medium">Processing started</p>
                      <p className="text-xs text-muted-foreground">{document.uploadDate} at 10:31 AM</p>
                    </div>
                  </div>
                  <div className="flex items-center gap-3">
                    <div className="w-2 h-2 bg-green-500 rounded-full" />
                    <div className="flex-1">
                      <p className="text-sm font-medium">Processing completed</p>
                      <p className="text-xs text-muted-foreground">{document.processedDate} at 10:32 AM</p>
                    </div>
                  </div>
                </div>
              </CardContent>
            </Card>
          </TabsContent>
        </Tabs>
      </DialogContent>
    </Dialog>
  )
}

export default function DocumentsPage() {
  const [searchQuery, setSearchQuery] = useState("")
  const [selectedType, setSelectedType] = useState("all")
  const [selectedStatus, setSelectedStatus] = useState("all")
  const [selectedCategory, setSelectedCategory] = useState("all")
  const [sortBy, setSortBy] = useState("uploadDate")
  const [sortOrder, setSortOrder] = useState<"asc" | "desc">("desc")
  const [selectedDocuments, setSelectedDocuments] = useState<number[]>([])
  const [selectedDocument, setSelectedDocument] = useState<any>(null)

  const filteredAndSortedDocuments = useMemo(() => {
    const filtered = mockDocuments.filter((doc) => {
      const matchesSearch = doc.name.toLowerCase().includes(searchQuery.toLowerCase())
      const matchesType = selectedType === "all" || doc.type === selectedType
      const matchesStatus = selectedStatus === "all" || doc.status === selectedStatus
      const matchesCategory = selectedCategory === "all" || doc.category === selectedCategory

      return matchesSearch && matchesType && matchesStatus && matchesCategory
    })

    filtered.sort((a, b) => {
      let aValue = a[sortBy as keyof typeof a]
      let bValue = b[sortBy as keyof typeof b]

      if (typeof aValue === "string") {
        aValue = aValue.toLowerCase()
        bValue = (bValue as string).toLowerCase()
      }

      if (sortOrder === "asc") {
        return aValue < bValue ? -1 : aValue > bValue ? 1 : 0
      } else {
        return aValue > bValue ? -1 : aValue < bValue ? 1 : 0
      }
    })

    return filtered
  }, [searchQuery, selectedType, selectedStatus, selectedCategory, sortBy, sortOrder])

  const handleSelectAll = (checked: boolean) => {
    if (checked) {
      setSelectedDocuments(filteredAndSortedDocuments.map((doc) => doc.id))
    } else {
      setSelectedDocuments([])
    }
  }

  const handleSelectDocument = (id: number, checked: boolean) => {
    if (checked) {
      setSelectedDocuments([...selectedDocuments, id])
    } else {
      setSelectedDocuments(selectedDocuments.filter((docId) => docId !== id))
    }
  }

  const getStatusBadge = (status: string) => {
    switch (status) {
      case "completed":
        return <Badge className="bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300">Completed</Badge>
      case "processing":
        return (
          <Badge className="bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-300">Processing</Badge>
        )
      case "error":
        return <Badge className="bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300">Error</Badge>
      default:
        return <Badge variant="outline">{status}</Badge>
    }
  }

  return (
    <div className="flex-1 space-y-6 p-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Document Management</h1>
          <p className="text-muted-foreground">Manage and analyze your processed documents</p>
        </div>
        <div className="flex items-center gap-2">
          <Button variant="outline">
            <Download className="h-4 w-4 mr-2" />
            Export
          </Button>
          {selectedDocuments.length > 0 && (
            <>
              <Button variant="outline">
                <RefreshCw className="h-4 w-4 mr-2" />
                Reprocess ({selectedDocuments.length})
              </Button>
              <Button variant="destructive">
                <Trash2 className="h-4 w-4 mr-2" />
                Delete ({selectedDocuments.length})
              </Button>
            </>
          )}
        </div>
      </div>

      {/* Filters */}
      <Card>
        <CardHeader>
          <CardTitle className="text-lg">Filters & Search</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex flex-wrap gap-4">
            <div className="flex-1 min-w-64">
              <div className="relative">
                <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
                <Input
                  placeholder="Search documents..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="pl-8"
                />
              </div>
            </div>
            <Select value={selectedType} onValueChange={setSelectedType}>
              <SelectTrigger className="w-32">
                <SelectValue placeholder="Type" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All Types</SelectItem>
                <SelectItem value="PDF">PDF</SelectItem>
                <SelectItem value="DOCX">DOCX</SelectItem>
                <SelectItem value="PPTX">PPTX</SelectItem>
                <SelectItem value="XLSX">XLSX</SelectItem>
              </SelectContent>
            </Select>
            <Select value={selectedStatus} onValueChange={setSelectedStatus}>
              <SelectTrigger className="w-32">
                <SelectValue placeholder="Status" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All Status</SelectItem>
                <SelectItem value="completed">Completed</SelectItem>
                <SelectItem value="processing">Processing</SelectItem>
                <SelectItem value="error">Error</SelectItem>
              </SelectContent>
            </Select>
            <Select value={selectedCategory} onValueChange={setSelectedCategory}>
              <SelectTrigger className="w-32">
                <SelectValue placeholder="Category" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All Categories</SelectItem>
                <SelectItem value="Financial">Financial</SelectItem>
                <SelectItem value="Legal">Legal</SelectItem>
                <SelectItem value="HR">HR</SelectItem>
                <SelectItem value="Marketing">Marketing</SelectItem>
                <SelectItem value="Technical">Technical</SelectItem>
              </SelectContent>
            </Select>
            <Select
              value={`${sortBy}-${sortOrder}`}
              onValueChange={(value) => {
                const [field, order] = value.split("-")
                setSortBy(field)
                setSortOrder(order as "asc" | "desc")
              }}
            >
              <SelectTrigger className="w-40">
                <SelectValue placeholder="Sort by" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="uploadDate-desc">Newest First</SelectItem>
                <SelectItem value="uploadDate-asc">Oldest First</SelectItem>
                <SelectItem value="name-asc">Name A-Z</SelectItem>
                <SelectItem value="name-desc">Name Z-A</SelectItem>
                <SelectItem value="quality-desc">Quality High-Low</SelectItem>
                <SelectItem value="quality-asc">Quality Low-High</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </CardContent>
      </Card>

      {/* Documents Table */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center justify-between">
            <span>Documents ({filteredAndSortedDocuments.length})</span>
            <div className="flex items-center gap-2">
              <Checkbox
                checked={
                  selectedDocuments.length === filteredAndSortedDocuments.length &&
                  filteredAndSortedDocuments.length > 0
                }
                onCheckedChange={handleSelectAll}
              />
              <span className="text-sm text-muted-foreground">Select All</span>
            </div>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <div className="space-y-4">
            {filteredAndSortedDocuments.map((document) => (
              <div key={document.id} className="flex items-center gap-4 p-4 border rounded-lg hover:bg-muted/50">
                <Checkbox
                  checked={selectedDocuments.includes(document.id)}
                  onCheckedChange={(checked) => handleSelectDocument(document.id, checked as boolean)}
                />
                <FileText className="h-8 w-8 text-muted-foreground" />
                <div className="flex-1 min-w-0">
                  <div className="flex items-center gap-2 mb-1">
                    <h3 className="font-medium truncate">{document.name}</h3>
                    {document.piiDetected && <Shield className="h-4 w-4 text-orange-500" />}
                  </div>
                  <div className="flex items-center gap-4 text-sm text-muted-foreground">
                    <span>{document.type}</span>
                    <span>{document.size}</span>
                    <span>{document.category}</span>
                    <span className="flex items-center gap-1">
                      <Calendar className="h-3 w-3" />
                      {document.uploadDate}
                    </span>
                    {document.status === "completed" && (
                      <span className="flex items-center gap-1">
                        <Clock className="h-3 w-3" />
                        {document.processingTime}s
                      </span>
                    )}
                  </div>
                </div>
                <div className="flex items-center gap-4">
                  {getStatusBadge(document.status)}
                  {document.status === "completed" && (
                    <div className="text-right">
                      <div className="text-sm font-medium">{(document.quality * 100).toFixed(1)}%</div>
                      <div className="text-xs text-muted-foreground">Quality</div>
                    </div>
                  )}
                  <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                      <Button variant="ghost" size="sm">
                        <MoreHorizontal className="h-4 w-4" />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent>
                      <DropdownMenuItem onClick={() => setSelectedDocument(document)}>
                        <Eye className="h-4 w-4 mr-2" />
                        View Details
                      </DropdownMenuItem>
                      <DropdownMenuItem>
                        <Download className="h-4 w-4 mr-2" />
                        Download
                      </DropdownMenuItem>
                      <DropdownMenuItem>
                        <RefreshCw className="h-4 w-4 mr-2" />
                        Reprocess
                      </DropdownMenuItem>
                      <DropdownMenuItem className="text-destructive">
                        <Trash2 className="h-4 w-4 mr-2" />
                        Delete
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </div>
              </div>
            ))}
          </div>
        </CardContent>
      </Card>

      <DocumentDetailsModal
        document={selectedDocument}
        isOpen={!!selectedDocument}
        onClose={() => setSelectedDocument(null)}
      />
    </div>
  )
}
