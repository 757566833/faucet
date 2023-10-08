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
import { useCallback } from "react";
async function digestMessage(message: string) {
  const msgUint8 = new TextEncoder().encode(message); // encode as (utf-8) Uint8Array
  const hashBuffer = await crypto.subtle.digest("SHA-256", msgUint8); // hash the message
  const hashArray = Array.from(new Uint8Array(hashBuffer)); // convert buffer to byte array
  const hashHex = hashArray
    .map((b) => b.toString(16).padStart(2, "0"))
    .join(""); // convert bytes to hex string
  return hashHex;
}
export default function Home() {
  const sha256 = useCallback(async () => {
    let result = "";
    let startTime = new Date().getTime();
    let nonce = 0;
    do {
      result = await digestMessage(nonce.toString());
      nonce++;
    } while (!result.startsWith("0000"));
    let endTime = new Date().getTime();
    console.log("time", endTime - startTime);
    console.log(result);
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
                <LoadingButton>发送验证码</LoadingButton>
              </InputAdornment>
            }
            label="邮箱"
          />
        </FormControl>
        <FormControl variant="outlined">
          <InputLabel>地址</InputLabel>
          <OutlinedInput label="地址" />
        </FormControl>
        <Button variant="outlined" onClick={sha256}>
          人机验证
        </Button>
        <Button variant="contained">确定</Button>
      </Stack>
    </Box>
  );
}
