"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Slider } from "@/components/ui/slider";
import { Switch } from "@/components/ui/switch";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import { 
  Globe, 
  Clock, 
  Shield, 
  Link, 
  Settings, 
  FileText, 
  AlertCircle,
  CheckCircle,
  Info,
  Plus,
  Trash2,
  Save
} from "lucide-react";

interface CrawlerConfig {
  maxDepth: number;
  crawlDelay: number;
  respectRobotsTxt: boolean;
  followRedirects: boolean;
  maxPages: number;
  timeout: number;
  userAgent: string;
  domainFilters: DomainFilter[];
  linkTypes: LinkType[];
  excludePatterns: string[];
  includePatterns: string[];
}

interface DomainFilter {
  id: string;
  domain: string;
  type: 'include' | 'exclude';
}

interface LinkType {
  id: string;
  type: string;
  enabled: boolean;
  description: string;
}

const defaultLinkTypes: LinkType[] = [
  { id: '1', type: 'internal', enabled: true, description: 'Links within the same domain' },
  { id: '2', type: 'external', enabled: false, description: 'Links to external domains' },
  { id: '3', type: 'subdomain', enabled: true, description: 'Links to subdomains' },
  { id: '4', type: 'mailto', enabled: false, description: 'Email links' },
  { id: '5', type: 'tel', enabled: false, description: 'Telephone links' },
  { id: '6', type: 'file', enabled: true, description: 'Direct file links (PDF, DOC, etc.)' }
];

const defaultConfig: CrawlerConfig = {
  maxDepth: 3,
  crawlDelay: 1000,
  respectRobotsTxt: true,
  followRedirects: true,
  maxPages: 1000,
  timeout: 30000,
  userAgent: 'Swoop Document Intelligence Bot/1.0',
  domainFilters: [],
  linkTypes: defaultLinkTypes,
  excludePatterns: [],
  includePatterns: []
};

