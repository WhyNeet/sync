import { Button, FormControl, FormHelperText, InputLabel, Link, OutlinedInput, Stack, Typography } from "@mui/material";
import { SubmitHandler, useForm } from "react-hook-form";
import { Link as RouterLink, useNavigate } from "react-router"
import { signIn, signUp } from "../lib/cases/auth";
import { useState } from "react";

interface Inputs {
  username: string,
  password: string,
  email: string,
  display_name: string
}

export function SignUp() {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<Inputs>();
  const [isLoading, setIsLoading] = useState(false);
  const navigate = useNavigate();


  const submit: SubmitHandler<Inputs> = data => {
    setIsLoading(true);
    data.display_name = data.username;
    signUp(data).then(success => {
      if (success) signIn(data).then(success => {
        if (success) navigate("/");
        else console.warn("todo: show error");
      });
      else console.warn("todo: show error");
    });
  };

  return <Stack justifyContent="center" alignItems="center" sx={{ height: "100%" }}>
    <Stack justifyContent="center" sx={{ width: "100%" }} maxWidth="500px">
      <Typography variant="h3" component="h1">Sign up</Typography>
      <Typography variant="subtitle1">Already have an account? <RouterLink to="/sign-in"><Link component="span">Sign in.</Link></RouterLink></Typography>
      <Stack component="form" autoComplete="off" onSubmit={handleSubmit(submit)} gap={1} mt={2}>
        <FormControl error={!!errors.email} required variant="outlined">
          <InputLabel htmlFor="email">Email</InputLabel>
          <OutlinedInput
            id="email"
            label="Email"
            aria-describedby="email-helper"
            {...register("email", { required: { value: true, message: "Required." } })}
          />
          {errors.email && <FormHelperText id="email-helper">{errors.email.message}</FormHelperText>}
        </FormControl>
        <FormControl error={!!errors.username} required variant="outlined">
          <InputLabel htmlFor="username">Username</InputLabel>
          <OutlinedInput
            id="username"
            label="Username"
            aria-describedby="username-helper"
            {...register("username", { required: { value: true, message: "Required." }, minLength: { value: 2, message: "Must be at least 2 characters long." }, maxLength: { value: 32, message: "Must be at most 32 characters long." } })}
          />
          {errors.username && <FormHelperText id="username-helper">{errors.username.message}</FormHelperText>}
        </FormControl>
        <FormControl error={!!errors.password} required variant="outlined">
          <InputLabel htmlFor="password">Password</InputLabel>
          <OutlinedInput
            id="password"
            type="password"
            label="Password"
            {...register("password", { required: { value: true, message: "Required." }, minLength: { value: 8, message: "Must be at least 8 characters long." }, maxLength: { value: 72, message: "Must be at most 72 characters long." } })}
          />
          {errors.password && <FormHelperText>{errors.password.message}</FormHelperText>}
        </FormControl>
        <Button loading={isLoading} type="submit" variant="contained" sx={{ mt: 2 }}>Sign Up</Button>
      </Stack>
    </Stack>
  </Stack>
}