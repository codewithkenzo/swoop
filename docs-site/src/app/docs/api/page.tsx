import dynamic from 'next/dynamic';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import Link from 'next/link';
import { ExternalLink, Code, Zap, Shield } from 'lucide-react';

// Dynamically import SwaggerUI to avoid SSR issues
const SwaggerUI = dynamic(() => import('swagger-ui-react'), { 
  ssr: false,
  loading: () => (
    <div className="flex items-center justify-center h-64">
      <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
    </div>
  )
});

export default function APIPage() {
  return (
    <div className="container mx-auto px-4 py-8">
      {/* Header */}
      <div className="mb-8">
        <div className="flex items-center gap-2 mb-4">
          <Badge className="bg-green-100 text-green-800 dark:bg-green-900/20 dark:text-green-400">
            REST API v0.2.0
          </Badge>
          <Badge variant="outline">OpenAPI 3.0</Badge>
        </div>
        <h1 className="text-4xl font-bold mb-4">API Reference</h1>
        <p className="text-xl text-muted-foreground max-w-3xl">
          Complete REST API documentation for Swoop. All endpoints are authenticated and support JSON responses with comprehensive error handling.
        </p>
      </div>

      {/* Quick Links */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <Card>
          <CardHeader className="pb-4">
            <div className="flex items-center gap-2">
              <Zap className="h-5 w-5 text-primary" />
              <CardTitle className="text-lg">Quick Start</CardTitle>
            </div>
          </CardHeader>
          <CardContent>
            <CardDescription className="mb-4">
              Get your API key and make your first request in minutes
            </CardDescription>
            <Button asChild variant="outline" size="sm">
              <Link href="/docs/get-started/quick-start">
                Get Started
                <ExternalLink className="ml-2 h-4 w-4" />
              </Link>
            </Button>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-4">
            <div className="flex items-center gap-2">
              <Code className="h-5 w-5 text-primary" />
              <CardTitle className="text-lg">Integration Examples</CardTitle>
            </div>
          </CardHeader>
          <CardContent>
            <CardDescription className="mb-4">
              JavaScript, Python, and cURL examples for common use cases
            </CardDescription>
            <Button asChild variant="outline" size="sm">
              <Link href="/docs/backend/integration">
                View Examples
                <ExternalLink className="ml-2 h-4 w-4" />
              </Link>
            </Button>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="pb-4">
            <div className="flex items-center gap-2">
              <Shield className="h-5 w-5 text-primary" />
              <CardTitle className="text-lg">Authentication</CardTitle>
            </div>
          </CardHeader>
          <CardContent>
            <CardDescription className="mb-4">
              API key authentication and security best practices
            </CardDescription>
            <Button asChild variant="outline" size="sm">
              <Link href="/docs/backend/security">
                Security Guide
                <ExternalLink className="ml-2 h-4 w-4" />
              </Link>
            </Button>
          </CardContent>
        </Card>
      </div>

      {/* Base URL and Authentication */}
      <div className="mb-8 space-y-4">
        <Card>
          <CardHeader>
            <CardTitle>Base URL</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="bg-muted rounded-md p-4 font-mono text-sm">
              https://api.swoop.dev
            </div>
            <p className="text-sm text-muted-foreground mt-2">
              For local development: <code>http://localhost:8080</code>
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Authentication</CardTitle>
          </CardHeader>
          <CardContent>
            <p className="mb-4">All API requests require authentication via Bearer token:</p>
            <div className="bg-muted rounded-md p-4 font-mono text-sm">
              Authorization: Bearer your-api-key-here
            </div>
            <p className="text-sm text-muted-foreground mt-2">
              Get your API key by running the Swoop server locally or signing up for our cloud service.
            </p>
          </CardContent>
        </Card>
      </div>

      {/* Rate Limits */}
      <Card className="mb-8">
        <CardHeader>
          <CardTitle>Rate Limits</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            <div>
              <div className="font-semibold">Standard Endpoints</div>
              <div className="text-sm text-muted-foreground">100 requests per minute</div>
            </div>
            <div>
              <div className="font-semibold">Upload Endpoints</div>
              <div className="text-sm text-muted-foreground">10 requests per minute</div>
            </div>
            <div>
              <div className="font-semibold">Chat Endpoints</div>
              <div className="text-sm text-muted-foreground">50 requests per minute</div>
            </div>
          </div>
        </CardContent>
      </Card>

      {/* Interactive API Documentation */}
      <Card>
        <CardHeader>
          <CardTitle>Interactive API Explorer</CardTitle>
          <CardDescription>
            Try out API endpoints directly from this page. You can authenticate and test requests in real-time.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="border rounded-lg overflow-hidden">
            <SwaggerUI
              url="/docs/openapi.yaml"
              deepLinking={true}
              displayOperationId={false}
              defaultModelsExpandDepth={1}
              defaultModelExpandDepth={1}
              docExpansion="list"
              filter={true}
              showExtensions={true}
              showCommonExtensions={true}
              tryItOutEnabled={true}
            />
          </div>
        </CardContent>
      </Card>
    </div>
  );
}

export const metadata = {
  title: 'API Reference',
  description: 'Complete REST API documentation for Swoop with interactive examples',
};