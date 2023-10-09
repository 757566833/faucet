import { enqueueSnackbar } from "notistack";
export const server = process.env.NEXT_PUBLIC_RESULTFUl || "";

const allHeaders: HeadersInit = {
  "Content-Type": "application/json",
};
export class Http {
  public static get = async <T>(url: string, params?: Record<string, any>) => {
    let fetchUrl;
    if (params) {
      const urlParams = new URLSearchParams();
      for (const key in params) {
        if (Object.prototype.hasOwnProperty.call(params, key)) {
          const element = params[key];
          urlParams.set(key, `${element}`);
        }
      }
      fetchUrl = `${server}${url}?${urlParams}`;
    } else {
      fetchUrl = `${server}${url}`;
    }
    try {
      const response = await fetch(fetchUrl, { headers: allHeaders });
      if (response.status >= 300) {
        enqueueSnackbar({ message: "服务异常", variant: "error" });
      } else {
        try {
          return (await response.json()) as T;
        } catch (error) {
          enqueueSnackbar({ message: "无法序列化", variant: "error" });
        }
      }
    } catch (error) {
      enqueueSnackbar({ message: "服务异常", variant: "error" });
    }
  };
  public static post = async <T>(url: string, params?: Record<string, any>) => {
    const fetchUrl = `${server}${url}`;

    try {
      const response = await fetch(fetchUrl, {
        method: "POST",
        body: JSON.stringify(params),
        headers: allHeaders,
      });
      if (response.status >= 300) {
        enqueueSnackbar({ message: "服务异常", variant: "error" });
      } else {
        try {
          return (await response.json()) as T;
        } catch (error) {
          enqueueSnackbar({ message: "无法序列化", variant: "error" });
        }
      }
    } catch (error) {
      enqueueSnackbar({ message: "服务异常", variant: "error" });
    }
  };
}
