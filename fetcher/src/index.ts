export interface Request {
  /** @example "https://api.github.com/" */
  url: string

  /** @example "GET" */
  method: string

  body?: string | Uint8Array

  /**
   * Headers are represented as a list of key-value pairs
   * where the key is the header name (not always lowercase) and the value is the header value.
   * @example [["content-type", "application/json"]]
   */
  headers: Array<[key: string, value: string]>

  /**
   * Whether we should follow redirects or not.
   */
  follow?: boolean
}

export interface Response {
  status: number
  /**
   * Headers are represented as a list of key-value pairs
   * where the key is the header name (always lowercase) and the value is the header value.
   * @example [["content-type", "application/json"]]
   */
  headers: Array<[key: string, value: string]>
  bytes: Uint8Array | ArrayBuffer
}

export type Fetcher = (req: Request) => Promise<Response>;

const isHeaders = (headers: any): headers is Headers => {
  return typeof headers.get === "function";
}

/**
 * Simple and default fetcher using `fetch` if none was given
 * in the authentication function.
 *
 * Should work out-of-the-box on Node.js>=18, Deno, Bun, React Native
 * and probably more environments.
 */
export const defaultFetcher: Fetcher = async (req) => {
  let headers: Array<[key: string, value: string]> | Record<string, string>;

  headers = {}; // initialize for the request
  for (const [key, value] of req.headers) {
    headers[key] = value;
  }

  const response = await fetch(req.url, {
    redirect: req.follow === false ? "manual" : "follow",
    method: req.method,
    body: req.body,
    headers,

    // We don't want to send saved cookies,
    // only the ones we set manually.
    credentials: "omit"
  });

  const status = response.status; // should be `u16`
  const bytes = await response.arrayBuffer();

  // Since `Headers.entries()` is not available in all environments
  // we have to do this dirty hack to get all headers...
  headers = (isHeaders(response.headers)
    // Available in this environment
    // so we can use it to get all headers.
    ? Array.from(response.headers.entries())
    // Not currently used, so we assume
    // that headers is a plain object.
    : Object.entries(response.headers as Record<string, string>)
  ).map(([key, value]) => [key.toLowerCase(), value] as [key: string, value: string]);

  return {
    status,
    headers,
    bytes
  };
};

export class FetcherError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "FetcherError";
  }
}
