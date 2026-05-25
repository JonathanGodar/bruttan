import { defineConfig } from 'orval';

export default defineConfig({
  bruttan: {
    output: {
      mode: 'tags-split',
      target: 'src/api/bruttan.ts',
      schemas: 'src/api/model',
      client: 'react-query',
      mock: true,
      override: {
        mutator: {
          path: 'src/api/mutator/custom-fetch.ts',
          name: 'customFetch',
        },
      },
    },
    input: {
      target: '../api.yaml',
    },
  },
});
