import { PrimeReactProvider } from "primereact/api"
import { RouterProvider, createBrowserRouter } from "react-router-dom"

import { LoginPage } from "./pages/login";
import { OverviewPage } from "./pages/overview";
import { OtpPage } from "./pages/otp";
import { WelcomePage } from "./pages/welcome";
import { useEffect, useRef } from "react";

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
        path: "/overview/*",
        element: <OverviewPage />,
    },
    {
        path: "/otp",
        element: <OtpPage />,
    },
]);

export const App = () => {
    return (
        <PrimeReactProvider>
            <RouterProvider router={router} />
        </PrimeReactProvider>
    )
}