import { useState } from "react"
import { CodeInput, EmailOtpVerify } from "../component/EmailOtpVerify"
import { invoke } from "@tauri-apps/api";
import { useNavigate } from "react-router-dom";

type OtpSuccessResponse = {
    verified: boolean;
    cookie: string;
};

type OtpErrorResponse = {
    message: string;
    status_code: number;
}

export const OtpPage = () => {
    const navigate = useNavigate();

    const [isLoading, setIsLoading] = useState(false);
    const [error, setError] = useState<string | undefined>();

    const onCodeVerify = async ({ code }: CodeInput) => {
        setIsLoading(true);

        try {
            const response: OtpSuccessResponse = await invoke("email_otp", {
                code,
                auth: localStorage.getItem('authCookie')
            });

            if (response.verified) {
                localStorage.setItem('2faCookie', response.cookie);
                navigate('/overview');
            }
        } catch (err) {
            const { message } = err as OtpErrorResponse;
            setError(message)
        } finally {
            setIsLoading(false)
        }
    }

    return <EmailOtpVerify error={error} isLoading={isLoading} onSubmit={onCodeVerify} />;
}