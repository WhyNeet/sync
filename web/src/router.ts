import { createBrowserRouter } from "react-router";
import { Root } from "./routes/root";
import { Messages } from "./routes";

export default createBrowserRouter([
  {
    path: "/",
    Component: Root,
    children: [
      {
        path: "/",
        Component: Messages
      }
    ]
  }
])