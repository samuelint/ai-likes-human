# !!! Before transfering this to APP router check this issue first !!!

**🤬🤬 App Router does not handle dynamic routes when exported to a Single Page APP (SPA)**

Main issue | https://github.com/vercel/next.js/discussions/55393
Related issue | https://github.com/vercel/next.js/issues/56253

## Solution until the problem is fixed

- Every dynamic routes are implemented using the legacy pages router.

## Caveats

- In dev mode (`pnpm run dev`), there's a slight delay when navigating from a `page route` to `app route` (or vice versa). This is probably due to the fact that the whole page is reloaded. BUT **This is not happening when the app is built (pnpm run build).**
