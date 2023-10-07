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

export default function Home() {
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
        <Button variant="outlined">人机验证</Button>
        <Button variant="contained">确定</Button>
      </Stack>
    </Box>
  );
}
