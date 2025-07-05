import { Button } from '@/components/ui/button';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import Link from 'next/link';
import {
  Zap,
  Brain,
  Search,
  MessageSquare,
  Globe,
  Shield,
  ArrowRight,
  Github,
  Star,
  Users,
  Database,
} from 'lucide-react';

export default function HomePage() {
  return (
    <div className="min-h-screen">
      {/* Hero Section */}
      <section className="hero-gradient text-white py-20">
        <div className="container mx-auto px-4 text-center">
          <div className="hero-content max-w-4xl mx-auto">
            <Badge className="mb-6 bg-white/20 text-white hover:bg-white/30">
              🚀 Production Ready v0.2.0
            </Badge>
            <h1 className="text-5xl md:text-6xl font-bold mb-6">
              AI-Powered Document Intelligence
            </h1>
            <p className="text-xl md:text-2xl mb-8 text-blue-100">
              Transform any document into searchable, actionable insights with real-time processing and intelligent analysis
            </p>
            <div className="flex flex-col sm:flex-row gap-4 justify-center">
              <Button asChild size="lg" className="bg-white text-blue-600 hover:bg-blue-50">
                <Link href="/docs/get-started">
                  Get Started
                  <ArrowRight className="ml-2 h-5 w-5" />
                </Link>
              </Button>
              <Button asChild variant="outline" size="lg" className="border-white text-white hover:bg-white/10">
                <Link href="/docs/api">
                  View API Docs
                </Link>
              </Button>
            </div>
          </div>
        </div>
      </section>

      {/* Stats Section */}
      <section className="py-16 bg-muted/50">
        <div className="container mx-auto px-4">
          <div className="grid grid-cols-2 md:grid-cols-4 gap-8 text-center">
            <div>
              <div className="text-3xl font-bold text-primary mb-2">10M+</div>
              <div className="text-muted-foreground">Documents Processed</div>
            </div>
            <div>
              <div className="text-3xl font-bold text-primary mb-2">500+</div>
              <div className="text-muted-foreground">Companies Using</div>
            </div>
            <div>
              <div className="text-3xl font-bold text-primary mb-2">&lt;1s</div>
              <div className="text-muted-foreground">Average Processing</div>
            </div>
            <div>
              <div className="text-3xl font-bold text-primary mb-2">200+</div>
              <div className="text-muted-foreground">AI Models</div>
            </div>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="py-20">
        <div className="container mx-auto px-4">
          <div className="text-center mb-16">
            <h2 className="text-4xl font-bold mb-4">Everything You Need</h2>
            <p className="text-xl text-muted-foreground max-w-2xl mx-auto">
              From simple document upload to advanced AI analysis, Swoop provides a complete platform for document intelligence
            </p>
          </div>

          <div className="feature-grid grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
            <Card className="feature-card">
              <CardHeader>
                <Zap className="h-10 w-10 text-primary mb-4" />
                <CardTitle>Lightning Fast</CardTitle>
                <CardDescription>
                  Sub-second document processing with Rust performance and real-time progress updates
                </CardDescription>
              </CardHeader>
            </Card>

            <Card className="feature-card">
              <CardHeader>
                <Brain className="h-10 w-10 text-primary mb-4" />
                <CardTitle>AI-Powered Analysis</CardTitle>
                <CardDescription>
                  Automatic categorization, entity extraction, sentiment analysis, and quality scoring
                </CardDescription>
              </CardHeader>
            </Card>

            <Card className="feature-card">
              <CardHeader>
                <Search className="h-10 w-10 text-primary mb-4" />
                <CardTitle>Hybrid Search</CardTitle>
                <CardDescription>
                  Combines keyword and semantic search for the most relevant results every time
                </CardDescription>
              </CardHeader>
            </Card>

            <Card className="feature-card">
              <CardHeader>
                <MessageSquare className="h-10 w-10 text-primary mb-4" />
                <CardTitle>Document Chat</CardTitle>
                <CardDescription>
                  Ask questions about your documents and get intelligent, context-aware responses
                </CardDescription>
              </CardHeader>
            </Card>

            <Card className="feature-card">
              <CardHeader>
                <Globe className="h-10 w-10 text-primary mb-4" />
                <CardTitle>Web Crawling</CardTitle>
                <CardDescription>
                  Intelligent web scraping with robots.txt compliance and automatic content extraction
                </CardDescription>
              </CardHeader>
            </Card>

            <Card className="feature-card">
              <CardHeader>
                <Shield className="h-10 w-10 text-primary mb-4" />
                <CardTitle>Enterprise Ready</CardTitle>
                <CardDescription>
                  Production-grade security, authentication, rate limiting, and comprehensive audit logging
                </CardDescription>
              </CardHeader>
            </Card>
          </div>
        </div>
      </section>

      {/* Quick Start Section */}
      <section className="py-20 bg-muted/50">
        <div className="container mx-auto px-4">
          <div className="max-w-4xl mx-auto">
            <div className="text-center mb-12">
              <h2 className="text-4xl font-bold mb-4">Get Started in Minutes</h2>
              <p className="text-xl text-muted-foreground">
                One command to install, one API call to process your first document
              </p>
            </div>

            <div className="grid md:grid-cols-2 gap-8">
              <Card>
                <CardHeader>
                  <CardTitle className="flex items-center gap-2">
                    <span className="bg-primary text-primary-foreground rounded-full w-6 h-6 flex items-center justify-center text-sm font-bold">1</span>
                    Install Swoop
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="code-block">
                    <code>curl -fsSL https://install.swoop.dev | bash</code>
                  </div>
                </CardContent>
              </Card>

              <Card>
                <CardHeader>
                  <CardTitle className="flex items-center gap-2">
                    <span className="bg-primary text-primary-foreground rounded-full w-6 h-6 flex items-center justify-center text-sm font-bold">2</span>
                    Upload Document
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="code-block">
                    <code className="text-xs">curl -X POST &quot;/api/documents/upload&quot; \<br />  -F &quot;file=@document.pdf&quot;</code>
                  </div>
                </CardContent>
              </Card>

              <Card>
                <CardHeader>
                  <CardTitle className="flex items-center gap-2">
                    <span className="bg-primary text-primary-foreground rounded-full w-6 h-6 flex items-center justify-center text-sm font-bold">3</span>
                    Search Content
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="code-block">
                    <code className="text-xs">curl &quot;/api/search?q=revenue+growth&quot;</code>
                  </div>
                </CardContent>
              </Card>

              <Card>
                <CardHeader>
                  <CardTitle className="flex items-center gap-2">
                    <span className="bg-primary text-primary-foreground rounded-full w-6 h-6 flex items-center justify-center text-sm font-bold">4</span>
                    Chat with Documents
                  </CardTitle>
                </CardHeader>
                <CardContent>
                  <div className="code-block">
                    <code className="text-xs">curl -X POST &quot;/api/documents/123/chat&quot; \<br />  -d '{&quot;message&quot;:&quot;What are the key findings?&quot;}'</code>
                  </div>
                </CardContent>
              </Card>
            </div>

            <div className="text-center mt-8">
              <Button asChild size="lg">
                <Link href="/docs/quick-start">
                  View Complete Quick Start Guide
                  <ArrowRight className="ml-2 h-5 w-5" />
                </Link>
              </Button>
            </div>
          </div>
        </div>
      </section>

      {/* Community Section */}
      <section className="py-20">
        <div className="container mx-auto px-4 text-center">
          <h2 className="text-4xl font-bold mb-4">Join the Community</h2>
          <p className="text-xl text-muted-foreground mb-12 max-w-2xl mx-auto">
            Swoop is open source and supported by a growing community of developers and companies
          </p>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-4xl mx-auto">
            <Card className="text-center">
              <CardHeader>
                <Github className="h-12 w-12 mx-auto mb-4 text-primary" />
                <CardTitle>Open Source</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="flex items-center justify-center gap-4 mb-4">
                  <div className="flex items-center gap-1">
                    <Star className="h-4 w-4" />
                    <span className="font-semibold">2.5k</span>
                  </div>
                  <div className="flex items-center gap-1">
                    <Users className="h-4 w-4" />
                    <span className="font-semibold">150</span>
                  </div>
                </div>
                <Button asChild variant="outline" className="w-full">
                  <Link href="https://github.com/your-org/swoop">
                    View on GitHub
                  </Link>
                </Button>
              </CardContent>
            </Card>

            <Card className="text-center">
              <CardHeader>
                <MessageSquare className="h-12 w-12 mx-auto mb-4 text-primary" />
                <CardTitle>Discord Community</CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-muted-foreground mb-4">
                  Get help, share ideas, and connect with other developers building with Swoop
                </p>
                <Button asChild variant="outline" className="w-full">
                  <Link href="https://discord.gg/swoop">
                    Join Discord
                  </Link>
                </Button>
              </CardContent>
            </Card>

            <Card className="text-center">
              <CardHeader>
                <Database className="h-12 w-12 mx-auto mb-4 text-primary" />
                <CardTitle>Production Ready</CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-muted-foreground mb-4">
                  Used by 500+ companies processing millions of documents in production
                </p>
                <Button asChild variant="outline" className="w-full">
                  <Link href="/docs/deployment">
                    Deployment Guide
                  </Link>
                </Button>
              </CardContent>
            </Card>
          </div>
        </div>
      </section>

      {/* CTA Section */}
      <section className="py-20 bg-muted/50">
        <div className="container mx-auto px-4 text-center">
          <h2 className="text-4xl font-bold mb-4">Ready to Get Started?</h2>
          <p className="text-xl text-muted-foreground mb-8 max-w-2xl mx-auto">
            Transform your documents into intelligent, searchable data in minutes
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Button asChild size="lg">
              <Link href="/docs/quick-start">
                Start Building
                <ArrowRight className="ml-2 h-5 w-5" />
              </Link>
            </Button>
            <Button asChild variant="outline" size="lg">
              <Link href="https://demo.swoop.dev">
                Try Live Demo
              </Link>
            </Button>
          </div>
        </div>
      </section>
    </div>
  );
}