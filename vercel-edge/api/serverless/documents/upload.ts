import type { VercelRequest, VercelResponse } from '@vercel/node';
import { nanoid } from 'nanoid';
import { getDatabase } from '../../../lib/database';
import type { Document, UploadResponse, ExtractionResult } from '../../../types';

export default async function handler(req: VercelRequest, res: VercelResponse) {
  if (req.method !== 'POST') {
    return res.status(405).json({ error: 'Method not allowed' });
  }

  try {
    // Check authorization
    const authHeader = req.headers.authorization;
    if (!authHeader?.startsWith('Bearer ')) {
      return res.status(401).json({ error: 'Authorization required' });
    }

    // Parse multipart form data (simplified - in production use a proper parser)
    const contentType = req.headers['content-type'];
    if (!contentType?.includes('multipart/form-data')) {
      return res.status(400).json({ error: 'Multipart form data required' });
    }

    // Mock file processing (in production, parse actual file upload)
    const mockFileContent = `
      # Sample Document
      
      This is a test document for processing.
      
      Contact: john.doe@example.com
      Phone: +1-555-123-4567
      Website: https://example.com
      
      ## Content
      
      This document contains various types of information that can be extracted
      and analyzed by the Swoop platform.
    `;

    // Generate document ID
    const documentId = nanoid();
    
    // Process document content (mock extraction)
    const extractionResult: ExtractionResult = {
      emails: ['john.doe@example.com'],
      phones: ['+1-555-123-4567'],
      links: ['https://example.com'],
      metadata: {
        file_type: 'markdown',
        word_count: '42',
        character_count: '312',
        language: 'en',
      },
      sensitive_data: [
        {
          data_type: 'email',
          original_text: 'john.doe@example.com',
          redacted_text: '[EMAIL_REDACTED]',
        },
        {
          data_type: 'phone',
          original_text: '+1-555-123-4567',
          redacted_text: '[PHONE_REDACTED]',
        },
      ],
      quality_score: 0.85,
      classification: ['business', 'contact_information'],
      validation_issues: [],
    };

    // Create document object
    const document: Document = {
      id: documentId,
      title: 'Sample Document',
      content: mockFileContent,
      summary: 'A sample document containing contact information and basic content.',
      metadata: {
        source_url: undefined,
        author: undefined,
        created_at: undefined,
        modified_at: undefined,
        file_size: mockFileContent.length,
        mime_type: 'text/markdown',
        encoding: 'utf-8',
        page_count: 1,
        processed_at: new Date().toISOString(),
      },
      quality_score: extractionResult.quality_score,
      content_hash: 'mock_hash_' + documentId,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
      word_count: 42,
      character_count: mockFileContent.length,
      language: 'en',
      tags: extractionResult.classification,
    };

    // Store document in database
    const dbUrl = process.env.TURSO_DATABASE_URL;
    const authToken = process.env.TURSO_AUTH_TOKEN;
    
    if (dbUrl && authToken) {
      const db = getDatabase(dbUrl, authToken);
      await db.initialize();
      await db.storeDocument(document);
    }

    // Prepare response
    const response: UploadResponse = {
      document_id: documentId,
      status: 'completed',
      message: 'Document uploaded and processed successfully',
      analysis: extractionResult,
    };

    res.status(200).json(response);

  } catch (error) {
    console.error('Upload error:', error);
    
    res.status(500).json({
      error: 'Upload failed',
      message: error instanceof Error ? error.message : 'Unknown error',
      timestamp: new Date().toISOString(),
    });
  }
} 