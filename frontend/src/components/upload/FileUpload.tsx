import { useCallback, useState } from 'react'
import { useDropzone } from 'react-dropzone'
import { Upload, File, X, CheckCircle, AlertCircle } from 'lucide-react'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { cn, formatFileSize } from '@/lib/utils'
import { UploadProgress } from '@/types'

interface FileUploadProps {
  onUpload: (files: File[]) => void
  maxFiles?: number
  maxSize?: number
  acceptedTypes?: string[]
  className?: string
}

export function FileUpload({
  onUpload,
  maxFiles = 10,
  maxSize = 50 * 1024 * 1024, // 50MB
  acceptedTypes = ['.pdf', '.doc', '.docx', '.txt', '.html', '.md'],
  className
}: FileUploadProps) {
  const [uploadProgress, setUploadProgress] = useState<UploadProgress[]>([])

  const onDrop = useCallback((acceptedFiles: File[], rejectedFiles: any[]) => {
    if (rejectedFiles.length > 0) {
      // Handle rejected files
      console.warn('Some files were rejected:', rejectedFiles)
    }

    if (acceptedFiles.length > 0) {
      // Initialize upload progress
      const newProgress: UploadProgress[] = acceptedFiles.map(file => ({
        file,
        progress: 0,
        status: 'pending'
      }))
      
      setUploadProgress(prev => [...prev, ...newProgress])
      onUpload(acceptedFiles)
    }
  }, [onUpload])

  const { getRootProps, getInputProps, isDragActive, isDragReject } = useDropzone({
    onDrop,
    maxFiles,
    maxSize,
    accept: {
      'application/pdf': ['.pdf'],
      'application/msword': ['.doc'],
      'application/vnd.openxmlformats-officedocument.wordprocessingml.document': ['.docx'],
      'text/plain': ['.txt'],
      'text/html': ['.html'],
      'text/markdown': ['.md'],
    },
    multiple: true
  })

  const removeFile = (index: number) => {
    setUploadProgress(prev => prev.filter((_, i) => i !== index))
  }

  const getStatusIcon = (status: UploadProgress['status']) => {
    switch (status) {
      case 'completed':
        return <CheckCircle className="h-4 w-4 text-green-500" />
      case 'error':
        return <AlertCircle className="h-4 w-4 text-red-500" />
      case 'uploading':
      case 'processing':
        return <div className="h-4 w-4 border-2 border-primary border-t-transparent rounded-full animate-spin" />
      default:
        return <File className="h-4 w-4 text-muted-foreground" />
    }
  }

  return (
    <div className={cn("space-y-4", className)}>
      {/* Drop Zone */}
      <Card>
        <CardContent className="p-6">
          <div
            {...getRootProps()}
            className={cn(
              "border-2 border-dashed rounded-lg p-8 text-center cursor-pointer transition-colors",
              isDragActive && !isDragReject && "border-primary bg-primary/5",
              isDragReject && "border-red-500 bg-red-50",
              !isDragActive && "border-muted-foreground/25 hover:border-muted-foreground/50"
            )}
          >
            <input {...getInputProps()} />
            <div className="flex flex-col items-center space-y-4">
              <Upload className={cn(
                "h-12 w-12",
                isDragActive && !isDragReject && "text-primary",
                isDragReject && "text-red-500",
                !isDragActive && "text-muted-foreground"
              )} />
              
              <div className="space-y-2">
                <p className="text-lg font-medium">
                  {isDragActive
                    ? isDragReject
                      ? "Some files are not supported"
                      : "Drop files here"
                    : "Drag & drop files here"
                  }
                </p>
                <p className="text-sm text-muted-foreground">
                  or click to browse files
                </p>
                <p className="text-xs text-muted-foreground">
                  Supports: {acceptedTypes.join(', ')} • Max {formatFileSize(maxSize)} per file
                </p>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Upload Progress */}
      {uploadProgress.length > 0 && (
        <Card>
          <CardContent className="p-6">
            <h3 className="font-medium mb-4">Upload Progress</h3>
            <div className="space-y-3">
              {uploadProgress.map((upload, index) => (
                <div key={index} className="flex items-center space-x-3">
                  {getStatusIcon(upload.status)}
                  
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center justify-between">
                      <p className="text-sm font-medium truncate">
                        {upload.file.name}
                      </p>
                      <span className="text-xs text-muted-foreground">
                        {formatFileSize(upload.file.size)}
                      </span>
                    </div>
                    
                    {upload.status === 'uploading' && (
                      <div className="mt-1">
                        <div className="bg-muted rounded-full h-2">
                          <div 
                            className="bg-primary h-2 rounded-full transition-all duration-300"
                            style={{ width: `${upload.progress}%` }}
                          />
                        </div>
                        <p className="text-xs text-muted-foreground mt-1">
                          {upload.progress}% uploaded
                        </p>
                      </div>
                    )}
                    
                    {upload.status === 'error' && upload.error && (
                      <p className="text-xs text-red-500 mt-1">
                        {upload.error}
                      </p>
                    )}
                  </div>
                  
                  <Button
                    variant="ghost"
                    size="icon"
                    onClick={() => removeFile(index)}
                    className="h-8 w-8"
                  >
                    <X className="h-4 w-4" />
                  </Button>
                </div>
              ))}
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  )
} 