import type { EdgeRequest } from '../../../types';
import { getDatabase } from '../../../lib/database';

export const runtime = 'edge';

export default async function handler(req: EdgeRequest) {
  const url = new URL(req.url);
  const documentId = url.pathname.split('/').pop();

  if (!documentId) {
    return new Response(
      JSON.stringify({ error: 'Document ID is required' }),
      { 
        status: 400,
        headers: { 'Content-Type': 'application/json' }
      }
    );
  }

  // Handle different HTTP methods
  switch (req.method) {
    case 'GET':
      return handleGet(req, documentId);
    case 'DELETE':
      return handleDelete(req, documentId);
    default:
      return new Response(
        JSON.stringify({ error: 'Method not allowed' }),
        { 
          status: 405,
          headers: { 'Content-Type': 'application/json' }
        }
      );
  }
}

async function handleGet(req: EdgeRequest, documentId: string) {
  try {
    const dbUrl = process.env.TURSO_DATABASE_URL;
    const authToken = process.env.TURSO_AUTH_TOKEN;
    
    if (!dbUrl || !authToken) {
      return new Response(
        JSON.stringify({ error: 'Database configuration missing' }),
        { 
          status: 500,
          headers: { 'Content-Type': 'application/json' }
        }
      );
    }

    const db = getDatabase(dbUrl, authToken);
    const document = await db.retrieveDocument(documentId);
    
    if (!document) {
      return new Response(
        JSON.stringify({ error: 'Document not found' }),
        { 
          status: 404,
          headers: { 'Content-Type': 'application/json' }
        }
      );
    }

    return new Response(
      JSON.stringify({
        document,
        retrieved_at: new Date().toISOString(),
        edge_region: req.geo?.region || 'unknown',
      }),
      { 
        status: 200,
        headers: { 
          'Content-Type': 'application/json',
          'Cache-Control': 'public, max-age=300', // Cache for 5 minutes
        }
      }
    );

  } catch (error) {
    return new Response(
      JSON.stringify({
        error: 'Failed to retrieve document',
        message: error instanceof Error ? error.message : 'Unknown error',
        timestamp: new Date().toISOString(),
      }),
      { 
        status: 500,
        headers: { 'Content-Type': 'application/json' }
      }
    );
  }
}

async function handleDelete(req: EdgeRequest, documentId: string) {
  try {
    // Check authorization header
    const authHeader = req.headers.get('Authorization');
    if (!authHeader?.startsWith('Bearer ')) {
      return new Response(
        JSON.stringify({ error: 'Authorization required' }),
        { 
          status: 401,
          headers: { 'Content-Type': 'application/json' }
        }
      );
    }

    const dbUrl = process.env.TURSO_DATABASE_URL;
    const authToken = process.env.TURSO_AUTH_TOKEN;
    
    if (!dbUrl || !authToken) {
      return new Response(
        JSON.stringify({ error: 'Database configuration missing' }),
        { 
          status: 500,
          headers: { 'Content-Type': 'application/json' }
        }
      );
    }

    const db = getDatabase(dbUrl, authToken);
    const deleted = await db.deleteDocument(documentId);
    
    if (!deleted) {
      return new Response(
        JSON.stringify({ error: 'Document not found or already deleted' }),
        { 
          status: 404,
          headers: { 'Content-Type': 'application/json' }
        }
      );
    }

    return new Response(
      JSON.stringify({
        success: true,
        message: 'Document deleted successfully',
        document_id: documentId,
        deleted_at: new Date().toISOString(),
      }),
      { 
        status: 200,
        headers: { 'Content-Type': 'application/json' }
      }
    );

  } catch (error) {
    return new Response(
      JSON.stringify({
        error: 'Failed to delete document',
        message: error instanceof Error ? error.message : 'Unknown error',
        timestamp: new Date().toISOString(),
      }),
      { 
        status: 500,
        headers: { 'Content-Type': 'application/json' }
      }
    );
  }
} 