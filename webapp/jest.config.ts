import type { Config } from 'jest';
import nextJest from 'next/jest.js';


const createJestConfig = nextJest({
  dir: './',
});

const config: Config = {
  preset: 'ts-jest',
  coverageProvider: 'v8',
  testEnvironment: 'jsdom',
  setupFilesAfterEnv: ['<rootDir>/jest.setup.ts', 'jest-extended/all'],
  rootDir: '.',
  moduleNameMapper: {
    '@/(.*)': '<rootDir>/$1'
  }
};

export default createJestConfig(config);