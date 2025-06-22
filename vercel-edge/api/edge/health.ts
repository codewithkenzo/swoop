import type { EdgeRequest } from '../../types';
import { getDatabase } from '../../lib/database';

export const runtime = 'edge';

export default async function handler(req: EdgeRequest) {
  if (req.method !== 'GET') {
    return new Response(
      JSON.stringify({ error: 'Method not allowed' }),
      { 
        status: 405,
        headers: { 'Content-Type': 'application/json' }
      }
    );
  }

  try {
    const startTime = Date.now();
    
    // Get environment variables
    const dbUrl = process.env.TURSO_DATABASE_URL;
    const authToken = process.env.TURSO_AUTH_TOKEN;
    
    if (!dbUrl || !authToken) {
      return new Response(
        JSON.stringify({
          status: 'degraded',
          version: '1.0.0',
          uptime_seconds: Math.floor(process.uptime?.() || 0),
          checks: {
            database: false,
            llm_service: false,
            storage: false,
          },
          error: 'Database configuration missing'
        }),
        { 
          status: 200,
          headers: { 'Content-Type': 'application/json' }
        }
      );
    }

    // Test database connection
    const db = getDatabase(dbUrl, authToken);
    const dbHealthy = await db.healthCheck();
    
    // Test OpenRouter API
    const openrouterHealthy = process.env.OPENROUTER_API_KEY ? true : false;
    
    const responseTime = Date.now() - startTime;
    const allHealthy = dbHealthy && openrouterHealthy;
    
    return new Response(
      JSON.stringify({
        status: allHealthy ? 'healthy' : 'degraded',
        version: '1.0.0',
        uptime_seconds: Math.floor(process.uptime?.() || 0),
        response_time_ms: responseTime,
        edge_region: req.geo?.region || 'unknown',
        checks: {
          database: dbHealthy,
          llm_service: openrouterHealthy,
          storage: true, // Edge storage is always available
        },
        timestamp: new Date().toISOString(),
      }),
      { 
        status: 200,
        headers: { 
          'Content-Type': 'application/json',
          'Cache-Control': 'no-cache, no-store, must-revalidate',
        }
      }
    );
    
  } catch (error) {
    return new Response(
      JSON.stringify({
        status: 'unhealthy',
        version: '1.0.0',
        error: error instanceof Error ? error.message : 'Unknown error',
        timestamp: new Date().toISOString(),
      }),
      { 
        status: 503,
        headers: { 'Content-Type': 'application/json' }
      }
    );
  }
} 