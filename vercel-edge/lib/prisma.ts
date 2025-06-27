import { PrismaClient } from '@prisma/client'

// In Edge runtime we want to reuse the Prisma instance between requests.
const globalForPrisma = globalThis as unknown as { prisma?: PrismaClient }

export const prisma =
  globalForPrisma.prisma ||
  new PrismaClient({
    // Prefer the Data Proxy for connection pooling / edge compatibility.
    datasources: {
      db: {
        url: process.env.DATABASE_URL,
      },
    },
  })

if (process.env.NODE_ENV !== 'production') globalForPrisma.prisma = prisma 