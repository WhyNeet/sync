import { createBrowserRouter } from "react-router";
import { Root } from "./routes/root";
import { Messages } from "./routes";
import { SignIn } from "./routes/signin";
import { SignUp } from "./routes/signup";

export default createBrowserRouter([
  {
    path: "/",
    Component: Root,
    children: [
      {
        path: "/",
        Component: Messages,
      },
      {
        path: "/sign-in",
        Component: SignIn
      },
      {
        path: "/sign-up",
        Component: SignUp
      }
    ]
  }
])