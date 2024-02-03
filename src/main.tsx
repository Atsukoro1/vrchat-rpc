import React from "react";
import ReactDOM from "react-dom/client";
import { PrimeReactProvider } from "primereact/api";
import { createBrowserRouter, RouterProvider } from "react-router-dom";

import "./styles.css";
import "primereact/resources/themes/lara-light-cyan/theme.css";
import { LoginPage } from "./pages/login";
import { OverviewPage } from "./pages/overview";
import { OtpPage } from "./pages/otp";
import { WelcomePage } from "./pages/welcome";

const router = createBrowserRouter([
	{
		path: "/",
		element: <WelcomePage />,
	},
	{
		path: "/login",
		element: <LoginPage />,
	},
	{
		path: "/overview",
		element: <OverviewPage />,
	},
	{
		path: "/otp",
		element: <OtpPage />,
	},
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<React.StrictMode>
		<PrimeReactProvider>
			<RouterProvider router={router} />
		</PrimeReactProvider>
	</React.StrictMode>,
);
