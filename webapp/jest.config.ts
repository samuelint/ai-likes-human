import type { Config } from 'jest';
import nextJest from 'next/jest.js';


const createJestConfig = nextJest({
  // Provide the path to your Next.js app to load next.config.js and .env files in your test environment
  dir: './',
});

// Add any custom config to be passed to Jest
const config: Config = {
  preset: 'ts-jest',
  coverageProvider: 'v8',
  testEnvironment: 'jsdom',
  setupFilesAfterEnv: ['<rootDir>/jest.setup.ts', 'jest-extended/all'],

  // roots: ['<rootDir>'],
  // modulePaths: [compilerOptions.baseUrl],
  // moduleNameMapper: pathsToModuleNameMapper(compilerOptions.paths),
  rootDir: '.',
  moduleNameMapper: {
    '@/(.*)': '<rootDir>/$1'
  }
};

export default createJestConfig(config);