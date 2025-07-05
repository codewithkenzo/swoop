import * as React from "react";
import { useQuery } from "@tanstack/react-query";
import {
  ColumnDef,
  flexRender,
  getCoreRowModel,
  getPaginationRowModel,
  useReactTable,
} from "@tanstack/react-table";
import { Search as SearchIcon, FileText, ChevronDown } from "lucide-react";

import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Badge } from "@/components/ui/badge";
import { apiClient } from "@/lib/api";

type SearchResult = {
  id: string;
  title: string;
  snippet: string;
  score: number;
};

async function searchDocuments(query: string): Promise<SearchResult[]> {
  if (!query) return [];
  const res: any = await apiClient.searchDocuments(query, { semantic: true, limit: 20 });
  return res.data || [];
}

const columns: ColumnDef<SearchResult>[] = [
  {
    accessorKey: "title",
    header: "Title",
    cell: ({ row }) => (
      <div className="flex items-center gap-2">
        <FileText className="h-4 w-4 text-muted-foreground" />
        <span className="font-medium">{row.original.title}</span>
      </div>
    ),
  },
  {
    accessorKey: "snippet",
    header: "Snippet",
    cell: ({ row }) => (
      <p className="line-clamp-2 text-muted-foreground text-sm max-w-prose">
        {row.original.snippet}
      </p>
    ),
  },
  {
    accessorKey: "score",
    header: "Score",
    cell: ({ row }) => (
      <Badge variant="secondary">{(row.original.score * 100).toFixed(1)}%</Badge>
    ),
  },
];

export function Search() {
  const [query, setQuery] = React.useState("");
  const { data = [], isFetching, refetch } = useQuery({
    queryKey: ["search", query],
    queryFn: () => searchDocuments(query),
    enabled: false,
  });

  const table = useReactTable({
    data,
    columns,
    getCoreRowModel: getCoreRowModel(),
    getPaginationRowModel: getPaginationRowModel(),
  });

  const handleSearch = (e: React.FormEvent) => {
    e.preventDefault();
    refetch();
  };

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-3xl font-bold tracking-tight">AI Search</h1>
        <p className="text-muted-foreground">Semantic search across your document collection</p>
      </div>

      {/* Search Bar */}
      <form onSubmit={handleSearch} className="flex gap-2">
        <div className="relative flex-1 max-w-xl">
          <SearchIcon className="absolute left-3 top-2.5 h-4 w-4 text-muted-foreground" />
          <Input
            placeholder="Search documents..."
            className="pl-9"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
        </div>
        <Button type="submit" disabled={isFetching || !query}>
          {isFetching ? "Searching..." : "Search"}
        </Button>
      </form>

      {/* Results Table */}
      <div className="rounded-md border">
        <Table>
          <TableHeader>
            {table.getHeaderGroups().map((hg) => (
              <TableRow key={hg.id}>
                {hg.headers.map((header) => (
                  <TableHead key={header.id}>
                    {flexRender(header.column.columnDef.header, header.getContext())}
                  </TableHead>
                ))}
              </TableRow>
            ))}
          </TableHeader>
          <TableBody>
            {table.getRowModel().rows.length ? (
              table.getRowModel().rows.map((row) => (
                <TableRow key={row.id} className="hover:bg-muted/40 cursor-pointer">
                  {row.getVisibleCells().map((cell) => (
                    <TableCell key={cell.id}>
                      {flexRender(cell.column.columnDef.cell, cell.getContext())}
                    </TableCell>
                  ))}
                </TableRow>
              ))
            ) : (
              <TableRow>
                <TableCell colSpan={columns.length} className="h-24 text-center">
                  {isFetching ? "Searching..." : "No results"}
                </TableCell>
              </TableRow>
            )}
          </TableBody>
        </Table>
      </div>

      {/* Pagination */}
      {table.getPageCount() > 1 && (
        <div className="flex items-center justify-end gap-2">
          <Button
            variant="outline"
            size="icon"
            onClick={() => table.previousPage()}
            disabled={!table.getCanPreviousPage()}
          >
            <ChevronDown className="h-4 w-4 rotate-90" />
          </Button>
          <span className="text-sm">
            Page {table.getState().pagination.pageIndex + 1} of {table.getPageCount()}
          </span>
          <Button
            variant="outline"
            size="icon"
            onClick={() => table.nextPage()}
            disabled={!table.getCanNextPage()}
          >
            <ChevronDown className="h-4 w-4 -rotate-90" />
          </Button>
        </div>
      )}
    </div>
  );
} 