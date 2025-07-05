import { useParams } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import { ScrollArea } from "@/components/ui/scroll-area";
import { apiClient } from "@/lib/api";

type DocMeta = {
  id: string;
  title: string;
  createdAt: string;
  size: string;
  type: string;
  status: string;
  tags: string[];
  content: string;
};

async function fetchDoc(id: string): Promise<DocMeta> {
  const res: any = await apiClient.getDocument(id);
  const doc = res.data;
  return {
    id: doc.id,
    title: doc.filename || doc.id,
    createdAt: doc.created_at,
    size: `${doc.size_bytes || 0} bytes`,
    type: doc.document_type || "unknown",
    status: doc.status || "unknown",
    tags: [],
    content: doc.content || "",
  };
}

export function DocumentDetail() {
  const { id } = useParams<{ id: string }>();
  const { data, isLoading } = useQuery({
    queryKey: ["document", id],
    queryFn: () => fetchDoc(id || ""),
    enabled: !!id,
  });

  if (isLoading || !data) {
    return <p className="p-6">Loading...</p>;
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">{data.title}</h1>
        <p className="text-muted-foreground">Detailed analysis and metadata</p>
      </div>

      <div className="grid gap-6 lg:grid-cols-3">
        {/* Metadata */}
        <Card>
          <CardHeader>
            <CardTitle>Metadata</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2 text-sm">
            <div className="flex justify-between">
              <span>ID</span>
              <span>{data.id}</span>
            </div>
            <div className="flex justify-between">
              <span>Created</span>
              <span>{data.createdAt}</span>
            </div>
            <div className="flex justify-between">
              <span>Type</span>
              <span>{data.type}</span>
            </div>
            <div className="flex justify-between">
              <span>Size</span>
              <span>{data.size}</span>
            </div>
            <div className="flex justify-between">
              <span>Status</span>
              <Badge variant="secondary">{data.status}</Badge>
            </div>
            <div>
              <span className="font-medium">Tags</span>
              <div className="flex flex-wrap gap-1 mt-1">
                {data.tags.map((tag) => (
                  <Badge key={tag}>{tag}</Badge>
                ))}
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Content Preview */}
        <Card className="lg:col-span-2">
          <CardHeader>
            <CardTitle>Content Preview</CardTitle>
            <CardDescription className="truncate">First 10,000 characters</CardDescription>
          </CardHeader>
          <CardContent>
            <ScrollArea className="h-[500px] pr-4">
              <pre className="whitespace-pre-wrap text-sm leading-relaxed">
                {data.content}
              </pre>
            </ScrollArea>
          </CardContent>
        </Card>
      </div>
    </div>
  );
} 