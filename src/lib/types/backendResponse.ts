export interface BackendResponse<T> {
  status: "success" | "error";
  message: string;
  data?: T;
}
