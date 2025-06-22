import { z } from 'zod';
import type { EdgeRequest } from '../../../types';

export const runtime = 'edge';

const AuthSchema = z.object({
  api_key: z.string().min(1),
});

// Simple API key validation (in production, use proper JWT or database lookup)
const VALID_API_KEYS = new Set([
  process.env.SWOOP_API_KEY,
  // Add more API keys as needed
].filter(Boolean));

export default async function handler(req: EdgeRequest) {
  if (req.method !== 'POST') {
    return new Response(
      JSON.stringify({ error: 'Method not allowed' }),
      { 
        status: 405,
        headers: { 'Content-Type': 'application/json' }
      }
    );
  }

  try {
    const body = await req.json();
    const { api_key } = AuthSchema.parse(body);

    // Check API key validity
    const isValid = VALID_API_KEYS.has(api_key);
    
    if (!isValid) {
      return new Response(
        JSON.stringify({ 
          valid: false, 
          error: 'Invalid API key',
          timestamp: new Date().toISOString(),
        }),
        { 
          status: 401,
          headers: { 'Content-Type': 'application/json' }
        }
      );
    }

    // Mock user data (in production, fetch from database)
    const userData = {
      id: 'user_' + api_key.slice(-8),
      email: 'user@example.com',
      tier: 'premium' as const,
      usage: {
        requests_today: 245,
        requests_limit: 10000,
        cost_today: 2.45,
      },
    };

    return new Response(
      JSON.stringify({
        valid: true,
        user: userData,
        edge_region: req.geo?.region || 'unknown',
        timestamp: new Date().toISOString(),
      }),
      { 
        status: 200,
        headers: { 
          'Content-Type': 'application/json',
          'Cache-Control': 'private, max-age=300', // Cache for 5 minutes
        }
      }
    );

  } catch (error) {
    if (error instanceof z.ZodError) {
      return new Response(
        JSON.stringify({ 
          valid: false,
          error: 'Invalid request format',
          details: error.errors,
        }),
        { 
          status: 400,
          headers: { 'Content-Type': 'application/json' }
        }
      );
    }

    return new Response(
      JSON.stringify({
        valid: false,
        error: 'Authentication service error',
        timestamp: new Date().toISOString(),
      }),
      { 
        status: 500,
        headers: { 'Content-Type': 'application/json' }
      }
    );
  }
} 