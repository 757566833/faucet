"use client";
import {
  AppBar,
  Box,
  Button,
  FormControl,
  IconButton,
  InputAdornment,
  InputLabel,
  OutlinedInput,
  Stack,
  Toolbar,
  Typography,
} from "@mui/material";
import { Email, Language, Link, Menu, Web } from "@mui/icons-material";
import { LoadingButton } from "@mui/lab";
import { useCallback, useEffect, useRef, useState } from "react";
import { faucet, getRoot, getVerificationCode } from "@/services";
import { useForm } from "react-hook-form";
import { isAddress } from "ethers";
async function digestMessage(message: string) {
  const msgUint8 = new TextEncoder().encode(message); // encode as (utf-8) Uint8Array
  const hashBuffer = await crypto.subtle.digest("SHA-256", msgUint8); // hash the message
  const hashArray = Array.from(new Uint8Array(hashBuffer)); // convert buffer to byte array
  const hashHex = hashArray
    .map((b) => b.toString(16).padStart(2, "0"))
    .join(""); // convert bytes to hex string
  return hashHex;
}
type Forms = {
  email: string;
  address: string;
  code: string;
};
const verifyEmail = (email: string) =>
  /^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$/.test(email);
export default function Home() {
  const {
    register,
    handleSubmit,
    watch,
    formState: { errors },
    setError,
    getValues,
  } = useForm<Forms>();
  const workerRef = useRef<Worker>();
  const cacheRef = useRef("");
  const [loading, setLoading] = useState(false);
  const handleSendSms = useCallback(async () => {
    let bool = verifyEmail(getValues().email);
    if (!bool) {
      setError("email", { type: "required", message: "Email is required" });
      return;
    }
    setLoading(true);
    const root = await getRoot();
    if (root) {
      workerRef.current?.postMessage(root.code);
      cacheRef.current = root.code;
    } else {
      setLoading(false);
    }
  }, [getValues, setError]);
  useEffect(() => {
    workerRef.current = new Worker(
      new URL("@/worker/sha256.ts", import.meta.url),
    );
    workerRef.current.onmessage = async (event: MessageEvent<string>) => {
      const data: { hash: string; nonce: number } = JSON.parse(event.data);

      let email = getValues().email;
      let bool = verifyEmail(getValues().email);
      if (!bool) {
        setError("email", { type: "required", message: "Email is required" });
      } else {
        const result = await getVerificationCode({
          hash: data.hash,
          root: cacheRef.current,
          nonce: data.nonce.toString(),
          email: email,
        });
        if (result) {
          console.log(result);
        }
      }
      setLoading(false);
    };
    return () => workerRef.current?.terminate();
  }, [getValues, setError]);
  const [submitLoading, setSubmitLoading] = useState(false);
  const handleSub = useCallback(async (data: Forms) => {
    setSubmitLoading(true);
    const res = await faucet(data);
    if (res) {
      console.log(res);
    }
    setSubmitLoading(false);
  }, []);

  return (
    <Box sx={{ flexGrow: 1 }}>
      <AppBar position="static">
        <Toolbar>
          <IconButton
            size="large"
            edge="start"
            color="inherit"
            aria-label="menu"
            sx={{ mr: 2 }}
          >
            <Language />
          </IconButton>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            即将在fzcode获取一些测试币
          </Typography>
        </Toolbar>
      </AppBar>
      <Stack
        margin={"0 auto"}
        width={600}
        marginTop={6}
        minHeight={600}
        spacing={3}
      >
        <FormControl variant="outlined">
          <InputLabel>邮箱</InputLabel>
          <OutlinedInput
            endAdornment={
              <InputAdornment position="end">
                <LoadingButton onClick={handleSendSms} loading={loading}>
                  发送验证码
                </LoadingButton>
              </InputAdornment>
            }
            label="邮箱"
            {...register("email", { required: true, validate: verifyEmail })}
            error={!!errors.email}
          />
        </FormControl>
        <FormControl variant="outlined">
          <InputLabel>地址</InputLabel>
          <OutlinedInput
            label="地址"
            {...register("address", { required: true, validate: isAddress })}
            error={!!errors.address}
          />
        </FormControl>
        <FormControl variant="outlined">
          <InputLabel>验证码</InputLabel>
          <OutlinedInput
            label="验证码"
            {...register("code", { required: true })}
            error={!!errors.code}
          />
        </FormControl>
        <LoadingButton
          loading={submitLoading}
          variant="contained"
          onClick={handleSubmit(handleSub)}
        >
          确定
        </LoadingButton>
      </Stack>
    </Box>
  );
}
