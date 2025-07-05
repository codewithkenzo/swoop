import { 
  Document, 
  AnalysisResults, 
  AnalysisConfig, 
  CrawlJob, 
  ApiResponse, 
  PaginatedResponse,
  DocumentFilter 
} from '@/types'

import { API_BASE_URL } from './env'

class ApiClient {
  private async request<T>(
    endpoint: string, 
    options: RequestInit = {}
  ): Promise<T> {
    const url = `${API_BASE_URL}${endpoint}`
    
    const response = await fetch(url, {
      headers: {
        'Content-Type': 'application/json',
        ...options.headers,
      },
      ...options,
    })

    if (!response.ok) {
      throw new Error(`API Error: ${response.status} ${response.statusText}`)
    }

    return response.json()
  }

  // Document Management
  async getDocuments(
    page = 1, 
    perPage = 20, 
    filter?: DocumentFilter
  ): Promise<PaginatedResponse<Document>> {
    const params = new URLSearchParams({
      page: page.toString(),
      per_page: perPage.toString(),
    })

    if (filter?.searchQuery) {
      params.append('search', filter.searchQuery)
    }
    if (filter?.category) {
      params.append('category', filter.category)
    }
    if (filter?.tags?.length) {
      filter.tags.forEach(tag => params.append('tags', tag))
    }

    return this.request<PaginatedResponse<Document>>(`/documents?${params}`)
  }

  async getDocument(id: string): Promise<ApiResponse<Document>> {
    return this.request<ApiResponse<Document>>(`/documents/${id}`)
  }

  async getDocumentPreview(id: string): Promise<ApiResponse<{ preview: string }>> {
    return this.request<ApiResponse<{ preview: string }>>(`/documents/${id}/preview`)
  }

  async reprocessDocument(id: string): Promise<ApiResponse<string>> {
    return this.request<ApiResponse<string>>(`/documents/${id}/reprocess`, {
      method: 'POST',
    })
  }

  async deleteDocument(id: string): Promise<ApiResponse<null>> {
    return this.request<ApiResponse<null>>(`/documents/${id}`, {
      method: 'DELETE',
    })
  }

  async updateDocument(
    id: string, 
    updates: Partial<Document>
  ): Promise<ApiResponse<Document>> {
    return this.request<ApiResponse<Document>>(`/documents/${id}`, {
      method: 'PUT',
      body: JSON.stringify(updates),
    })
  }

  // File Upload
  async uploadFile(
    file: File, 
    onProgress?: (progress: number) => void
  ): Promise<ApiResponse<Document>> {
    const formData = new FormData()
    formData.append('file', file)

    return new Promise((resolve, reject) => {
      const xhr = new XMLHttpRequest()

      xhr.upload.addEventListener('progress', (e) => {
        if (e.lengthComputable && onProgress) {
          const progress = (e.loaded / e.total) * 100
          onProgress(progress)
        }
      })

      xhr.addEventListener('load', () => {
        if (xhr.status >= 200 && xhr.status < 300) {
          resolve(JSON.parse(xhr.responseText))
        } else {
          reject(new Error(`Upload failed: ${xhr.status} ${xhr.statusText}`))
        }
      })

      xhr.addEventListener('error', () => {
        reject(new Error('Upload failed'))
      })

      xhr.open('POST', `${API_BASE_URL}/documents/upload`)
      xhr.send(formData)
    })
  }

  // AI Analysis
  async analyzeDocument(
    documentId: string, 
    config: AnalysisConfig
  ): Promise<ApiResponse<AnalysisResults>> {
    return this.request<ApiResponse<AnalysisResults>>(`/documents/${documentId}/analyze`, {
      method: 'POST',
      body: JSON.stringify(config),
    })
  }

  async reanalyzeDocument(
    documentId: string, 
    config: AnalysisConfig
  ): Promise<ApiResponse<AnalysisResults>> {
    return this.request<ApiResponse<AnalysisResults>>(`/documents/${documentId}/reanalyze`, {
      method: 'POST',
      body: JSON.stringify(config),
    })
  }

  // Web Crawling
  async startCrawl(url: string): Promise<ApiResponse<{ job_id: string }>> {
    return this.request<ApiResponse<{ job_id: string }>>('/crawl', {
      method: 'POST',
      body: JSON.stringify({ url }),
    })
  }

  async getCrawlJob(jobId: string): Promise<ApiResponse<CrawlJob>> {
    return this.request<ApiResponse<CrawlJob>>(`/crawl/${jobId}`)
  }

  async getCrawlJobs(): Promise<ApiResponse<CrawlJob[]>> {
    return this.request<ApiResponse<CrawlJob[]>>('/crawl')
  }

  async stopCrawl(jobId: string): Promise<ApiResponse<null>> {
    return this.request<ApiResponse<null>>(`/crawl/${jobId}/stop`, {
      method: 'POST',
    })
  }

  async getCrawlResults(jobId: string): Promise<ApiResponse<any>> {
    return this.request<ApiResponse<any>>(`/crawl/${jobId}/results`)
  }

  // Search
  async searchDocuments(
    query: string, 
    options?: {
      semantic?: boolean
      limit?: number
    }
  ): Promise<ApiResponse<Document[]>> {
    const params = new URLSearchParams({
      q: query,
      semantic: (options?.semantic ?? false).toString(),
      limit: (options?.limit ?? 10).toString(),
    })

    return this.request<ApiResponse<Document[]>>(`/search?${params}`)
  }

  // Health Check
  async healthCheck(): Promise<ApiResponse<{ status: string; version: string }>> {
    return this.request<ApiResponse<{ status: string; version: string }>>('/health')
  }

  // Stats & Metrics
  async getStats() {
    return this.request<ApiResponse<any>>('/stats')
  }

  async getMetrics() {
    return this.request<ApiResponse<any>>('/metrics')
  }
}

export const apiClient = new ApiClient()
export default apiClient 