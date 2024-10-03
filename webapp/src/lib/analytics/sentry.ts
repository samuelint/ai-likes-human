import appConfig from '@/app.config';
import * as Sentry from '@sentry/react';

export function initSentry() {
  if (appConfig.sentry_dsn) {
    Sentry.init({
      dsn: appConfig.sentry_dsn,
      integrations: [],
    });
  }
}