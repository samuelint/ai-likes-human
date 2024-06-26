module.exports = {
  extends: ['next', 'prettier', 'plugin:@typescript-eslint/recommended', 'plugin:storybook/recommended'],
  plugins: ['@typescript-eslint', 'unicorn'],
  parser: '@typescript-eslint/parser',
  rules: {
    'no-multi-spaces': 'error',
    'no-trailing-spaces': 'error',
    'indent': ['error', 2],
    'quotes': ['error', 'single'],
    'import/newline-after-import': ['error', { 'count': 2 }],
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
    ]
  }
};
