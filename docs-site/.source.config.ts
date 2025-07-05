import { defineConfig, defineDocs } from 'fumadocs-mdx/config';

export default defineConfig({
  generateTypeTable: true,
});

export const { docs, meta } = defineDocs({
  dir: 'src/content',
});