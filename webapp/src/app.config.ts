const api_url = 'http://localhost:1234';
const openai_api_url = `${api_url}/openai/v1`;

const AVAILABLE_LLM_MODELS: readonly [string, ...string[]] = ['openai:gpt-4o', 'openai:gpt-4o-mini', 'anthropic:claude-3-5-sonnet-20240620', 'anthropic:claude-3-opus-20240229'];
const LLM_API_KEYS_KEYS: readonly [string, ...string[]] = ['OPENAI_API_KEY', 'ANTHROPIC_API_KEY'];

const appConfig = {
  app_title: 'AI Likes Human',
  app_description: 'The ultimate AI client',
  api_url,
  openai_api_url,
  available_llm_models: AVAILABLE_LLM_MODELS,
  sentry_dsn: process.env.SENTRY_DSN ?? 'https://9245f149e4e89cfcce3c336ce6df072d@o4508059383824384.ingest.us.sentry.io/4508059486060544',
};

export { api_url, openai_api_url, LLM_API_KEYS_KEYS };
export default appConfig;
