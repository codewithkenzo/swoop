import type { VercelRequest, VercelResponse } from '@vercel/node';
import { z } from 'zod';
import type { ChatRequest, ChatResponse } from '../../../types';

const ChatRequestSchema = z.object({
  messages: z.array(z.object({
    role: z.enum(['user', 'assistant', 'system']),
    content: z.string(),
  })),
  document_ids: z.array(z.string()).optional(),
  model: z.string().optional().default('openai/gpt-4o-mini'),
  stream: z.boolean().optional().default(false),
  temperature: z.number().min(0).max(2).optional().default(0.7),
  max_tokens: z.number().positive().optional().default(1000),
});

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

    // Validate request body
    const chatRequest = ChatRequestSchema.parse(req.body);
    
    // Check if OpenRouter API key is available
    const openrouterApiKey = process.env.OPENROUTER_API_KEY;
    if (!openrouterApiKey) {
      return res.status(500).json({ 
        error: 'OpenRouter API key not configured' 
      });
    }

    // Prepare system message with document context
    let systemMessage = 'You are Swoop AI, an intelligent document processing assistant.';
    
    if (chatRequest.document_ids && chatRequest.document_ids.length > 0) {
      // In production, fetch actual document content from database
      systemMessage += ` You have access to ${chatRequest.document_ids.length} document(s) for context.`;
    }

    // Prepare messages for OpenRouter
    const messages = [
      { role: 'system' as const, content: systemMessage },
      ...chatRequest.messages,
    ];

    // Call OpenRouter API
    const openrouterResponse = await fetch('https://openrouter.ai/api/v1/chat/completions', {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${openrouterApiKey}`,
        'Content-Type': 'application/json',
        'HTTP-Referer': 'https://swoop.ai',
        'X-Title': 'Swoop AI Document Intelligence',
      },
      body: JSON.stringify({
        model: chatRequest.model,
        messages,
        temperature: chatRequest.temperature,
        max_tokens: chatRequest.max_tokens,
        stream: chatRequest.stream,
      }),
    });

    if (!openrouterResponse.ok) {
      const errorData = await openrouterResponse.text();
      return res.status(openrouterResponse.status).json({
        error: 'OpenRouter API error',
        details: errorData,
      });
    }

    // Handle streaming response
    if (chatRequest.stream) {
      res.setHeader('Content-Type', 'text/event-stream');
      res.setHeader('Cache-Control', 'no-cache');
      res.setHeader('Connection', 'keep-alive');
      
      const reader = openrouterResponse.body?.getReader();
      if (!reader) {
        return res.status(500).json({ error: 'Failed to read stream' });
      }

      try {
        while (true) {
          const { done, value } = await reader.read();
          if (done) break;
          
          const chunk = new TextDecoder().decode(value);
          res.write(chunk);
        }
      } finally {
        reader.releaseLock();
        res.end();
      }
      return;
    }

    // Handle regular response
    const responseData = await openrouterResponse.json();
    
    // Transform to our ChatResponse format
    const chatResponse: ChatResponse = {
      id: responseData.id || 'chat_' + Date.now(),
      choices: responseData.choices || [],
      usage: responseData.usage || {
        prompt_tokens: 0,
        completion_tokens: 0,
        total_tokens: 0,
      },
      model: responseData.model || chatRequest.model,
    };

    res.status(200).json(chatResponse);

  } catch (error) {
    console.error('Chat error:', error);
    
    if (error instanceof z.ZodError) {
      return res.status(400).json({
        error: 'Invalid request format',
        details: error.errors,
      });
    }

    res.status(500).json({
      error: 'Chat service error',
      message: error instanceof Error ? error.message : 'Unknown error',
      timestamp: new Date().toISOString(),
    });
  }
} 