function CrawlerConfigurationComponent() {
  const [config, setConfig] = useState<CrawlerConfig>(defaultConfig);
  const [newDomain, setNewDomain] = useState('');
  const [newExcludePattern, setNewExcludePattern] = useState('');
  const [newIncludePattern, setNewIncludePattern] = useState('');

  const updateConfig = (updates: Partial<CrawlerConfig>) => {
    setConfig(prev => ({ ...prev, ...updates }));
  };

  const addDomainFilter = (type: 'include' | 'exclude') => {
    if (!newDomain.trim()) return;
    
    const filter: DomainFilter = {
      id: Date.now().toString(),
      domain: newDomain.trim(),
      type
    };
    
    updateConfig({
      domainFilters: [...config.domainFilters, filter]
    });
    setNewDomain('');
  };

  const removeDomainFilter = (id: string) => {
    updateConfig({
      domainFilters: config.domainFilters.filter(f => f.id !== id)
    });
  };

  const toggleLinkType = (id: string) => {
    updateConfig({
      linkTypes: config.linkTypes.map(lt => 
        lt.id === id ? { ...lt, enabled: !lt.enabled } : lt
      )
    });
  };

  const addPattern = (type: 'exclude' | 'include') => {
    const pattern = type === 'exclude' ? newExcludePattern : newIncludePattern;
    if (!pattern.trim()) return;

    const currentPatterns = type === 'exclude' ? config.excludePatterns : config.includePatterns;
    const updatedPatterns = [...currentPatterns, pattern.trim()];

    updateConfig({
      [type === 'exclude' ? 'excludePatterns' : 'includePatterns']: updatedPatterns
    });

    if (type === 'exclude') {
      setNewExcludePattern('');
    } else {
      setNewIncludePattern('');
    }
  };

  const removePattern = (pattern: string, type: 'exclude' | 'include') => {
    const currentPatterns = type === 'exclude' ? config.excludePatterns : config.includePatterns;
    const updatedPatterns = currentPatterns.filter(p => p !== pattern);

    updateConfig({
      [type === 'exclude' ? 'excludePatterns' : 'includePatterns']: updatedPatterns
    });
  };

  const handleSave = () => {
    // Save configuration logic here
    console.log('Saving crawler configuration:', config);
  };

  const resetToDefaults = () => {
    setConfig(defaultConfig);
  };

  return (
    <div className="w-full max-w-4xl mx-auto p-6 space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-foreground">Crawler Configuration</h1>
          <p className="text-muted-foreground mt-2">
            Configure advanced crawling settings for the Swoop document intelligence platform
          </p>
        </div>
        <div className="flex gap-2">
          <Button variant="outline" onClick={resetToDefaults}>
            Reset to Defaults
          </Button>
          <Button onClick={handleSave} className="flex items-center gap-2">
            <Save className="h-4 w-4" />
            Save Configuration
          </Button>
        </div>
      </div>

      <Tabs defaultValue="general" className="w-full">
        <TabsList className="grid w-full grid-cols-4">
          <TabsTrigger value="general" className="flex items-center gap-2">
            <Settings className="h-4 w-4" />
            General
          </TabsTrigger>
          <TabsTrigger value="domains" className="flex items-center gap-2">
            <Globe className="h-4 w-4" />
            Domains
          </TabsTrigger>
          <TabsTrigger value="links" className="flex items-center gap-2">
            <Link className="h-4 w-4" />
            Link Types
          </TabsTrigger>
          <TabsTrigger value="patterns" className="flex items-center gap-2">
            <FileText className="h-4 w-4" />
            Patterns
          </TabsTrigger>
        </TabsList>

        <TabsContent value="general" className="space-y-6">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Settings className="h-5 w-5" />
                  Basic Settings
                </CardTitle>
                <CardDescription>
                  Configure basic crawling parameters
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="space-y-2">
                  <Label htmlFor="maxDepth">Maximum Crawl Depth: {config.maxDepth}</Label>
                  <Slider
                    id="maxDepth"
                    min={1}
                    max={10}
                    step={1}
                    value={[config.maxDepth]}
                    onValueChange={(value) => updateConfig({ maxDepth: value[0] })}
                    className="w-full"
                  />
                  <p className="text-sm text-muted-foreground">
                    How many levels deep to crawl from the starting URL
                  </p>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="maxPages">Maximum Pages: {config.maxPages}</Label>
                  <Slider
                    id="maxPages"
                    min={10}
                    max={10000}
                    step={10}
                    value={[config.maxPages]}
                    onValueChange={(value) => updateConfig({ maxPages: value[0] })}
                    className="w-full"
                  />
                  <p className="text-sm text-muted-foreground">
                    Maximum number of pages to crawl
                  </p>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="userAgent">User Agent</Label>
                  <Input
                    id="userAgent"
                    value={config.userAgent}
                    onChange={(e) => updateConfig({ userAgent: e.target.value })}
                    placeholder="User agent string"
                  />
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Clock className="h-5 w-5" />
                  Timing & Performance
                </CardTitle>
                <CardDescription>
                  Configure timing and performance settings
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="space-y-2">
                  <Label htmlFor="crawlDelay">Crawl Delay: {config.crawlDelay}ms</Label>
                  <Slider
                    id="crawlDelay"
                    min={0}
                    max={5000}
                    step={100}
                    value={[config.crawlDelay]}
                    onValueChange={(value) => updateConfig({ crawlDelay: value[0] })}
                    className="w-full"
                  />
                  <p className="text-sm text-muted-foreground">
                    Delay between requests to be respectful to servers
                  </p>
                </div>

                <div className="space-y-2">
                  <Label htmlFor="timeout">Request Timeout: {config.timeout}ms</Label>
                  <Slider
                    id="timeout"
                    min={5000}
                    max={120000}
                    step={1000}
                    value={[config.timeout]}
                    onValueChange={(value) => updateConfig({ timeout: value[0] })}
                    className="w-full"
                  />
                  <p className="text-sm text-muted-foreground">
                    Maximum time to wait for each request
                  </p>
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2">
                  <Shield className="h-5 w-5" />
                  Compliance & Behavior
                </CardTitle>
                <CardDescription>
                  Configure compliance and crawling behavior
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="flex items-center justify-between">
                  <div className="space-y-0.5">
                    <Label htmlFor="respectRobots">Respect robots.txt</Label>
                    <p className="text-sm text-muted-foreground">
                      Follow robots.txt directives
                    </p>
                  </div>
                  <Switch
                    id="respectRobots"
                    checked={config.respectRobotsTxt}
                    onCheckedChange={(checked) => updateConfig({ respectRobotsTxt: checked })}
                  />
                </div>

                <Separator />

                <div className="flex items-center justify-between">
                  <div className="space-y-0.5">
                    <Label htmlFor="followRedirects">Follow Redirects</Label>
                    <p className="text-sm text-muted-foreground">
                      Automatically follow HTTP redirects
                    </p>
                  </div>
                  <Switch
                    id="followRedirects"
                    checked={config.followRedirects}
                    onCheckedChange={(checked) => updateConfig({ followRedirects: checked })}
                  />
                </div>
              </CardContent>
            </Card>
          </div>
        </TabsContent>

        <TabsContent value="domains" className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Globe className="h-5 w-5" />
                Domain Filtering
              </CardTitle>
              <CardDescription>
                Control which domains the crawler can access
              </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
              <div className="flex gap-2">
                <Input
                  placeholder="Enter domain (e.g., example.com)"
                  value={newDomain}
                  onChange={(e) => setNewDomain(e.target.value)}
                  onKeyDown={(e) => {
                    if (e.key === 'Enter') {
                      addDomainFilter('include');
                    }
                  }}
                />
                <Button onClick={() => addDomainFilter('include')} variant="default">
                  <Plus className="h-4 w-4 mr-1" />
                  Include
                </Button>
                <Button onClick={() => addDomainFilter('exclude')} variant="destructive">
                  <Plus className="h-4 w-4 mr-1" />
                  Exclude
                </Button>
              </div>

              <div className="space-y-2">
                <Label>Domain Filters</Label>
                {config.domainFilters.length === 0 ? (
                  <div className="text-center py-8 text-muted-foreground">
                    <Globe className="h-8 w-8 mx-auto mb-2 opacity-50" />
                    No domain filters configured. All domains will be crawled.
                  </div>
                ) : (
                  <div className="space-y-2">
                    {config.domainFilters.map((filter) => (
                      <div key={filter.id} className="flex items-center justify-between p-2 border rounded">
                        <div className="flex items-center gap-2">
                          <Badge variant={filter.type === 'include' ? 'default' : 'destructive'}>
                            {filter.type}
                          </Badge>
                          <span className="font-mono">{filter.domain}</span>
                        </div>
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => removeDomainFilter(filter.id)}
                        >
                          <Trash2 className="h-4 w-4" />
                        </Button>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="links" className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Link className="h-5 w-5" />
                Link Type Filtering
              </CardTitle>
              <CardDescription>
                Choose which types of links to follow during crawling
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                {config.linkTypes.map((linkType) => (
                  <div key={linkType.id} className="flex items-center justify-between p-3 border rounded">
                    <div className="space-y-1">
                      <div className="flex items-center gap-2">
                        <span className="font-medium capitalize">{linkType.type}</span>
                        <Badge variant={linkType.enabled ? 'default' : 'secondary'}>
                          {linkType.enabled ? 'Enabled' : 'Disabled'}
                        </Badge>
                      </div>
                      <p className="text-sm text-muted-foreground">{linkType.description}</p>
                    </div>
                    <Switch
                      checked={linkType.enabled}
                      onCheckedChange={() => toggleLinkType(linkType.id)}
                    />
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="patterns" className="space-y-6">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2 text-red-600">
                  <AlertCircle className="h-5 w-5" />
                  Exclude Patterns
                </CardTitle>
                <CardDescription>
                  URL patterns to exclude from crawling (regex supported)
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="flex gap-2">
                  <Input
                    placeholder="e.g., /admin/*, *.pdf"
                    value={newExcludePattern}
                    onChange={(e) => setNewExcludePattern(e.target.value)}
                    onKeyDown={(e) => {
                      if (e.key === 'Enter') {
                        addPattern('exclude');
                      }
                    }}
                  />
                  <Button onClick={() => addPattern('exclude')} variant="destructive">
                    <Plus className="h-4 w-4" />
                  </Button>
                </div>

                <div className="space-y-2">
                  {config.excludePatterns.length === 0 ? (
                    <p className="text-sm text-muted-foreground text-center py-4">
                      No exclude patterns defined
                    </p>
                  ) : (
                    config.excludePatterns.map((pattern, index) => (
                      <div key={index} className="flex items-center justify-between p-2 border rounded">
                        <code className="text-sm bg-muted px-2 py-1 rounded">{pattern}</code>
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => removePattern(pattern, 'exclude')}
                        >
                          <Trash2 className="h-4 w-4" />
                        </Button>
                      </div>
                    ))
                  )}
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle className="flex items-center gap-2 text-green-600">
                  <CheckCircle className="h-5 w-5" />
                  Include Patterns
                </CardTitle>
                <CardDescription>
                  URL patterns to specifically include (regex supported)
                </CardDescription>
              </CardHeader>
              <CardContent className="space-y-4">
                <div className="flex gap-2">
                  <Input
                    placeholder="e.g., /docs/*, /blog/*"
                    value={newIncludePattern}
                    onChange={(e) => setNewIncludePattern(e.target.value)}
                    onKeyDown={(e) => {
                      if (e.key === 'Enter') {
                        addPattern('include');
                      }
                    }}
                  />
                  <Button onClick={() => addPattern('include')} variant="default">
                    <Plus className="h-4 w-4" />
                  </Button>
                </div>

                <div className="space-y-2">
                  {config.includePatterns.length === 0 ? (
                    <p className="text-sm text-muted-foreground text-center py-4">
                      No include patterns defined
                    </p>
                  ) : (
                    config.includePatterns.map((pattern, index) => (
                      <div key={index} className="flex items-center justify-between p-2 border rounded">
                        <code className="text-sm bg-muted px-2 py-1 rounded">{pattern}</code>
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => removePattern(pattern, 'include')}
                        >
                          <Trash2 className="h-4 w-4" />
                        </Button>
                      </div>
                    ))
                  )}
                </div>
              </CardContent>
            </Card>
          </div>

          <Card>
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Info className="h-5 w-5" />
                Pattern Examples
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
                <div>
                  <h4 className="font-medium mb-2">Common Exclude Patterns:</h4>
                  <ul className="space-y-1 text-muted-foreground">
                    <li><code>/admin/*</code> - Admin pages</li>
                    <li><code>*.pdf</code> - PDF files</li>
                    <li><code>/api/*</code> - API endpoints</li>
                    <li><code>*login*</code> - Login pages</li>
                  </ul>
                </div>
                <div>
                  <h4 className="font-medium mb-2">Common Include Patterns:</h4>
                  <ul className="space-y-1 text-muted-foreground">
                    <li><code>/docs/*</code> - Documentation</li>
                    <li><code>/blog/*</code> - Blog posts</li>
                    <li><code>/products/*</code> - Product pages</li>
                    <li><code>*.html</code> - HTML pages only</li>
                  </ul>
                </div>
              </div>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  );
}

export default CrawlerConfigurationComponent; 