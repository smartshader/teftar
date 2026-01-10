import { type RouteConfig, index, route } from "@react-router/dev/routes";

export default [
  index("routes/home.tsx"),
  route("signup", "routes/signup.tsx"),
  route("signin", "routes/signin.tsx"),
  route("confirm-email", "routes/confirm-email.tsx"),
  route("auth/callback", "routes/auth-callback.tsx"),
  route("*", "routes/$.tsx"),
] satisfies RouteConfig;
