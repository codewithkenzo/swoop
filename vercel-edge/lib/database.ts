import { createClient } from '@libsql/client';
import type { Client } from '@libsql/client';
import type { Document, DocumentBatch, DocumentMetadata } from '../types';

export class EdgeDatabase {
  private client: Client;

  constructor(url: string, authToken: string) {
    this.client = createClient({
      url,
      authToken,
    });
  }

  async initialize(): Promise<void> {
    // Create tables if they don't exist
    await this.client.execute(`
      CREATE TABLE IF NOT EXISTS documents (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        content TEXT NOT NULL,
        summary TEXT,
        metadata TEXT NOT NULL,
        quality_score REAL NOT NULL DEFAULT 0.0,
        content_hash TEXT NOT NULL,
        created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
        word_count INTEGER NOT NULL DEFAULT 0,
        character_count INTEGER NOT NULL DEFAULT 0,
        language TEXT,
        tags TEXT NOT NULL DEFAULT '[]'
      )
    `);

    await this.client.execute(`
      CREATE TABLE IF NOT EXISTS document_batches (
        id TEXT PRIMARY KEY,
        document_ids TEXT NOT NULL,
        total_documents INTEGER NOT NULL,
        status TEXT NOT NULL CHECK (status IN ('pending', 'processing', 'completed', 'failed')),
        created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
      )
    `);

    await this.client.execute(`
      CREATE INDEX IF NOT EXISTS idx_documents_created_at ON documents(created_at);
    `);

    await this.client.execute(`
      CREATE INDEX IF NOT EXISTS idx_documents_content_hash ON documents(content_hash);
    `);
  }

  async storeDocument(document: Document): Promise<void> {
    await this.client.execute({
      sql: `
        INSERT OR REPLACE INTO documents 
        (id, title, content, summary, metadata, quality_score, content_hash, 
         created_at, updated_at, word_count, character_count, language, tags)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
      `,
      args: [
        document.id,
        document.title,
        document.content,
        document.summary || null,
        JSON.stringify(document.metadata),
        document.quality_score,
        document.content_hash,
        document.created_at,
        document.updated_at,
        document.word_count,
        document.character_count,
        document.language || null,
        JSON.stringify(document.tags),
      ],
    });
  }

  async retrieveDocument(id: string): Promise<Document | null> {
    const result = await this.client.execute({
      sql: 'SELECT * FROM documents WHERE id = ?',
      args: [id],
    });

    if (result.rows.length === 0) {
      return null;
    }

    const row = result.rows[0]!;
    const summary = (row.summary as string | null) ?? "";
    return {
      id: row.id as string,
      title: row.title as string,
      content: row.content as string,
      summary,
      metadata: JSON.parse(row.metadata as string) as DocumentMetadata,
      quality_score: row.quality_score as number,
      content_hash: row.content_hash as string,
      created_at: row.created_at as string,
      updated_at: row.updated_at as string,
      word_count: row.word_count as number,
      character_count: row.character_count as number,
      language: (row.language as string | null) ?? "",
      tags: JSON.parse(row.tags as string) as string[],
    };
  }

  async listDocuments(limit = 50, offset = 0): Promise<Document[]> {
    const result = await this.client.execute({
      sql: 'SELECT * FROM documents ORDER BY created_at DESC LIMIT ? OFFSET ?',
      args: [limit, offset],
    });

    return result.rows.map((row): Document => {
      const summary = (row.summary as string | null) ?? "";
      return {
        id: row.id as string,
        title: row.title as string,
        content: row.content as string,
        summary,
        metadata: JSON.parse(row.metadata as string) as DocumentMetadata,
        quality_score: row.quality_score as number,
        content_hash: row.content_hash as string,
        created_at: row.created_at as string,
        updated_at: row.updated_at as string,
        word_count: row.word_count as number,
        character_count: row.character_count as number,
        language: (row.language as string | null) ?? "",
        tags: JSON.parse(row.tags as string) as string[],
      };
    });
  }

  async deleteDocument(id: string): Promise<boolean> {
    const result = await this.client.execute({
      sql: 'DELETE FROM documents WHERE id = ?',
      args: [id],
    });

    return result.rowsAffected > 0;
  }

  async searchDocuments(query: string, limit = 20): Promise<Document[]> {
    const result = await this.client.execute({
      sql: `
        SELECT * FROM documents 
        WHERE title LIKE ? OR content LIKE ?
        ORDER BY created_at DESC 
        LIMIT ?
      `,
      args: [`%${query}%`, `%${query}%`, limit],
    });

    return result.rows.map((row): Document => {
      const summary = (row.summary as string | null) ?? "";
      return {
        id: row.id as string,
        title: row.title as string,
        content: row.content as string,
        summary,
        metadata: JSON.parse(row.metadata as string) as DocumentMetadata,
        quality_score: row.quality_score as number,
        content_hash: row.content_hash as string,
        created_at: row.created_at as string,
        updated_at: row.updated_at as string,
        word_count: row.word_count as number,
        character_count: row.character_count as number,
        language: (row.language as string | null) ?? "",
        tags: JSON.parse(row.tags as string) as string[],
      };
    });
  }

  async getDocumentCount(): Promise<number> {
    const result = await this.client.execute('SELECT COUNT(*) as count FROM documents');
    return (result.rows[0]! .count as number) ?? 0;
  }

  async healthCheck(): Promise<boolean> {
    try {
      await this.client.execute('SELECT 1');
      return true;
    } catch {
      return false;
    }
  }

  async close(): Promise<void> {
    this.client.close();
  }
}

// Singleton instance for edge runtime
let dbInstance: EdgeDatabase | null = null;

export function getDatabase(url?: string, authToken?: string): EdgeDatabase {
  if (!dbInstance) {
    if (!url || !authToken) {
      throw new Error('Database URL and auth token are required for first initialization');
    }
    dbInstance = new EdgeDatabase(url, authToken);
  }
  return dbInstance;
} 