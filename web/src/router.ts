import { createBrowserRouter } from "react-router";
import { Root } from "./routes/root";
import { Messages } from "./routes";
import { SignIn } from "./routes/signin";

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
      }
    ]
  }
])