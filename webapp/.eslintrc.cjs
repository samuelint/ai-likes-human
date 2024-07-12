module.exports = {
  root: true,
  env: { browser: true, es2020: true },
  extends: ['eslint:recommended', 'prettier', 'plugin:@typescript-eslint/recommended', 'plugin:react-hooks/recommended', 'plugin:storybook/recommended'],
  ignorePatterns: ['dist', '.eslintrc.cjs'],
  parser: '@typescript-eslint/parser',
  plugins: ['react-refresh', '@typescript-eslint', 'unicorn'],
  rules: {
    'no-multi-spaces': 'error',
    'no-trailing-spaces': 'error',
    'indent': ['error', 2],
    'quotes': ['error', 'single'],
    '@typescript-eslint/explicit-member-accessibility': 'error',
    'semi': 'off',
    '@typescript-eslint/semi': ['error'],
    'no-unexpected-multiline': 'error',
    'object-curly-spacing': ['error', 'always'],
    'keyword-spacing': ['error', { 'before': true, 'after': true }],
    '@typescript-eslint/no-unused-vars': [
      'error',
      {
        args: 'after-used',
        caughtErrors: 'none',
        ignoreRestSiblings: true,
        vars: 'all',
        argsIgnorePattern: '^_',
        varsIgnorePattern: '^_',
        caughtErrorsIgnorePattern: '^_'
      }
    ],
    'prefer-const': 'error',
    'react-hooks/exhaustive-deps': 'error',
    'unicorn/filename-case': [
      'error',
      {
        case: 'kebabCase'
      }
    ],
    'react-refresh/only-export-components': [
      'warn',
      { allowConstantExport: true },
    ],
  },
}
