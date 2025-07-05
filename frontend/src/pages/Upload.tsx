
import { useState } from 'react'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { FileUpload } from '@/components/upload/FileUpload'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { StreamingProgress } from '@/components/StreamingProgress'
import { apiClient } from '@/lib/api'
import { Brain, Settings } from 'lucide-react'

export function Upload() {
  const queryClient = useQueryClient()
  const [processingDocuments, setProcessingDocuments] = useState<string[]>([])

  const uploadMutation = useMutation({
    mutationFn: async (files: File[]) => {
      const uploadPromises = files.map(file => 
        apiClient.uploadFile(file, (progress) => {
          // Update progress in real-time
          console.log(`Upload progress for ${file.name}: ${progress}%`)
        })
      )
      
      const results = await Promise.all(uploadPromises)
      
      // Start tracking document processing
      const documentIds = results.map(result => result.data?.id).filter(Boolean) as string[]
      setProcessingDocuments(documentIds)
      
      return results
    },
    onSuccess: () => {
      // Invalidate and refetch documents
      queryClient.invalidateQueries({ queryKey: ['documents'] })
      queryClient.invalidateQueries({ queryKey: ['dashboard-stats'] })
    },
    onError: (error) => {
      console.error('Upload failed:', error)
    }
  })

  const handleDocumentComplete = (documentId: string) => {
    setProcessingDocuments(prev => prev.filter(id => id !== documentId))
    // Refresh document list
    queryClient.invalidateQueries({ queryKey: ['documents'] })
  }

  const handleDocumentError = (documentId: string, error: string) => {
    console.error(`Document ${documentId} processing failed:`, error)
    // Still remove from processing list
    setProcessingDocuments(prev => prev.filter(id => id !== documentId))
  }

  const handleUpload = (files: File[]) => {
    uploadMutation.mutate(files)
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div>
        <h1 className="text-3xl font-bold tracking-tight">Upload Documents</h1>
        <p className="text-muted-foreground">
          Upload documents for AI-powered analysis and processing
        </p>
      </div>

      <div className="grid gap-6 lg:grid-cols-3">
        {/* Upload Area */}
        <div className="lg:col-span-2 space-y-6">
          <FileUpload
            onUpload={handleUpload}
            maxFiles={20}
            maxSize={100 * 1024 * 1024} // 100MB
          />
          
          {/* Real-time Processing Progress */}
          {processingDocuments.length > 0 && (
            <div className="space-y-4">
              <h3 className="text-lg font-semibold">Processing Documents</h3>
              {processingDocuments.map((documentId) => (
                <StreamingProgress
                  key={documentId}
                  type="document"
                  id={documentId}
                  title={`Document ${documentId.slice(0, 8)}...`}
                  onComplete={() => handleDocumentComplete(documentId)}
                  onError={(error) => handleDocumentError(documentId, error)}
                />
              ))}
            </div>
          )}
        </div>

        {/* Settings Panel */}
        <div className="space-y-4">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center space-x-2">
                <Brain className="h-5 w-5" />
                <span>AI Analysis</span>
              </CardTitle>
              <CardDescription>
                Configure automatic analysis settings
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex items-center justify-between">
                <span className="text-sm">Document Categorization</span>
                <input type="checkbox" defaultChecked className="rounded" />
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm">Entity Extraction</span>
                <input type="checkbox" defaultChecked className="rounded" />
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm">Auto-Tagging</span>
                <input type="checkbox" defaultChecked className="rounded" />
              </div>
              <div className="flex items-center justify-between">
                <span className="text-sm">Generate Embeddings</span>
                <input type="checkbox" defaultChecked className="rounded" />
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="flex items-center space-x-2">
                <Settings className="h-5 w-5" />
                <span>Processing Options</span>
              </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
              <div>
                <label className="text-sm font-medium">Processing Priority</label>
                <select className="w-full mt-1 p-2 border rounded-md">
                  <option value="normal">Normal</option>
                  <option value="high">High</option>
                  <option value="low">Low</option>
                </select>
              </div>
              <div>
                <label className="text-sm font-medium">Notification</label>
                <select className="w-full mt-1 p-2 border rounded-md">
                  <option value="completion">On Completion</option>
                  <option value="errors">On Errors Only</option>
                  <option value="none">None</option>
                </select>
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Supported Formats</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-2 gap-2 text-sm">
                <div className="flex items-center space-x-2">
                  <div className="w-2 h-2 bg-red-500 rounded-full"></div>
                  <span>PDF</span>
                </div>
                <div className="flex items-center space-x-2">
                  <div className="w-2 h-2 bg-blue-500 rounded-full"></div>
                  <span>Word</span>
                </div>
                <div className="flex items-center space-x-2">
                  <div className="w-2 h-2 bg-green-500 rounded-full"></div>
                  <span>Text</span>
                </div>
                <div className="flex items-center space-x-2">
                  <div className="w-2 h-2 bg-orange-500 rounded-full"></div>
                  <span>HTML</span>
                </div>
                <div className="flex items-center space-x-2">
                  <div className="w-2 h-2 bg-purple-500 rounded-full"></div>
                  <span>Markdown</span>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
      </div>

      {/* Quick Actions */}
      <Card>
        <CardHeader>
          <CardTitle>Quick Actions</CardTitle>
          <CardDescription>
            After uploading, you can perform these actions
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex flex-wrap gap-2">
            <Button variant="outline" size="sm">
              View All Documents
            </Button>
            <Button variant="outline" size="sm">
              Start Batch Analysis
            </Button>
            <Button variant="outline" size="sm">
              Export Results
            </Button>
            <Button variant="outline" size="sm">
              Configure Rules
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  )
} 