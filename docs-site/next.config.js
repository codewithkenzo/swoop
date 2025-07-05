const { createMDX } = require('fumadocs-mdx/next');

const withMDX = createMDX();

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  swcMinify: true,
  pageExtensions: ['js', 'jsx', 'ts', 'tsx', 'md', 'mdx'],
  experimental: {
    mdxRs: true,
  },
  async rewrites() {
    return [
      {
        source: '/api/proxy/:path*',
        destination: process.env.SWOOP_API_URL + '/:path*',
      },
    ];
  },
  env: {
    SWOOP_API_URL: process.env.SWOOP_API_URL || 'http://localhost:8080',
    SWOOP_DEMO_MODE: process.env.SWOOP_DEMO_MODE || 'false',
  },
};

module.exports = withMDX(nextConfig);