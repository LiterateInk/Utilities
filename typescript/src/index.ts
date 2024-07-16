export interface Request {
  url: string
  method: string
  /** Body of the request of type given in the "Content-Type" header. */
  content: string | undefined
  /** Headers that should be appended to the request. */
  headers: Array<[string, string]>
  // TODO: add in rust redirect: "follow" | "manual"
}

export interface Response {
  status: number
  content: string
  headers: Array<[string, string]>
}

export type Fetcher = (req: Request) => Promise<Response>;

/**
 * Simple and default fetcher using `fetch` if none was given
 * in the authentication function.
 * 
 * Should work out-of-the-box on Node.js>=18, Deno, Bun, React Native
 * and probably more environments.
 */
export const defaultFetcher = async (req: Request): Promise<Response> => {
  const request_headers: Record<string, string> = {};
  req.headers.forEach(([key, value]) => {
    request_headers[key] = value;
  });

  const response = await fetch(req.url, {
    headers: request_headers,
    method: req.method,
    body: req.content
  });

  const response_headers: Array<[string, string]> = [];
  response.headers.forEach((value, key) => {
    response_headers.push([key, value]);
  });

  return {
    status: response.status,
    content: await response.text(),
    headers: response_headers
  };
}