export const API_HOST = process.env.NEXT_PUBLIC_API_HOST || "127.0.0.1";
export const API_PORT = process.env.NEXT_PUBLIC_API_PORT || "8080";
export const PROTOCOL = process.env.NODE_ENV === "development" ? "http" : "https";
export const API_URL = process.env.NEXT_PUBLIC_API_URL || `${PROTOCOL}://${API_HOST}:${API_PORT}`;
