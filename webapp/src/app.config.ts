import { LLMVendorConfiguration } from './lib/llm';

const api_url = 'http://localhost:1234';
const openai_api_url = `${api_url}/openai/v1`;

const AVAILABLE_LLM_MODELS: readonly [string, ...string[]] = ['openai:gpt-4o', 'openai:gpt-4o-mini', 'anthropic:claude-3-5-sonnet-20240620', 'anthropic:claude-3-opus-20240229'];

const LLM_VENDORS_CONFIGURATIONS: readonly [LLMVendorConfiguration, ...LLMVendorConfiguration[]] = [
  {
    name: 'OpenAI',
    api_key_key: 'OPENAI_API_KEY',
    how_to_get_api_key_url: 'https://platform.openai.com/api-keys'
  },
  {
    name: 'Anthropic',
    api_key_key: 'ANTHROPIC_API_KEY',
    how_to_get_api_key_url: 'https://console.anthropic.com/settings/keys',
  }
];

const appConfig = {
  app_title: 'AI Likes Human',
  app_description: 'The ultimate AI client',
  api_url,
  openai_api_url,
  available_llm_models: AVAILABLE_LLM_MODELS,
  sentry_dsn: 'https://bd55cf78661dc0dab72fc80983efadde@o4508059383824384.ingest.us.sentry.io/4508059490189312',
};

export { api_url, openai_api_url, LLM_VENDORS_CONFIGURATIONS };
export default appConfig;
