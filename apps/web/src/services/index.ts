import { Http } from "@/http";

export const server = process.env.NEXT_PUBLIC_RESULTFUl || "";

export const getRoot = () => {
  return Http.get<{ code: string }>("/root/code");
};
export const getVerificationCode = (params: {
  hash: string;
  root: string;
  nonce: string;
  email: string;
}) => {
  return Http.post<{ code: string }>("/verification/code", params);
};

export const faucet = (params: {
  address: string;
  email: string;
  code: string;
}) => {
  return Http.post<{ code: string }>("/faucet", params);
};
