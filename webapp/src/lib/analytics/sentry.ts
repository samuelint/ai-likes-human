import appConfig from '@/app.config';
import * as Sentry from '@sentry/react';

export function initSentry() {
  if (appConfig.sentry_dsn) {
    Sentry.init({
      dsn: appConfig.sentry_dsn,
      environment: process.env.NODE_ENV === 'development' ? 'development' : 'production',
      integrations: [Sentry.feedbackSyncIntegration({
        colorScheme: 'system',
        autoInject: false,
        formTitle: 'Give Feedback',
        submitButtonLabel: 'Send Feedback',
        messagePlaceholder: 'How can we make this better?\nDid you have a bug?',
      }),],
    });
  }
}