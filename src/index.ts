import { splitCookiesString } from "set-cookie-parser";

interface HeadersLike {
  get (key: string): string | null
};

/**
 * Extracts cookies from the "set-cookie" header of a response.
 */
export const getCookiesFromResponse = (response: Response): string[] => {
  const setCookieHeader = getHeaderFromResponse(response, "set-cookie");
  if (setCookieHeader === null) return [];

  return splitCookiesString(setCookieHeader)
    .map((cookie) => cookie.split(";")[0]);
};

/**
 * Extracts a header from a response.
 */
export const getHeaderFromResponse = (response: Response, item: string): string | null => {
  const headers = response.headers;

  return isHeader(headers)
    ? headers.get(item)
    : (headers as Record<string, string>)[item];
};

const isHeader = (headers: Response["headers"]): headers is Headers => {
  return typeof headers.get === "function";
}

export const setHeaderToRequest = (request: Request, key: string, value: string): void => {
  if (!request.headers) request.headers = {};
  
  if (isHeader(request.headers)) {
    request.headers.set(key, value);
  }
  else {
    request.headers[key] = value;
  }
}

export interface Request {
  url: URL
  
  /**
   * @default "GET"
   */
  method?: "GET" | "POST"

  /**
   * Body of the request.
   * @default undefined
   */
  content?: string
  
  /**
   * Headers that should be appended to the request.
   * @default {}
   */
  headers?: Record<string, string> | Headers
  
  /**
   * @default "follow"
   */
  redirect?: "follow" | "manual"
}

export interface Response {
  status: number
  content: string
  headers: Record<string, string> | Headers | HeadersLike
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
  const response = await fetch(req.url.href, {
    redirect: req.redirect ?? "follow",
    headers: req.headers ?? {},
    method: req.method ?? "GET",
    body: req.content
  });

  return {
    status: response.status,
    content: await response.text(),
    headers: response.headers
  };
}
