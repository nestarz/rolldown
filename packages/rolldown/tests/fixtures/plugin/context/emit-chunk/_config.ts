// cSpell:disable
import { defineTest } from '@tests'
import { getOutputFileNames } from '@tests/utils'
import { expect } from 'vitest'

export default defineTest({
  skipComposingJsPlugin: true,
  config: {
    output: {
      entryFileNames: '[name].js',
      chunkFileNames: '[name]-[hash].js',
    },
    plugins: [
      {
        name: 'test-plugin-context',
        load() {
          this.emitFile({
            type: 'chunk',
            name: 'main-with-name',
            id: './main.js',
          })
          this.emitFile({
            type: 'chunk',
            fileName: 'main-with-fileName.js',
            id: './main.js',
          })
        },
      },
    ],
  },
  afterTest: (output) => {
    expect(getOutputFileNames(output)).toMatchInlineSnapshot(
      `
      [
        "main-Fv4vYntb.js",
        "main-with-fileName.js",
        "main-with-name.js",
        "main.js",
      ]
    `,
    )
  },
})
