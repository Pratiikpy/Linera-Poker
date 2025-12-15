// Worker environment polyfills for document and window
// Prevents "document is not defined" errors in WASM workers

if (typeof document === 'undefined') {
  globalThis.document = {
    getElementById: () => null,
    createElement: () => ({}),
    body: {},
    head: {},
    addEventListener: () => {},
    removeEventListener: () => {},
    querySelector: () => null,
    querySelectorAll: () => [],
  };
}

if (typeof window === 'undefined') {
  globalThis.window = globalThis;
}

if (typeof navigator === 'undefined') {
  globalThis.navigator = {
    userAgent: 'Worker',
  };
}
